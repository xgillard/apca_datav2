//! This module contains utility function that help customizing the 
//! serial/deserialization process.

use serde::Deserialize;
use serde_json::Value;

pub(crate) fn null_as_emptyvec<'de, T, D>(d: D) -> Result<Vec<T>, D::Error>
where D: serde::Deserializer<'de>,
      T: serde::Deserialize<'de>
{
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or_default()
        })
}

pub(crate) fn number_as_f64<'de, D>(d: D) -> Result<f64, D::Error>
where D: serde::Deserializer<'de>,
{
    match Value::deserialize(d)? {
        Value::String(txt) => 
            if let Ok(val) = txt.parse::<f64>() {
                Ok(val)
            } else {
                Err(serde::de::Error::custom("expected a number"))
            },
        Value::Number(num) => 
            Ok(num.as_f64().ok_or_else(|| serde::de::Error::custom("Invalid number"))?),
        _ => 
            Err(serde::de::Error::custom("expected a number"))
    }
}

pub(crate) fn option_as_f64<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where D: serde::Deserializer<'de>,
{
    match Value::deserialize(d)? {
        Value::String(txt) => 
            if let Ok(val) = txt.parse::<f64>() {
                Ok(Some(val))
            } else {
                Err(serde::de::Error::custom("expected a number"))
            },
        Value::Number(num) => 
            Ok(Some(num.as_f64().ok_or_else(|| serde::de::Error::custom("Invalid number"))?)),
        Value::Null => 
            Ok(None),
        _ => 
            Err(serde::de::Error::custom("expected a number"))
    }
}

