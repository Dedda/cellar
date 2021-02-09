use std::convert::TryInto;
use std::fmt::{Display, Formatter};

pub struct ConversionError(String);

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone)]
pub enum Value {
    Text(String),
    Bool(bool),
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn parse(data: String) -> Self {
        use Value::*;
        if data.eq("TRUE") {
            Bool(true)
        } else if data.eq("FALSE") {
            Bool(false)
        } else if let Ok(i) = data.parse() {
            Int(i)
        } else if let Ok(f) = data.parse() {
            Float(f)
        } else {
            Text(data)
        }
    }
}

impl Into<String> for &Value {
    fn into(self) -> String {
        use Value::*;
        match self {
            Text(s) => s.clone(),
            Bool(b) => if b.clone() { "TRUE" } else { "FALSE" }.into(),
            Int(i) => format!("{}", i),
            Float(f) => format!("{}", f),
        }
    }
}

impl TryInto<bool> for &Value {
    type Error = ConversionError;

    fn try_into(self) -> Result<bool, Self::Error> {
        use Value::*;
        match self {
            Text(s) => if s.eq("TRUE") {
                Ok(true)
            } else if s.eq("FALSE") {
                Ok(false)
            } else {
                Err(ConversionError(format!("Cannot convert `{}` to bool", s)))
            },
            Bool(b) => Ok(b.clone()),
            Int(i) => Ok(i > &0),
            Float(f) => Ok(f > &0.0),
        }
    }
}

impl TryInto<i64> for &Value {
    type Error = ConversionError;

    fn try_into(self) -> Result<i64, Self::Error> {
        use Value::*;
        match self {
            Text(s) =>
                if let Ok(i) = s.parse() {
                    Ok(i)
                } else {
                    Err(ConversionError(format!("Cannot convert `{}` to int", s)))
                },
            Bool(b) => if b.clone() { Ok(1) } else { Ok(0) },
            Int(i) => Ok(i.clone()),
            Float(f) => Ok(f.clone() as i64),
        }
    }
}

impl TryInto<f64> for &Value {
    type Error = ConversionError;

    fn try_into(self) -> Result<f64, Self::Error> {
        use Value::*;
        match self {
            Text(s) =>
                if let Ok(f) = s.parse() {
                    Ok(f)
                } else {
                    Err(ConversionError(format!("Cannot convert `{}` to float", s)))
                },
            Bool(b) => if b.clone() { Ok(1.0) } else { Ok(0.0) },
            Int(i) => Ok(i.clone() as f64),
            Float(f) => Ok(f.clone()),
        }
    }
}