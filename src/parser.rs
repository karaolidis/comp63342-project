use crate::{
    codegen::generate_counterexample,
    jbmc::{self, Output},
};

use std::error::Error;

pub fn parse_jdmc_output(
    output: Vec<Output>,
    class: &str,
    entrypoint: &str,
) -> Vec<Result<String, Box<dyn Error + Send + Sync>>> {
    output
        .into_iter()
        .filter_map(|o| match o {
            Output::Result { result } => Some(result),
            _ => None,
        })
        .flatten()
        .filter(|r| matches!(r.status, jbmc::result::Status::Failure { .. }))
        .map(|result| {
            let trace = match result.status {
                jbmc::result::Status::Failure { trace } => trace,
                jbmc::result::Status::Success => unreachable!(),
            };
            generate_counterexample(trace, class, entrypoint)
        })
        .collect()
}
