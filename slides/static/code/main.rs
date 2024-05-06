use ...;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct Args {
    ...
}

#[derive(Debug, Default)]
struct Stats {
    ...
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let javac_path = canonicalize(Path::new(&args.javac_path))?;
    let jbmc_path = canonicalize(Path::new(&args.jbmc_path))?;

    ...

    let (file_paths, entrypoints): (Vec<PathBuf>, Vec<String>) = args
        .file_paths
        .iter()
        .filter_map(|fpe| {
            ...
        })
        .unzip();

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
            stats.clone(),
        ));

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    progress.clear().unwrap();
}

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

    let output = compile_java_class(&file_path, &javac_path, timeout).await;
    if let Err(e) = output {
        return;
    }

    stats.lock().await.compiled += 1;
    let class = file_path.file_stem().unwrap().to_str().unwrap();

    let output = run_jbmc(&file_path, &entrypoint, &jbmc_path, timeout).await;
    if let Err(e) = output {
        return;
    }

    stats.lock().await.analyzed += 1;
    let output = output.unwrap();

    let counterexamples = generate_counterexamples(output, class, &entrypoint);
    if counterexamples.is_empty() {
        return;
    }

    stats.lock().await.counterexamples += counterexamples.len();

    for (i, counterexample) in counterexamples.into_iter().enumerate() {
        match counterexample {
            Ok((counterexample, reason)) => {
                let file = file_path.with_file_name(format!("{class}-CE-{i}-{reason}.java"));
                let result = fs::write(&file, counterexample);

                match result {
                    ...
                }
            }
            Err(e) => {
                ...
            }
        }
    }
}
