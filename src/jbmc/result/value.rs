use serde::{
    de::{Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    #[serde(deserialize_with = "deserialize_value_to_i32")]
    I32(i32),
    #[serde(deserialize_with = "deserialize_value_to_f64")]
    F64(f64),
    #[serde(deserialize_with = "deserialize_value_to_bool")]
    Bool(bool),
    #[serde(deserialize_with = "deserialize_value_to_pointer")]
    Pointer(Pointer),
    #[serde(deserialize_with = "deserialize_value_to_string")]
    String(String),
    #[serde(deserialize_with = "deserialize_value_to_struct")]
    Struct(Struct),
    #[serde(untagged)]
    Other,
}

fn deserialize_value_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is an i32")
        }

        fn visit_map<A>(self, mut map: A) -> Result<i32, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut name: Option<serde_json::Value> = None;
            let mut type_: Option<serde_json::Value> = None;
            let mut width: Option<serde_json::Value> = None;
            let mut data: Option<serde_json::Value> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "name" => name = map.next_value()?,
                    "type" => type_ = map.next_value()?,
                    "width" => width = map.next_value()?,
                    "data" => data = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "integer")
                && matches!(&type_, Some(serde_json::Value::String(s)) if s == "int" || s == "signed int")
                && matches!(&width, Some(serde_json::Value::Number(n)) if n.as_i64() == Some(32))
            {
                if let Some(serde_json::Value::String(s)) = data {
                    if let Ok(i) = i32::from_str(&s) {
                        return Ok(i);
                    }
                }
            }

            Err(Error::custom("not an i32 value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}

fn deserialize_value_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is an f64")
        }

        fn visit_map<A>(self, mut map: A) -> Result<f64, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut name: Option<serde_json::Value> = None;
            let mut width: Option<serde_json::Value> = None;
            let mut data: Option<serde_json::Value> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "name" => name = map.next_value()?,
                    "width" => width = map.next_value()?,
                    "data" => data = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "float")
                && matches!(&width, Some(serde_json::Value::Number(n)) if n.as_i64() == Some(64))
            {
                if let Some(serde_json::Value::String(s)) = data {
                    if let Ok(f) = f64::from_str(&s) {
                        return Ok(f);
                    }
                }
            }

            Err(Error::custom("not an f64 value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}

fn deserialize_value_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is a bool")
        }

        fn visit_map<A>(self, mut map: A) -> Result<bool, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut name: Option<serde_json::Value> = None;
            let mut type_: Option<serde_json::Value> = None;
            let mut width: Option<serde_json::Value> = None;
            let mut data: Option<serde_json::Value> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "name" => name = map.next_value()?,
                    "type" => type_ = map.next_value()?,
                    "width" => width = map.next_value()?,
                    "data" => data = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "boolean") {
                if let Some(serde_json::Value::Bool(b)) = data {
                    return Ok(b);
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "integer")
                && matches!(&type_, Some(serde_json::Value::String(s)) if s == "const boolean")
                && matches!(&width, Some(serde_json::Value::Number(n)) if n.as_i64() == Some(8))
            {
                if let Some(serde_json::Value::String(s)) = data {
                    if let Ok(b) = bool::from_str(&s) {
                        return Ok(b);
                    }
                }
            }

            Err(Error::custom("not a bool value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Pointer {
    pub data: Option<String>,
    pub type_: String,
}

fn deserialize_value_to_pointer<'de, D>(deserializer: D) -> Result<Pointer, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = Pointer;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is a pointer")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Pointer, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut name: Option<serde_json::Value> = None;
            let mut type_: Option<serde_json::Value> = None;
            let mut data: Option<serde_json::Value> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "name" => name = map.next_value()?,
                    "type" => type_ = map.next_value()?,
                    "data" => data = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "pointer")
                && matches!(&type_, Some(serde_json::Value::String(s)) if s.ends_with('*'))
            {
                if let Some(serde_json::Value::String(s)) = data {
                    let data = if s == "null" { None } else { Some(s) };

                    return Ok(Pointer {
                        data,
                        type_: type_.unwrap().as_str().unwrap().to_string(),
                    });
                }
            }

            Err(Error::custom("not a pointer value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}

fn deserialize_value_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is a string")
        }

        fn visit_map<A>(self, mut map: A) -> Result<String, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut name: Option<serde_json::Value> = None;
            let mut data: Option<serde_json::Value> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "name" => name = map.next_value()?,
                    "data" => data = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if matches!(&name, Some(serde_json::Value::String(s)) if s == "string") {
                if let Some(serde_json::Value::String(s)) = data {
                    return Ok(s);
                }
            }

            Err(Error::custom("not a string value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Struct {
    pub members: Vec<Member>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Member {
    pub name: String,
    pub value: Value,
}

fn deserialize_value_to_struct<'de, D>(deserializer: D) -> Result<Struct, D::Error>
where
    D: Deserializer<'de>,
{
    struct ValueVisitor;

    impl<'de> Visitor<'de> for ValueVisitor {
        type Value = Struct;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a value that is a struct")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Struct, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut members: Option<Vec<Member>> = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "members" => members = map.next_value()?,
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            if let Some(members) = members {
                return Ok(Struct { members });
            }

            Err(Error::custom("not a struct value"))
        }
    }

    deserializer.deserialize_map(ValueVisitor)
}
