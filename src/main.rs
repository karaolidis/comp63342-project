#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_docs_in_private_items)]

mod codegen;
mod jbmc;
mod parser;
mod runner;

use codegen::generate_counterexample;
use parser::parse_jdmc_output;
use runner::{compile_java_class, run_jbmc};

use clap::Parser;
use log::{info, warn};
use pathsearch::find_executable_in_path;
use std::{fs, path::Path, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
#[allow(clippy::struct_field_names)]
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
}

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

    let mut file_path = Path::new(&args.file_path)
        .canonicalize()
        .expect("Failed to canonicalize file path, ensure the file exists");

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

    let javac_path = Path::new(&args.javac_path)
        .canonicalize()
        .unwrap_or_else(|_| {
            find_executable_in_path(&args.javac_path).expect("Failed to find javac, ensure that the Java compiler is installed and accessible via the path in -c (--javac-path)")
        });

    let jbmc_path = Path::new(&args.jbmc_path)
        .canonicalize()
        .unwrap_or_else(|_| find_executable_in_path(&args.jbmc_path).expect("Failed to find jbmc, ensure that JBMC is installed and accessible via the path in -j (--jbmc-path)"));

    let javac_version = Command::new(&javac_path)
        .arg("--version")
        .output()
        .expect("Failed to run javac --version");

    let javac_version = String::from_utf8_lossy(&javac_version.stdout)
        .trim()
        .to_string();

    info!("Java compiler version: {}", javac_version);

    if !javac_version.contains(" 17.") {
        warn!("JBMC may not work as expected with Java versions other than 17");
    }

    let jbmc_version = Command::new(&jbmc_path)
        .arg("--version")
        .output()
        .expect("Failed to run jbmc --version");

    let jbmc_version = String::from_utf8_lossy(&jbmc_version.stdout)
        .trim()
        .to_string();

    info!("JBMC version: {}", jbmc_version);

    compile_java_class(&file_path, &javac_path);

    let output = run_jbmc(&file_path, &args.entrypoint, &jbmc_path);

    let inputs = parse_jdmc_output(output);
    let class = file_path.file_stem().unwrap().to_str().unwrap();

    for (i, input) in inputs.into_iter().enumerate() {
        let counterexample_class = format!("{class}CE{i}");
        let counterexample =
            generate_counterexample(class, &args.entrypoint, &counterexample_class, input);

        let counterexample_file = file_path.with_file_name(format!("{counterexample_class}.java"));
        fs::write(&counterexample_file, counterexample)
            .expect("Failed to write counterexample file");
    }
}
