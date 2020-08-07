use serde::{Deserialize};
use serde_repr::{Deserialize_repr};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    VInt(i32),
    VFloat(f32),
    VString(String),
    VVU8(Vec<u8>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::VFloat(f) => f.to_string(),
            Self::VInt(i) => i.to_string(),
            Self::VString(s) => s.clone(),
            Self::VVU8(v) => {
                let mut s = "[".to_string();
                for (i, u) in v.iter().enumerate() {
                    s.push((*u) as char );
                    if i + 1 != v.len() {
                        s.push(',')
                    }
                }
                s.push(']');
                s
            },
        }
    }
}

pub type Index = u32;