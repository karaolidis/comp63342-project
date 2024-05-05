#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_docs_in_private_items)]

mod codegen;
mod jbmc;
mod runner;

use codegen::generate_counterexamples;
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use runner::{compile_java_class, run_jbmc};

use clap::Parser;
use dunce::canonicalize;
use pathsearch::find_executable_in_path;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::Duration,
};
use tokio::{
    spawn,
    sync::{Mutex, OwnedSemaphorePermit, Semaphore},
    task::JoinHandle,
};

const TICK_INTERVAL: Duration = Duration::from_millis(100);
const LOGO: &str = r"
         #         #
          #       #
           #######
         ##       ##           .-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-.
         ##       ##           |  _________                      __                _________ __         .__ __             |
           #######             !  \_   ___ \  ____  __ __  _____/  |_  ___________/   _____//  |________|__|  | __ ____    !
         ###########           :  /    \  \/ /  _ \|  |  \/    \   __\/ __ \_  __ \_____  \\   __\_  __ \  |  |/ // __ \   :
      ###   #####   ###        :  \     \___(  <_> )  |  /   |  \  | \  ___/|  | \/        \|  |  |  | \/  |    <\  ___/   :
    ## #      #      # ##      !   \______  /\____/|____/|___|  /__|  \___  >__| /_______  /|__|  |__|  |__|__|_ \\___  >  !
  #    # ########### #    #    |          \/                  \/          \/             \/                     \/    \/   |
  ######             ######    .-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-.
        ###       ###
           #######
";

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

#[derive(Debug, Default)]
struct Stats {
    paths: usize,
    compiled: usize,
    analyzed: usize,
    counterexamples: usize,
    successful_counterexamples: usize,
    vulnerabilities: HashMap<String, usize>,
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
    let progress = MultiProgress::new();
    let args = Args::parse();

    let (height, width) = console::Term::stdout().size();
    if width >= 128 && height >= 16 {
        println!("{LOGO}");
    }

    let timeout = Duration::from_secs(args.timeout.try_into().unwrap());

    println!(
        "{} {} Resolving dependencies...",
        style("[1/4]").bold().dim(),
        Emoji("🔍", "I")
    );

    let javac_path = canonicalize(Path::new(&args.javac_path))
        .unwrap_or_else(|_| find_executable_in_path(&args.javac_path).expect("Failed to find javac, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)"));

    let jbmc_path = canonicalize(Path::new(&args.jbmc_path))
        .unwrap_or_else(|_| find_executable_in_path(&args.jbmc_path).expect("Failed to find jbmc, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)"));

    let javac_output = Command::new(&javac_path)
        .arg("--version")
        .output()
        .expect("Failed to run javac --version, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)");

    let javac_version = String::from_utf8_lossy(&javac_output.stdout)
        .trim()
        .to_string();

    println!(
        "      {} Java compiler version: {}",
        Emoji("🔍", "I"),
        javac_version
    );

    if !javac_version.contains(" 17.") {
        println!(
            "      {} JBMC may not work as expected with Java versions other than 17",
            Emoji("⚠", "W")
        );
    }

    let jbmc_output = Command::new(&jbmc_path)
        .arg("--version")
        .output()
        .expect("Failed to run jbmc --version, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)");

    let jbmc_version = String::from_utf8_lossy(&jbmc_output.stdout)
        .trim()
        .to_string();

    println!(
        "      {} JBMC version: {}",
        Emoji("🔍", "[I] "),
        jbmc_version
    );

    println!(
        "{} {} Analyzing Java files...",
        style("[2/4]").bold().dim(),
        Emoji("🧪", "I"),
    );

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

            let canonical_file_path = canonicalize(&file_path);
            let short_file_path = file_path.file_name().unwrap();

            if let Err(e) = canonical_file_path {
                println!(
                    "      {} File {:?}: {e}",
                    Emoji("💥", "E"),
                    short_file_path
                );
                return None;
            }

            let mut file_path = canonical_file_path.unwrap();

            if file_path.is_dir() {
                let folder_name = short_file_path.to_str().unwrap();

                let main_file = file_path.join("Main.java");
                let folder_file = file_path.join(format!("{folder_name}.java"));

                if main_file.exists() {
                    file_path = canonicalize(&main_file).unwrap();
                } else if folder_file.exists() {
                    file_path = canonicalize(&folder_file).unwrap();
                } else {
                    println!(
                        r#"      {} Folder {folder_name:?}: No "Main.java" or "{folder_name}.java" found"#,
                        Emoji("💥", "E"),
                    );
                    return None;
                }
            }

            println!(
                "      {} File: {:?}, Function: {entrypoint:?}",
                Emoji("🧪", "I"),
                short_file_path,
            );

