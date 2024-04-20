pub mod trace;
pub mod value;

pub use trace::Trace;
pub use value::Value;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Result {
    #[serde(flatten)]
    pub status: Status,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "status")]
pub enum Status {
    Success,
    Failure { trace: Vec<Trace> },
}
