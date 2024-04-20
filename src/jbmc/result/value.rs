use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(tag = "name", content = "data")]
#[serde(rename_all = "lowercase")]
pub enum Value {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Integer(isize),
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Float(f64),
    #[serde(rename = "void *")]
    Pointer(String),
}
