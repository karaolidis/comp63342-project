use crate::jbmc::{self, Output};

use std::{
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use wait_timeout::ChildExt;

pub fn compile_java_class(file: &Path, javac_path: &Path, timeout: Duration) {
    let mut proc = Command::new(javac_path)
        .arg(file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn Java compiler");

    if let Some(status) = proc.wait_timeout(timeout).unwrap() {
        let output = proc.wait_with_output().unwrap();

        assert!(
            status.success(),
            "Failed to compile Java file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        proc.kill().expect("Failed to kill Java compiler");
        panic!("Java compiler timed out");
    }
}

pub fn run_jbmc(file: &Path, entrypoint: &str, jbmc_path: &Path, timeout: Duration) -> Vec<Output> {
    let stem = file.file_stem().unwrap().to_str().unwrap();

    let mut proc = Command::new(jbmc_path)
        .arg("--json-ui")
        .arg(format!("{stem}.{entrypoint}"))
        .current_dir(file.parent().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn JBMC");

    if proc.wait_timeout(timeout).unwrap().is_some() {
        let output = proc.wait_with_output().unwrap();
        let output = serde_json::from_slice::<Vec<Output>>(&output.stdout).unwrap();

        if let Some(e) = output.iter().find_map(|o| match o {
            Output::Message(jbmc::Message::Error(e)) => Some(e.trim()),
            _ => None,
        }) {
            panic!("JBMC error: {e}");
        }

        output
    } else {
        proc.kill().expect("Failed to kill JBMC");
        panic!("JBMC timed out");
    }
}
