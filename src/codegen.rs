use crate::jbmc::result::Value;

use genco::{lang::java, quote};
use indexmap::IndexMap;

pub fn inputs_to_java_variables(inputs: IndexMap<String, Value>) -> java::Tokens {
    quote! {
        $(for (name, value) in inputs {
            $(match value {
                Value::Integer(i) => (int $name = $i;),
                Value::Float(f) => (double $name = $(format!("{f:?}"));)
                Value::Pointer(_) => todo!(),
            })$['\r']
        })
    }
}

pub fn generate_counterexample(
    class: &str,
    entrypoint: &str,
    counterexample_class: &str,
    inputs: IndexMap<String, Value>,
) -> String {
    let arguments = inputs
        .keys()
        .map(std::string::String::as_str)
        .collect::<Vec<&str>>()
        .join(", ");

    let tokens: java::Tokens = quote! {
        class $counterexample_class {
            public static void main(String[] args) {
                $(inputs_to_java_variables(inputs))

                $class.$entrypoint($arguments);
            }
        }
    };

    tokens.to_file_string().unwrap()
}
