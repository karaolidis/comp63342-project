use crate::jbmc::{
    self,
    result::{trace, Trace, Value},
    Output,
};

use genco::{lang::java, quote};
use regex::Regex;
use std::error::Error;

pub fn generate_counterexamples(
    output: Vec<Output>,
    class: &str,
    entrypoint: &str,
) -> Vec<Result<(String, String), Box<dyn Error + Send + Sync>>> {
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

#[allow(clippy::too_many_lines)]
pub fn generate_counterexample(
    traces: Vec<Trace>,
    class: &str,
    entrypoint: &str,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let traces = traces.into_iter().filter(|t| !t.hidden).collect::<Vec<_>>();
    let mut body: java::Tokens = quote! {};

    let mut done = false;
    let mut function_call_name = None;
    let mut function_call_params = vec![];
    let mut reason = None;

    for (index, trace) in traces.iter().cloned().enumerate() {
        let next_trace = traces.get(index + 1);

        match trace.variant {
            trace::Variant::Assignment(assignment) => {
                if !assignment.lhs.starts_with("arg") || done {
                    continue;
                }

                match assignment.variant {
                    trace::AssignmentVariant::Variable => match assignment.value {
                        Value::I8(i) => {
                            body.append(quote! {
                                byte $(&assignment.lhs) = $i;$['\r']
                            });
                        }
                        Value::I16(i) => {
                            body.append(quote! {
                                short $(&assignment.lhs) = $i;$['\r']
                            });
                        }
                        Value::I32(i) => {
                            body.append(quote! {
                                int $(&assignment.lhs) = $i;$['\r']
                            });
                        }
                        Value::I64(i) => {
                            body.append(quote! {
                                long $(&assignment.lhs) = $i;$['\r']
                            });
                        }
                        Value::F32(f) => {
                            body.append(quote! {
                                float $(&assignment.lhs) = $(format!("{:?}", f));$['\r']
                            });
                        }
                        Value::F64(f) => {
                            body.append(quote! {
                                double $(&assignment.lhs) = $(format!("{:?}", f));$['\r']
                            });
                        }
                        Value::Bool(b) => {
                            body.append(quote! {
                                boolean $(&assignment.lhs) = $(format!("{:?}", b));$['\r']
                            });
                        }
                        Value::Char(c) => {
                            body.append(quote! {
                                char $(&assignment.lhs) = $(format!("{:?}", c));$['\r']
                            });
                        }
                        Value::Pointer(pointer) => {
                            let type_regex = Regex::new(r"struct (?P<type>.+) \*").unwrap();
                            let captures = type_regex.captures(&pointer.type_).unwrap();
                            let type_ = captures.name("type").unwrap().as_str();

                            if type_.contains("$$") {
                                return Err(
                                    "Handling classes with $ in their name is not implemented"
                                        .into(),
                                );
                            }

                            let type_ = type_.replace('$', ".");

                            if pointer.data.is_some() {
                                return Err("Handling pointers with data is not implemented".into());
                            }

                            body.append(quote! {
                                $type_ $(&assignment.lhs) = null;$['\r']
                            });
                        }
                        Value::String(_) => {
                            return Err("Handling strings is not implemented".into());
                        }
                        Value::Struct(_) => {
                            return Err("Handling structs is not implemented".into());
                        }
                        Value::Other => {
                            return Err("Handling other values is not implemented".into());
                        }
                    },
                    trace::AssignmentVariant::ActualParameter => {
                        function_call_params.push(assignment.lhs.clone());

                        if let Some(next_trace) = next_trace {
                            if let trace::Variant::Assignment(next_assignment) = &next_trace.variant
                            {
                                if next_assignment.variant
                                    == trace::AssignmentVariant::ActualParameter
                                {
                                    continue;
                                }
                            }
                        }

                        body.append(quote! {
                           ($(function_call_params.join(", ")));$['\n']
                        });

                        function_call_params.clear();

                        if let Some(function_call_name) = function_call_name.take() {
                            if function_call_name == entrypoint {
                                done = true;
                            }
                        }
                    }
                }
            }
            trace::Variant::FunctionCall(function_call) => {
                if !function_call.function.identifier.starts_with("java::") || done {
                    continue;
                }

                let function_name_regex =
                    Regex::new(r"java::(?P<class>.+)\.(?P<function>.+):.*").unwrap();

                let captures = function_name_regex.captures(&function_call.function.identifier);

                if captures.is_none() {
                    continue;
                }

                let captures = captures.unwrap();

                let capture_class = captures.name("class").unwrap().as_str();
                let capture_function = captures.name("function").unwrap().as_str();

                body.append(quote! {
                    $['\n']$capture_class.$capture_function
                });

                function_call_name = Some(capture_function.to_string());
            }
            trace::Variant::Failure(failure) => {
                reason = Some(
                    failure
                        .property
                        .as_str()
                        .split('.')
                        .nth_back(1)
                        .unwrap()
                        .to_string(),
                );
            }
            trace::Variant::Other => {}
        }
    }

    let tokens: java::Tokens = quote! {
        class $(format!("{class}CE")) {
            public static void main(String[] args) {
                $body
            }
        }
    };

    Ok((
        tokens.to_file_string().unwrap(),
        reason.unwrap_or_else(|| String::from("unknown")),
    ))
}
