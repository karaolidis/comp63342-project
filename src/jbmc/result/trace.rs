use super::Value;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Trace {
    #[serde(flatten)]
    pub variant: Variant,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "stepType")]
#[serde(rename_all = "kebab-case")]
pub enum Variant {
    Input(Input),
    Failure(Failure),
    #[serde(other)]
    Other,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Input {
    #[serde(rename = "inputID")]
    pub id: String,
    pub values: Vec<Value>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Failure {
    pub reason: String,
}
