#![feature(decl_macro, proc_macro_hygiene)]
extern crate rocket;

use rust_embed::RustEmbed;
use rocket_contrib::serve::StaticFiles;

use std::sync::mpsc::{channel, Sender};

mod gui;
use gui::*;

mod plugin;
use plugin::*;

mod audio_anywhere;
use audio_anywhere::*;

mod standalone;
use standalone::*;

mod comms;
mod messages;
mod bundle;
mod utils;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Asset;

//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------

fn main() {
    Standalone::new("http://127.0.0.1:8000")
        .unwrap()
        .run()
        .unwrap();
}