            Some((file_path, entrypoint))
        })
        .unzip();

    println!(
        "{} {} Executing...",
        style("[3/4]").bold().dim(),
        Emoji("🚀", "I"),
    );

    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    let semaphore = Arc::new(Semaphore::new(args.threads));
    let stats = Arc::new(Mutex::new(Stats {
        paths: file_paths.len(),
        ..Default::default()
    }));

    for (file_path, entrypoint) in file_paths.into_iter().zip(entrypoints) {
        let task = spawn(execute(
            file_path,
            entrypoint,
            javac_path.clone(),
            jbmc_path.clone(),
            timeout,
            semaphore.clone().acquire_owned().await.unwrap(),
            progress.clone(),
            stats.clone(),
        ));

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    progress.clear().unwrap();

    println!("{} {} Done!", style("[4/4]").bold().dim(), Emoji("🎉", "I"));

    let stats = stats.lock().await;

    println!("      {} Total paths: {}", Emoji("📊", "I"), stats.paths);

    println!(
        "      {} Total compiled: {}",
        Emoji("📊", "I"),
        stats.compiled
    );

    println!(
        "      {} Total analyzed: {}",
        Emoji("📊", "I"),
        stats.analyzed
    );

    println!(
        "      {} Total counterexamples: {}",
        Emoji("📊", "I"),
        stats.counterexamples
    );

    println!(
        "      {} Total successful counterexamples: {}",
        Emoji("📊", "I"),
        stats.successful_counterexamples
    );

    println!(
        "      {} Vulnerabilities Generated:",
        Emoji("📊", "[I]")
    );

    for (vulnerability, count) in &stats.vulnerabilities {
        println!("          - {vulnerability}: {count}");
    }
}

#[allow(clippy::too_many_arguments)]
async fn execute(
    file_path: PathBuf,
    entrypoint: String,
    javac_path: PathBuf,
    jbmc_path: PathBuf,
    timeout: Duration,
    permit: OwnedSemaphorePermit,
    progress: MultiProgress,
    stats: Arc<Mutex<Stats>>,
) {
    let _permit = permit;
    let short_file_path = file_path.file_name().unwrap().to_str().unwrap();

    let task_progress = ProgressBar::new_spinner();
    task_progress.enable_steady_tick(TICK_INTERVAL);
    task_progress.set_style(
        ProgressStyle::with_template("      {spinner}  {msg} [{elapsed}]")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );
    progress.add(task_progress.clone());

    task_progress.set_message(format!("{short_file_path:?} - Compiling..."));

    let output = compile_java_class(&file_path, &javac_path, timeout).await;

    if let Err(e) = output {
        task_progress.println(format!(
            "      {} {short_file_path:?}: Failed to compile: {e}",
            Emoji("💥", "E"),
        ));
        task_progress.finish_and_clear();
        return;
    }

    stats.lock().await.compiled += 1;
    task_progress.set_message(format!("{short_file_path:?} - Analyzing..."));
    let class = file_path.file_stem().unwrap().to_str().unwrap();

    let output = run_jbmc(&file_path, &entrypoint, &jbmc_path, timeout).await;

    if let Err(e) = output {
        task_progress.println(format!(
            "      {} {short_file_path:?}: Failed to analyze: {e}",
            Emoji("💥", "E"),
        ));
        task_progress.finish_and_clear();
        return;
    }

    stats.lock().await.analyzed += 1;
    let output = output.unwrap();
    task_progress.set_message(format!(
        "{short_file_path:?} - Generating counterexamples..."
    ));

    let counterexamples = generate_counterexamples(output, class, &entrypoint);

    if counterexamples.is_empty() {
        task_progress.println(format!(
            "      {} {short_file_path:?}: No counterexamples found",
            Emoji("✅", "I"),
        ));
        task_progress.finish_and_clear();
        return;
    }

    stats.lock().await.counterexamples += counterexamples.len();
    task_progress.set_message(format!("{short_file_path:?} - Writing counterexamples..."));

    for (i, counterexample) in counterexamples.into_iter().enumerate() {
        match counterexample {
            Ok((counterexample, reason)) => {
                let file = file_path.with_file_name(format!("{class}-CE-{i}-{reason}.java"));
                let result = fs::write(&file, counterexample);

                match result {
                    Ok(()) => {
                        let mut stats = stats.lock().await;
                        stats.successful_counterexamples += 1;
                        *stats.vulnerabilities.entry(reason).or_insert(0) += 1;
                        drop(stats);

                        task_progress.println(format!(
                            "      {} {short_file_path:?}: Counterexample written to file {:?}",
                            Emoji("🎯", "I"),
                            file.file_name().unwrap(),
                        ));
                    }
                    Err(e) => {
                        task_progress.println(format!(
                            "      {} {short_file_path:?}: Failed to write counterexample to file {:?}: {e}",
                            Emoji("💥", "E"),
                            file.file_name().unwrap(),
                        ));
                    }
                }
            }
            Err(e) => {
                task_progress.println(format!(
                    "      {} {short_file_path:?}: Failed to generate counterexample: {e}",
                    Emoji("💥", "E"),
                ));
            }
        }
    }

    task_progress.finish_and_clear();
}
