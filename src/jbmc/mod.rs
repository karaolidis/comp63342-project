pub mod c_prover_status;
pub mod message;
pub mod result;

pub use c_prover_status::CProverStatus;
pub use message::Message;
pub use result::Result;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Output {
    Program { program: String },
    Message(Message),
    Result { result: Vec<Result> },
    CProverStatus(CProverStatus),
}
