use ...;

pub fn generate_counterexample(
    traces: Vec<Trace>,
    class: &str,
    entrypoint: &str,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    ...

    match trace.variant {
        trace::Variant::Assignment(assignment) => {
            ...

            match assignment.variant {
                trace::AssignmentVariant::Variable => match assignment.value {
                    ...
                    Value::I64(i) => {
                        body.append(quote! {
                            long $(&assignment.lhs) = $i;$['\r']
                        });
                    }
                    Value::Bool(b) => {
                        body.append(quote! {
                            boolean $(&assignment.lhs) = $(format!("{:?}", b));$['\r']
                        });
                    }
                    ...
                    Value::Pointer(pointer) => {
                        let type_regex = Regex::new(r"struct (?P<type>.+) \*").unwrap();
                        let captures = type_regex.captures(&pointer.type_).unwrap();
                        let type_ = captures.name("type").unwrap().as_str();

                        ...
                        
                        if pointer.data.is_some() {
                            return Err("Handling pointers with data is not implemented".into());
                        }

                        body.append(quote! {
                            $type_ $(&assignment.lhs) = null;$['\r']
                        });
                    }
                    ...
                },
                trace::AssignmentVariant::ActualParameter => {
                    function_call_params.push(assignment.lhs.clone());

                    ...

                    body.append(quote! {
                        ($(function_call_params.join(", ")));$['\n']
                    });

                    ...
                }
            }
        }
        trace::Variant::FunctionCall(function_call) => {
            let function_name_regex =
                Regex::new(r"java::(?P<class>.+)\.(?P<function>.+):.*").unwrap();

            let captures = function_name_regex.captures(&function_call.function.identifier);

            ...

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

    ...

    let tokens: java::Tokens = quote! {
        class $(format!("{class}CE")) {
            public static void main(String[] args) {
                $body
            }
        }
    };

    ...
}
