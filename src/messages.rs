#[macro_use]
use serde::{Deserialize};
use serde_repr::{Deserialize_repr};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    VInt(i32),
    VFloat(f32),
    VString(String),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::VFloat(f) => f.to_string(),
            Self::VInt(i) => i.to_string(),
            Self::VString(s) => s.clone(),
        }
    }
}

pub type Index = u32;