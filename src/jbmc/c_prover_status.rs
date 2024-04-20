use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "cProverStatus")]
pub enum CProverStatus {
    Success,
    Failure,
}
