use rocket::http::{ContentType, Status};
use rocket::response;
use rust_embed::RustEmbed;

extern crate vst;

use std::sync::mpsc::{channel, Sender, Receiver};

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};
use vst::event::Event;

use crate::gui::*;
use crate::audio_anywhere::*;
use crate::messages::*;
use crate::utils::*;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Asset;

struct Plug {
    /// swapable WASM audio plugin
    wasm_plugin: AAUnit,
    /// incomming messages from GUI
    receive_from_gui: Receiver<(Index, Value)>,
}

impl Plug {
    pub fn new() -> Result<Self> {
        let (send_from_gui, receive_from_gui) = channel();

        let mut wasm_code = get_vec("http://127.0.0.1:8000/gain/gain.wasm").unwrap();
        
        // TODO: create null/default plugin that is loaded on init
        AAUnit::new(
            send_from_gui,
            &wasm_code[..])
                .map(|wasm_plugin| Plug { wasm_plugin, receive_from_gui, })
    }
}

impl Plugin for Plug {
    fn get_info(&self) -> Info {
        self.wasm_plugin.info.clone()
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        match self.wasm_plugin.init.call(sample_rate as f64) {
            _ => {
                // errors go unreported
            }
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => (), //self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // intercept messages from UI, as we need to handle change plugin
        // can we really do this in the audio component?

        // loop {
        //     match self.incoming.try_recv() {
        //         Ok(e) => {
        //             match e {
        //                 UIEvent::Frequency(frequency) => {
        //                     self.set_freq.call(frequency as f64).unwrap();
        //                 },
        //                 _ => {
        //                     // handle more events
        //                 }
        //             }
        //         },
        //         _ => {
        //             break;
        //         }
        //     }
        // }       

        unsafe { 
            let samples = buffer.samples();
            LEFT_MEMORY_BUFFER = buffer.raw_outputs()[0];
            LEFT_INPUT_MEMORY_BUFFER = buffer.raw_inputs()[0];

            // RIGHT_MEMORY_BUFFER = buffer.raw_outputs()[1]; 
            match self.wasm_plugin.compute.call(samples as u32) {
                _ => {
                    // errors go unreported
                }
            }
        }
    }

}