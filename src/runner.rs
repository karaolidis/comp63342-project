use crate::jbmc::{self, Output};

use std::{path::Path, process::Command};

pub fn compile_java_class(file: &Path, javac_path: &Path) {
    let output = Command::new(javac_path)
        .arg("-g")
        .arg(file)
        .output()
        .expect("Failed to compile Java file");

    assert!(
        output.status.success(),
        "Failed to compile Java file: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn run_jbmc(file: &Path, entrypoint: &str, jbmc_path: &Path) -> Vec<Output> {
    let stem = file.file_stem().unwrap().to_str().unwrap();

    let output = Command::new(jbmc_path)
        .arg("--json-ui")
        .arg(format!("{stem}.{entrypoint}"))
        .current_dir(file.parent().unwrap())
        .output()
        .expect("Failed to run JBMC");

    let output = serde_json::from_slice::<Vec<Output>>(&output.stdout).unwrap();

    if let Some(e) = output.iter().find_map(|o| match o {
        Output::Message(jbmc::Message::Error(e)) => Some(e.trim()),
        _ => None,
    }) {
        panic!("JBMC error: {e}");
    }

    output
}
