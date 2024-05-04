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
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// Path to the Java file/folder to be analyzed.
    /// If it's a folder, the file must be named either Main.java or the same as the folder.
    file_path: String,

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
    timeout: u64,
}

#[allow(clippy::too_many_lines)]
fn main() {
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

    let mut file_path = canonicalize(Path::new(&args.file_path))
        .expect("Failed to canonicalize file path, ensure the file exists");

    if let Ok(p) = file_path.strip_prefix(r"\\?\") {
        file_path = p.to_path_buf();
    }

    if file_path.is_dir() {
        let folder_name = file_path.file_name().unwrap().to_str().unwrap();

        let main_file = file_path.join("Main.java");
        let folder_file = file_path.join(format!("{folder_name}.java"));

        if main_file.exists() {
            file_path = main_file;
        } else if folder_file.exists() {
            file_path = folder_file;
        } else {
            panic!("Failed to find Main.java or {folder_name}.java in the folder");
        }
    }

    let mut javac_path = canonicalize(Path::new(&args.javac_path)
        )
        .unwrap_or_else(|_| {
            find_executable_in_path(&args.javac_path).expect("Failed to find javac, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)")
        });

    if let Ok(p) = javac_path.strip_prefix(r"\\?\") {
        javac_path = p.to_path_buf();
    }

    let mut jbmc_path = canonicalize(Path::new(&args.jbmc_path)
        )
        .unwrap_or_else(|_| find_executable_in_path(&args.jbmc_path).expect("Failed to find jbmc, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)"));

    if let Ok(p) = jbmc_path.strip_prefix(r"\\?\") {
        jbmc_path = p.to_path_buf();
    }

    let timeout = Duration::from_secs(args.timeout);

    let mut javac_proc = Command::new(&javac_path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn javac --version");

    if javac_proc.wait_timeout(timeout).unwrap().is_some() {
        let output = javac_proc.wait_with_output().unwrap();

        let javac_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        info!("Java compiler version: {}", javac_version);

        if !javac_version.contains(" 17.") {
            warn!("JBMC may not work as expected with Java versions other than 17");
        }
    } else {
        javac_proc.kill().expect("Failed to kill Java compiler");
        panic!("Java compiler took too long to respond")
    }

    let mut jbmc_proc = Command::new(&jbmc_path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run jbmc --version");

    if jbmc_proc.wait_timeout(timeout).unwrap().is_some() {
        let output = jbmc_proc.wait_with_output().unwrap();

        let jbmc_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        info!("JBMC version: {}", jbmc_version);
    } else {
        jbmc_proc.kill().expect("Failed to kill JBMC");
        panic!("JBMC took too long to respond")
    }

    compile_java_class(&file_path, &javac_path, timeout);
    let class = file_path.file_stem().unwrap().to_str().unwrap();

    let output = run_jbmc(&file_path, &args.entrypoint, &jbmc_path, timeout);
    let counterexamples = parse_jdmc_output(output, class, &args.entrypoint);

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
}
