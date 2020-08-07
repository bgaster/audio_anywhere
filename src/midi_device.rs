extern crate midir;
extern crate rimd;

use rimd::{MidiMessage, Status};

use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, Ignore, MidiInputConnection, MidiInputPort};
//use std::sync::mpsc::{Sender};
use crossbeam_channel as cb;

use crate::midi_utils::NoteSym;

use crate::utils::*;

// pub struct DeviceIn {
//     input: MidiInput,
//     port: usize,
// }

pub struct Midi {
    input_connections: Vec<MidiInputConnection<()>>,
}

unsafe impl Send for Midi {

}

impl Midi {
    pub fn new() -> Self {
       Self {
           input_connections: Vec::new(),
       }
    }

    pub fn get_inputs(&self) -> Result<Vec<String>> {
        MidiInput::new("midi input").map_or(
            err(), 
            |input| { 
                let mut inputs = Vec::new();
                for (i, p) in input.ports().iter().enumerate() {
                    if let Ok(name) = input.port_name(p) {
                        inputs.push(name);
                    }
                }
            ok(inputs)
        })
    }

    pub fn open_input(&mut self, device_name: String, sender: cb::Sender<MidiMessage>) -> Result<()> {
        MidiInput::new("midi input").map_or(
            err(), 
            |input| { 
            for (i, p) in input.ports().iter().enumerate() {
                if let Ok(name) = input.port_name(p) {
                    if name == device_name {
                        let connection = input.connect(
                            p, 
                            &name, 
                            move |stamp, message, _| {
                                //println!("{}: {:?} (len = {})", stamp, message, message.len());
                                let message = MidiMessage::from_bytes(message.iter().cloned().collect());
                                match sender.send(message) {
                                    _ => {}
                                }
                            }, ());
                        
                        if connection.is_err() {
                            return err();
                        }
                        self.input_connections.push(connection.unwrap());
                        return ok(());
                    }
                }
            }
            err()
        })
    }
}

