use crate::jbmc::result::{trace, Trace, Value};

use genco::{lang::java, quote};
use log::warn;
use regex::Regex;

#[allow(clippy::too_many_lines)]
pub fn generate_counterexample(traces: Vec<Trace>, class: &str, entrypoint: &str) -> String {
    let traces = traces.into_iter().filter(|t| !t.hidden).collect::<Vec<_>>();
    let mut body: java::Tokens = quote! {};

    let mut function_call_is_last = false;
    let mut function_call_params = Vec::<String>::new();

    for (index, trace) in traces.iter().cloned().enumerate() {
        let next_trace = traces.get(index + 1);

        match trace.variant {
            trace::Variant::Assignment(assignment) => {
                if !assignment.lhs.starts_with("arg") {
                    continue;
                }

                match assignment.variant {
                    trace::AssignmentVariant::Variable => match assignment.value {
                        Value::I32(i) => {
                            body.append(quote! {
                                int $(&assignment.lhs) = $i;$['\r']
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
                        Value::Pointer(pointer) => {
                            let type_regex = Regex::new(r"struct (?P<type>.+) \*").unwrap();
                            let captures = type_regex.captures(&pointer.type_).unwrap();
                            let type_ = captures.name("type").unwrap().as_str();

                            if type_.contains("$$") {
                                warn!("Handling classes with $ in their name is not implemented");
                                break;
                            }

                            let type_ = type_.replace('$', ".");

                            if pointer.data.is_some() {
                                warn!("Handling data is not implemented");
                                break;
                            }

                            body.append(quote! {
                                $type_ $(&assignment.lhs) = null;$['\r']
                            });
                        }
                        Value::String(_) => todo!(),
                        Value::Struct(_) => todo!(),
                        Value::Other => unimplemented!(),
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

                        if function_call_is_last {
                            break;
                        }

                        function_call_params.clear();
                    }
                }
            }
            trace::Variant::FunctionCall(function_call) => {
                if !function_call.function.identifier.starts_with("java::") {
                    continue;
                }

                let function_name_regex =
                    Regex::new(r"java::(?P<class>.+)\.(?P<function>.+):.*").unwrap();

                let captures = function_name_regex
                    .captures(&function_call.function.identifier)
                    .unwrap();

                let capture_class = captures.name("class").unwrap().as_str();
                let capture_function = captures.name("function").unwrap().as_str();

                body.append(quote! {
                    $['\n']$capture_class.$capture_function
                });

                if capture_class == class && capture_function == entrypoint {
                    function_call_is_last = true;
                }
            }
            _ => {}
        }
    }

    let tokens: java::Tokens = quote! {
        class $(format!("{class}CE")) {
            public static void main(String[] args) {
                $body
            }
        }
    };

    tokens.to_file_string().unwrap()
}
