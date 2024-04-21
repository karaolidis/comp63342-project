use crate::{
    codegen::generate_counterexample,
    jbmc::{self, Output},
};

use log::{error, info};

pub fn parse_jdmc_output(output: Vec<Output>, class: &str, entrypoint: &str) -> Vec<String> {
    if !output
        .iter()
        .any(|o| matches!(o, Output::CProverStatus(jbmc::CProverStatus::Failure)))
    {
        info!("No assertion failures found");
    }

    let failures: Vec<_> = output
        .into_iter()
        .filter_map(|o| match o {
            Output::Result { result } => Some(result),
            _ => None,
        })
        .flatten()
        .filter(|r| matches!(r.status, jbmc::result::Status::Failure { .. }))
        .collect();

    info!("Found {} assertion failure(s)", failures.len());

    failures
        .into_iter()
        .filter_map(|result| {
            let trace = match result.status {
                jbmc::result::Status::Failure { trace } => trace,
                jbmc::result::Status::Success => unreachable!(),
            };

            match generate_counterexample(trace, class, entrypoint) {
                Ok(counterexample) => Some(counterexample),
                Err(e) => {
                    error!("Failed to generate counterexample: {}", e);
                    None
                }
            }
        })
        .collect()
}
