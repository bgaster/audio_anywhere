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
    // let _rocket_server = std::thread::Builder::new()
    //         .spawn(move || {
    //             rocket::ignite()
    //                 .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")))
    //                 .launch()
    //             //rocket::ignite().mount("/", routes![index, dist]).launch();
    //         }).unwrap();

    // let html_content = Asset::get("gain.html").unwrap();

    // let (audio_sender, audio_receiver) = channel();

    // // representative of our audio thread
    // let _audio_server = std::thread::Builder::new()
    //         .spawn(move || {
    //             loop {
    //                 match audio_receiver.recv() {
    //                     Ok(message) => { 
    //                         println!("recv: {:?}", message);
    //                     },
    //                     _ => { 
    //                         break; 
    //                     }
    //                 }
                    
    //             }
    //         }).unwrap();

    // let mut gui = GUI::new(
    //     std::str::from_utf8(&html_content).unwrap(), 
    //     audio_sender,
    //     vec![Value::VFloat(45.)],
    //     "Gain WASM",
    //     (220, 200)).unwrap();
    // let gui_comms = gui.comms();
    // gui.run();

    let mut standalone = Standalone::new("http://127.0.0.1:8000").unwrap();
    standalone.run().unwrap();
}