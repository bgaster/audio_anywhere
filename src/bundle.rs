#[macro_use]
use serde::{Deserialize};
use serde_repr::{Deserialize_repr};

use crate::utils::{err, ok, Result};
use crate::messages::*;

#[derive(Deserialize, Debug, Clone)]
pub struct GUIBundle {
    pub url: String,
    pub name: String,
    pub params: Vec<Value>,
    pub width: i32,
    pub height: i32, 
}

#[derive(Deserialize, Debug, Clone)]
pub struct Info {
    name: String,
    vendor: String,
    presets: u32,
    parameters: u32,
    inputs: u32,
    outputs: u32,
    midi_inputs: u32,
    midi_outputs: u32,
    id: u32,
    version: u32,
    category: String,
    initial_delay: u32,
    preset_chunks: bool,
    f64_precision: bool,
    silent_when_stopped: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bundle {
    pub wasm_url: String,
    pub gui: GUIBundle,
    pub info: Info,
}

impl Bundle {
    pub fn from_json(data: &str) -> Result<Self> {
        let bundle : serde_json::Result<Bundle> = serde_json::from_str(data);
        //println!("{:?}", bundle);
        bundle.map_or(err(), |b| ok(b))
    }
}