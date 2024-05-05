use crate::jbmc::{self, Output};

use log::{error, info};
use std::{error::Error, path::Path, process::Stdio, time::Duration};
use tokio::{process::Command, time::timeout};

pub async fn compile_java_class(file: &Path, javac_path: &Path, duration: Duration) {
    let mut proc = Command::new(javac_path)
        .arg(file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to spawn Java compiler");

    match timeout(duration, proc.wait()).await {
        Ok(Ok(status)) if status.success() => info!("Finished compiling {file:?}"),
        Ok(Ok(_)) => unreachable!(),
        Ok(Err(e)) => error!("Java compiler error: {e}"),
        Err(_) => error!("Java compiler timed out"),
    }
}

pub async fn run_jbmc(
    file: &Path,
    entrypoint: &str,
    jbmc_path: &Path,
    duration: Duration,
) -> Result<Vec<Output>, Box<dyn Error>> {
    let stem = file.file_stem().unwrap().to_str().unwrap();

    let proc = Command::new(jbmc_path)
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
        Ok(Err(e)) => {
            error!("JBMC error: {e}");
            Err(e.into())
        }
        Err(_) => {
            error!("JBMC timed out");
            Err("JBMC timed out".into())
        }
    }
}
