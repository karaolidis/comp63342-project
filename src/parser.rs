use crate::jbmc::{self, result::Value, Output};

use indexmap::IndexMap;
use log::info;

pub fn parse_jdmc_output(output: Vec<Output>) -> Vec<IndexMap<String, Value>> {
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
        .map(|failure| {
            let mut reason = None;
            let mut inputs: IndexMap<String, Value> = IndexMap::new();

            match &failure.status {
                jbmc::result::Status::Failure { trace } => {
                    for t in trace {
                        match &t.variant {
                            jbmc::result::trace::Variant::Input(i) => {
                                inputs.insert(i.id.clone(), i.values.first().unwrap().clone());
                            }
                            jbmc::result::trace::Variant::Failure(f) => {
                                reason = Some(f.reason.clone());
                            }
                            jbmc::result::trace::Variant::Other => (),
                        }
                    }
                }
                jbmc::result::Status::Success => unreachable!(),
            };

            info!(
                "Reason: {}, Inputs: {:?}",
                reason.unwrap_or_else(|| "Unknown".to_string()),
                inputs
            );

            inputs
        })
        .collect::<Vec<_>>()
}
