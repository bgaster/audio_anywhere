
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::sync::Arc;

extern crate crossbeam_channel;
use crossbeam_channel as cb;

use crate::gui::*;
use crate::audio_anywhere::*;
use crate::messages::*;
use crate::comms::*;
use crate::utils::*;
use crate::bundle::*;

use curl::easy::Easy;

use std::f64::consts::PI;

extern crate portaudio;
use portaudio as pa;

pub struct Standalone<'a> {
    /// url used for interface, modules, and the like
    url: String, 
    bundle: Bundle,
    /// swapable AA Unit
    aaunit: AAUnit,
    gui: GUI<'a>,
    /// incomming messages from GUI
    receive_from_gui: cb::Receiver<Message>,
    /// send from audio
    send_from_audio: Sender<(u32, Value)>,
    /// gui comms, for sending messages to GUI
    comms: Box<dyn Send>,
    comms_sender: Sender<Message>,
}

impl <'a>Standalone<'a> {
    pub fn new(url: &str) -> Result<Self> {
       
        // Load GUI HTML, index.html is the same for all anywhere modules
        let html = get_string(&[url, "index.html"].join("/")).unwrap(); 
        
        let (send_from_gui, receive_from_gui) = cb::unbounded();
        let (send_from_audio, receive_from_audio) = channel();

        let json = "default/default.json";
        let json = "gain/gain.json";

        Self::create_aaunit(url, json, send_from_audio.clone()).and_then(|(aaunit, bundle)| {
            let html = html.replace("$gui_page$", &bundle.gui.url)
                .replace("$width$", &bundle.gui.width.to_string())
                .replace("$height$", &bundle.gui.height.to_string());

            GUI::new(
                &html[..],
                Box::new(LocalSendCB::new(send_from_gui)),
                bundle.gui.params.clone(), //vec![Value::VFloat(-50.)],
                "Audio Anywhere",
                (600,600)).and_then(|gui| {
                    let comms = gui.comms();
                    let comms_sender = gui.comms_sender();
                    // set default values for GUI and AAUnit
                    Self::send_params(&comms_sender, &bundle.gui.params);
                    Self::set_params(&aaunit, &bundle.gui.params);
                    Ok(Self {
                        url: url.to_string(),
                        bundle,
                        aaunit,
                        gui,
                        receive_from_gui,
                        send_from_audio,
                        comms,
                        comms_sender,
                    })
            })
        })
    }

    // send a list of params settings, indexed by position in the vector, to GUI
    fn send_params(comms: &Sender<Message>, params: &Vec<Value>) {
        for (index, p) in params.iter().enumerate() {
            comms.send(Message {
                id: MessageID::Param,
                index: index as Index, 
                value: (*p).clone(),
            }).unwrap();
        }
    }

    /// create an instance of an aaunit
    fn create_aaunit(url: &str, json: &str, send_from_audio: Sender<(u32, Value)>) -> Result<(AAUnit, Bundle)> {
        // firstly load the json bundle
        get_string(&[url, json].join("/")).and_then(|json| {
            Bundle::from_json(&json).and_then(|bundle| {
                // Load WASM module
                get_vec(&[url, &bundle.wasm_url].join("")).and_then(|wasm_code| {
                    AAUnit::new(send_from_audio.clone(), &wasm_code[..]).map(|aaunit| {
                        (aaunit, bundle)
                    })
                })
            })
        })
    }

    #[inline]
    fn set_param(aaunit: &AAUnit, index: Index, param: Value) {
        match param {
            Value::VFloat(f) => {
                aaunit.set_param_float.call(index, f).unwrap();
            },
            Value::VInt(i) => {
                aaunit.set_param_int.call(index, i).unwrap();
            },
            _ => {
            }
        }
    }

    fn set_params(aaunit: &AAUnit, params: &Vec<Value>) {
        for (index, param) in params.iter().enumerate() {
            Self::set_param(aaunit, index as u32, (*param).clone());
        }
    }

