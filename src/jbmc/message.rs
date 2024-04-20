use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "messageType", content = "messageText")]
#[allow(dead_code)]
pub enum Message {
    #[serde(rename = "STATUS-MESSAGE")]
    Info(String),
    #[serde(rename = "WARNING")]
    Warning(String),
    #[serde(rename = "ERROR")]
    Error(String),
}
