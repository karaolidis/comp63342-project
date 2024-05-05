#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_docs_in_private_items)]

mod codegen;
mod jbmc;
mod parser;
mod runner;

use parser::parse_jdmc_output;
use runner::{compile_java_class, run_jbmc};

use clap::Parser;
use dunce::canonicalize;
use log::{error, info, warn};
use pathsearch::find_executable_in_path;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::Duration,
};
use tokio::{spawn, sync::Semaphore, task::JoinHandle};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// Path(s) to the Java file(s)/folder(s) to be analyzed.
    /// If a path is a folder, the file must be named either Main.java or the same as the folder.
    /// Entrypoint can be specified with the format path:entrypoint.
    /// If no entrypoint is provided, the default entrypoint is used.
    file_paths: Vec<String>,

    /// Entrypoint function to be analyzed
    #[arg(short, long, default_value = "test")]
    entrypoint: String,

    /// Path to the Java compiler executable
    #[arg(short = 'c', long = "javac", default_value = "javac")]
    javac_path: String,

    /// Path to the JBMC executable
    #[arg(short, long = "jbmc", default_value = "jbmc")]
    jbmc_path: String,

    /// Execution timeout in seconds
    #[arg(short, long, default_value = "60")]
    timeout: usize,

    /// Maximum number of threads to use
    #[arg(short = 'n', long, default_value = "1")]
    threads: usize,
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
    log4rs::init_config(
        log4rs::Config::builder()
            .appender(
                log4rs::config::Appender::builder().build(
                    "stdout",
                    Box::new(
                        log4rs::append::console::ConsoleAppender::builder()
                            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                                "{d} {h({l})} {M}::{L} - {m}{n}",
                            )))
                            .build(),
                    ),
                ),
            )
            .build(
                log4rs::config::Root::builder()
                    .appender("stdout")
                    .build(log::LevelFilter::Info),
            )
            .unwrap(),
    )
    .unwrap();

    let args = Args::parse();

    let javac_path = canonicalize(Path::new(&args.javac_path)
        )
        .unwrap_or_else(|_| {
            find_executable_in_path(&args.javac_path).expect("Failed to find javac, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)")
        });

    let jbmc_path = canonicalize(Path::new(&args.jbmc_path)
        )
        .unwrap_or_else(|_| find_executable_in_path(&args.jbmc_path).expect("Failed to find jbmc, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)"));

    let timeout = Duration::from_secs(args.timeout.try_into().unwrap());

    let javac_output = Command::new(&javac_path)
        .arg("--version")
        .output()
        .expect("Failed to run javac --version, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)");

    let javac_version = String::from_utf8_lossy(&javac_output.stdout)
        .trim()
        .to_string();

    info!("Java compiler version: {}", javac_version);

    if !javac_version.contains(" 17.") {
        warn!("JBMC may not work as expected with Java versions other than 17");
    }

    let jbmc_output = Command::new(&jbmc_path)
        .arg("--version")
        .output()
        .expect("Failed to run jbmc --version, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)");

    let jbmc_version = String::from_utf8_lossy(&jbmc_output.stdout)
        .trim()
        .to_string();

    info!("JBMC version: {}", jbmc_version);

    let (file_paths, entrypoints): (Vec<PathBuf>, Vec<String>) = args
        .file_paths
        .iter()
        .filter_map(|fpe| {
            let (file_path, entrypoint) = if let Some(colon) = fpe.rfind(':') {
                let (file_path, entrypoint) = fpe.split_at(colon);

                let file_path = Path::new(file_path).to_path_buf();
                let entrypoint = &entrypoint[1..];

                if file_path.exists() {
                    (file_path, entrypoint.to_string())
                } else {
                    (Path::new(fpe).to_path_buf(), args.entrypoint.clone())
                }
            } else {
                (Path::new(fpe).to_path_buf(), args.entrypoint.clone())
            };

            let canonical = canonicalize(&file_path);

            if let Err(e) = canonical {
                error!("Failed to canonicalize file path {file_path:?}: {e}",);
                return None;
            }

            let mut file_path = canonical.unwrap();

            if file_path.is_dir() {
                let folder_name = file_path.file_name().unwrap().to_str().unwrap();

                let main_file = file_path.join("Main.java");
                let folder_file = file_path.join(format!("{folder_name}.java"));

                if main_file.exists() {
                    file_path = canonicalize(&main_file).unwrap();
                } else if folder_file.exists() {
                    file_path = canonicalize(&folder_file).unwrap();
                } else {
                    error!("No Main.java or {folder_name}.java found in {file_path:?}");
                    return None;
                }
            }

            Some((file_path, entrypoint))
        })
        .unzip();

    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    let semaphore = Arc::new(Semaphore::new(args.threads));

    for (file_path, entrypoint) in file_paths.into_iter().zip(entrypoints) {
        let javac_path = javac_path.clone();
        let jbmc_path = jbmc_path.clone();

        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let task = spawn(async move {
            let _permit = permit;

            compile_java_class(&file_path, &javac_path, timeout).await;
            let class = file_path.file_stem().unwrap().to_str().unwrap();

            let output = run_jbmc(&file_path, &entrypoint, &jbmc_path, timeout).await;

            if let Err(e) = output {
                error!("JBMC error: {}", e);
                return;
            }

            let counterexamples = parse_jdmc_output(output.unwrap(), class, &entrypoint);

            for (i, counterexample) in counterexamples.into_iter().enumerate() {
                match counterexample {
                    Ok(counterexample) => {
                        let file = file_path.with_file_name(format!("{class}CE_{i}.java"));
                        let result = fs::write(&file, counterexample);

                        if let Err(e) = result {
                            error!("Failed to write counterexample to file: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to generate counterexample: {}", e);
                    }
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