    fn audio(
        aaunit: AAUnit, 
        url: String, 
        receive_from_gui: cb::Receiver<Message>, 
        send_from_audio: Sender<(u32, Value)>) -> Option<String> {
        let pa = pa::PortAudio::new().unwrap();

        const TABLE_SIZE: usize = 200;

        let mut sine = [0.0; TABLE_SIZE];
        for i in 0..TABLE_SIZE {
            sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        }
        let mut left_phase = 0;
        let mut right_phase = 0;

        let mut settings =
            pa.default_duplex_stream_settings(1, 1, 44_100.0, 64).unwrap();

        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        let mut input: [f32;64] = [0.;64];

        let (send_stop,rec_stop) = channel();

        let callback = move |pa::DuplexStreamCallbackArgs {
            in_buffer, 
            out_buffer, 
            frames, 
            .. }| {
                
                // handle any incomming messages from UI
                loop {
                    if let Ok(message) = receive_from_gui.try_recv() {
                        match message.id {
                            MessageID::Param => {
                                Self::set_param(&aaunit, message.index, message.value);
                            },
                            MessageID::Control => {},
                            MessageID::ChangeModule => {
                                if let Value::VString(s) = message.value {
                                    send_stop.send(Some(s)).unwrap();
                                    return pa::Complete;
                                }
                            },
                        }
                    }
                    else {
                        break;
                    }
                }                                                

                let mut idx = 0;
                for _ in 0..frames {
                    let foo: f32 = in_buffer[idx];
                    input[idx] = sine[left_phase];
                    //buffer[idx + 1] = sine[right_phase];
                    left_phase += 1;
                    if left_phase >= TABLE_SIZE {
                        left_phase -= TABLE_SIZE;
                    }
                    right_phase += 3;
                    if right_phase >= TABLE_SIZE {
                        right_phase -= TABLE_SIZE;
                    }
                    idx += 1;
                }

                unsafe { 
                    LEFT_INPUT_MEMORY_BUFFER = &input[0]; //&in_buffer[0];
                    LEFT_MEMORY_BUFFER = &mut out_buffer[0]; 
                }

                aaunit.compute.call(frames as u32).unwrap();

                pa::Continue
        };

        let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();
        stream.start().unwrap();

        // block until we recieve message to swap module
        match rec_stop.recv() {
            Ok(s) => {
                stream.stop().unwrap();
                s 
            }
            _ => {
                stream.stop().unwrap();
                None
            }
        }
    }

    pub fn run(mut self) -> Result<()> {
        let mut gui = self.gui;
        let mut aaunit = Some(self.aaunit); // help the borrow checker
        let url = self.url;
        let receive_from_gui = self.receive_from_gui;
        let send_from_audio = self.send_from_audio;
        let comms = self.comms_sender;

        let _audio_thread = thread::spawn(move || { 
            loop {
                match Self::audio(
                    aaunit.unwrap(), // save as we wrapped it
                    url.clone(), 
                    receive_from_gui.clone(), 
                    send_from_audio.clone()) {
                    Some(json) => {
                        if let Ok((au, bundle)) = Self::create_aaunit(&url, &json, send_from_audio.clone()) {
                            // order of messages is important here
                            comms.send(
                                Message::change_module(
                                    &([&url[..], &bundle.gui.url[..]].join("")), bundle.gui.width, bundle.gui.height)).unwrap();
                            // set default values for GUI
                            Self::send_params(&comms, &bundle.gui.params);
                            Self::set_params(&au, &bundle.gui.params);
                            aaunit = Some(au);
                        }
                        else {
                            aaunit = None;
                            break;
                        }
                    },
                    _ => {
                        // TODO: send quit message to GUI..
                        aaunit = None;
                        break;
                    }
                }
            }
        });

        gui.run();
        
        Ok(())
    }
}