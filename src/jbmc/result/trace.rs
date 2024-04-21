use super::Value;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Trace {
    pub hidden: bool,
    #[serde(flatten)]
    pub variant: Variant,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(tag = "stepType")]
#[serde(rename_all = "kebab-case")]
pub enum Variant {
    Assignment(Assignment),
    // Input(Input), Not sure what this is
    FunctionCall(FunctionCall),
    Failure(Failure),
    #[serde(other)]
    Other,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Assignment {
    pub mode: Mode,
    #[serde(rename = "assignmentType")]
    pub variant: AssignmentVariant,
    pub lhs: String,
    pub value: Value,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum AssignmentVariant {
    Variable,
    ActualParameter,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Input {
    pub mode: Mode,
    #[serde(rename = "inputID")]
    pub id: String,
    pub values: Vec<Value>,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub enum Mode {
    C,
    #[serde(rename = "java")]
    Java,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FunctionCall {
    pub function: Function,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Function {
    pub identifier: String,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Failure {
    pub reason: String,
}
