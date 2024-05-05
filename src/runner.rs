use crate::jbmc::{self, Output};

use std::{error::Error, path::Path, process::Stdio, time::Duration};
use tokio::{process::Command, time::timeout};

pub async fn compile_java_class(
    file: &Path,
    javac_path: &Path,
    duration: Duration,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let proc = Command::new(javac_path)
        .arg(file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn Java compiler");

    match timeout(duration, proc.wait_with_output()).await {
        Ok(Ok(output)) if output.status.success() => Ok(()),
        Ok(Ok(output)) => {
            let e = String::from_utf8_lossy(&output.stderr)
                .lines()
                .next()
                .unwrap()
                .to_string();

            if let Some(error) = e.find("error: ") {
                return Err(e[error..].into());
            }

            Err(e.into())
        }
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err("Java compiler timed out".into()),
    }
}

pub async fn run_jbmc(
    file: &Path,
    entrypoint: &str,
    jbmc_path: &Path,
    duration: Duration,
) -> Result<Vec<Output>, Box<dyn Error + Send + Sync>> {
    let stem = file.file_stem().unwrap().to_str().unwrap();

    let proc = Command::new(jbmc_path)
        .arg("--unwind")
        .arg("25")
        .arg("--json-ui")
        .arg(format!("{stem}.{entrypoint}"))
        .current_dir(file.parent().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn JBMC");

    match timeout(duration, proc.wait_with_output()).await {
        Ok(Ok(output)) => {
            let output = serde_json::from_slice::<Vec<Output>>(&output.stdout)?;

            if let Some(e) = output.iter().find_map(|o| match o {
                Output::Message(jbmc::Message::Error(e)) => Some(e.trim()),
                _ => None,
            }) {
                return Err(e.into());
            }

            Ok(output)
        }
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err("JBMC timed out".into()),
    }
}
