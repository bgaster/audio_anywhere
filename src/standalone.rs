//! 
//! Wasmer implementation of standalone app
//! Copyright: Benedict R. Gaster
//! 
#![allow(dead_code)]

use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

extern crate crossbeam_channel;
use crossbeam_channel as cb;

use crate::gui::*;
use crate::audio_anywhere::*;
use crate::messages::*;
use crate::comms::*;
use crate::utils::*;
use crate::bundle::*;

extern crate portaudio;
use portaudio as pa;

pub struct Standalone<'a> {
    /// url used for interface, modules, and the like
    url: String, 
    /// bundle for the currently loaded module
    bundle: Bundle,
    /// swapable AA Unit, representing the currently loaded module
    aaunit: AAUnit,
    /// currently selected audio input device
    input_device: pa::DeviceIndex,
    /// currenlty selected audio outut device
    output_device: pa::DeviceIndex,
    /// GUI, only one instance for application, modules are injected iframe
    gui: GUI<'a>,
    /// incomming messages from GUI
    receive_from_gui: cb::Receiver<Message>,
    /// send from audio
    send_from_audio: Sender<(u32, Value)>,
    /// send from gui
    send_from_gui: cb::Sender<Message>,
    /// gui comms, for sending messages to GUI
    comms: Box<dyn Send>,
    comms_sender: Sender<Message>,
}

impl <'a>Standalone<'a> {
    pub fn new(url: &str) -> Result<Self> {
       
        // Load GUI HTML, index.html is the same for all anywhere modules
        let html = get_string(&[url, "index.html"].join("/")).unwrap(); 
        let modules = get_string(&[url, "modules.json"].join("/")).unwrap();
        Modules::from_json(&modules).and_then(|modules| {
            let (send_from_gui, receive_from_gui) = cb::unbounded();
            let (send_from_audio, receive_from_audio) = channel();

            let json = "default/default.json";
            let json = &modules.default.clone();

            Self::create_aaunit(url, json, send_from_audio.clone()).and_then(|(aaunit, bundle)| {
                let html = html.replace("$gui_page$", &bundle.gui.url)
                    .replace("$width$", &bundle.gui.width.to_string())
                    .replace("$height$", &bundle.gui.height.to_string());

                GUI::new(
                    &html[..],
                    Box::new(LocalSendCB::new(send_from_gui.clone())),
                    bundle.gui.params.clone(), //vec![Value::VFloat(-50.)],
                    "Audio Anywhere",
                    (900,900)).and_then(|gui| {
                        let pa = pa::PortAudio::new().unwrap();
                        let input_device = pa.default_input_device().unwrap();
                        let output_device = pa.default_output_device().unwrap();

                        let comms_sender = gui.comms_sender();
                        let comms = gui.comms();
                        
                        // send Modules to GUI
                        Self::send_modules(&comms_sender, &modules.modules);
                        // send Audio devices to GUI
                        Self::send_audio_devices(&comms_sender);
                        // set default values for GUI and AAUnit
                        Self::send_params(&comms_sender, &bundle.gui.params);
                        Self::set_params(&aaunit, &bundle.gui.params);
                        
                        Ok(Self {
                            url: url.to_string(),
                            bundle,
                            aaunit,
                            input_device,
                            output_device,
                            gui,
                            receive_from_gui,
                            send_from_audio,
                            send_from_gui,
                            comms,
                            comms_sender,
                        })
                })
            })
        })
    }

    // send a list of input/output audio devices to GUI
    fn send_audio_devices(comms: &Sender<Message>) {
        let pa = pa::PortAudio::new().unwrap();
        for device in pa.devices().unwrap() {
            let (index, info) = device.unwrap();
        
            if info.max_input_channels > 0 {
                Self::send_add_input_device(&comms, info.name, index);
            }

            if info.max_output_channels > 0 {
                Self::send_add_output_device(&comms, info.name, index);
            }
        }
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

    // send a list of modules to GUI
    fn send_modules(comms: &Sender<Message>, modules: &Vec<Module>) {
        for m in modules {
            Self::send_add_module(comms, &m.name, &m.json_url);
        }
    }

    // send a message to GUI to add a module to drop down menu
    fn send_add_module(comms: &Sender<Message>, name: &str, json_url: &str) {
        comms.send(Message {
            id: MessageID::AddModule,
            index: 0,
            value: Value::VString([name, json_url].join("="))
        }).unwrap();
    }

    // send a message to GUI to add an input audio device
    fn send_add_input_device(comms: &Sender<Message>, name: &str, index: pa::DeviceIndex) {
        comms.send(Message {
            id: MessageID::AddInputDevice,
            index: 0,
            value: Value::VString([name, &index.0.to_string()].join("="))
        }).unwrap();
    }

    // send a message to GUI to add an output audio device
    fn send_add_output_device(comms: &Sender<Message>, name: &str, index: pa::DeviceIndex) {
        comms.send(Message {
            id: MessageID::AddOutputDevice,
            index: 0,
            value: Value::VString([name, &index.0.to_string()].join("="))
        }).unwrap();
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

    // set a aaunit parameter
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

    // set aaunit parameters from a list of parameters
    fn set_params(aaunit: &AAUnit, params: &Vec<Value>) {
        for (index, param) in params.iter().enumerate() {
            Self::set_param(aaunit, index as u32, (*param).clone());
        }
    }

    /// audio handler for duplex streams (i.e. input and output)
    fn audio_duplex(
        aaunit: Rc<RefCell<AAUnit>>, 
        input_device: pa::DeviceIndex,
        output_device: pa::DeviceIndex,
        bundle: Bundle, 
        receive_from_gui: cb::Receiver<Message>, 
        send_from_audio: Sender<(u32, Value)>) -> Option<Message> {
        let pa = pa::PortAudio::new().unwrap();

        let input_params = pa::stream::Parameters::new(
            input_device, 
            bundle.info.inputs,
            true,
            0.1);

        let output_params = pa::stream::Parameters::new(
            output_device, 
            bundle.info.outputs,
            true,
            0.1);

        let settings = 
            pa::stream::DuplexSettings::new(
                input_params, output_params, 44_100.0, 64);

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
                                Self::set_param(&aaunit.borrow(), message.index, message.value);
                            },
                            MessageID::Control => {},
                            MessageID::ChangeModule 
                                | MessageID::AddInputDevice 
                                | MessageID::AddOutputDevice 
                                | MessageID::Exit => {
                                send_stop.send(Some(message.clone())).unwrap();
                                return pa::Complete;
                            },
                            _ => { }
                        }
                    }
                    else {
                        break;
                    }
                }                                                

                unsafe { 
                    LEFT_INPUT_MEMORY_BUFFER = &in_buffer[0]; //&input[0]; //
                    if bundle.info.inputs > 1 {
                        RIGHT_INPUT_MEMORY_BUFFER = &in_buffer[1];
                    }
                    LEFT_MEMORY_BUFFER = &mut out_buffer[0]; 
                    if bundle.info.outputs > 1 {
                        RIGHT_MEMORY_BUFFER = &mut out_buffer[1];
                    }
                }

                aaunit.borrow().compute.call(frames as u32).unwrap();

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

    /// audio handler for output stream only
    fn audio_output_only(
        aaunit: Rc<RefCell<AAUnit>>, 
        output_device: pa::DeviceIndex,
        bundle: Bundle, 
        receive_from_gui: cb::Receiver<Message>, 
        send_from_audio: Sender<(u32, Value)>) -> Option<Message> {
        let pa = pa::PortAudio::new().unwrap();

        let output_params = pa::stream::Parameters::new(
            output_device, 
            bundle.info.outputs,
            true,
            0.1);

        let settings = 
            pa::stream::OutputSettings::new(output_params, 44_100.0, 64);        

        let (send_stop,rec_stop) = channel();
        let callback = move |pa::OutputStreamCallbackArgs {
            buffer, 
            frames, 
            .. }| { 
                // handle any incomming messages from UI
                loop {
                    if let Ok(message) = receive_from_gui.try_recv() {
                        match message.id {
                            MessageID::Param => {
                                Self::set_param(&aaunit.borrow(), message.index, message.value);
                            },
                            MessageID::Control => {},
                            MessageID::ChangeModule 
                                | MessageID::AddInputDevice 
                                | MessageID::AddOutputDevice 
                                | MessageID::Exit => {
                                send_stop.send(Some(message.clone())).unwrap();
                                return pa::Complete;
                            },
                            _ => { }
                        }
                    }
                    else {
                        break;
                    }
                }                                                

                unsafe { 
                    LEFT_MEMORY_BUFFER = &mut buffer[0];
                    if bundle.info.outputs > 1 {
                        RIGHT_MEMORY_BUFFER = &mut buffer[1];
                    }
                }

                aaunit.borrow().compute.call(frames as u32).unwrap();

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

    /// audio handler for either duplex or output only streams
    #[inline]
    fn audio(
        aaunit: Rc<RefCell<AAUnit>>, 
        input_device: pa::DeviceIndex,
        output_device: pa::DeviceIndex,
        bundle: Bundle, 
        receive_from_gui: cb::Receiver<Message>, 
        send_from_audio: Sender<(u32, Value)>) -> Option<Message> {
        if bundle.info.inputs > 0 {
            Self::audio_duplex(aaunit, input_device, output_device, bundle, receive_from_gui, send_from_audio)
        }
        else {
            Self::audio_output_only(aaunit, output_device, bundle, receive_from_gui, send_from_audio)
        }
    }

    /// Take hold of module a run Audio handler and GUI.
    /// The audio handler can be dynanically swapped on module change or input/output audio device change
    pub fn run(self) -> Result<()> {
        let mut gui = self.gui;
        let aaunit = self.aaunit;
        let mut input_device = self.input_device;
        let mut output_device = self.output_device;
        let url = self.url;
        let receive_from_gui = self.receive_from_gui;
        let send_from_audio = self.send_from_audio;
        let comms = self.comms_sender;
        let mut bundle = self.bundle;

        // create thread to handle all things audio...
        let audio_thread = thread::spawn(move || { 
            let aaunit = Rc::new(RefCell::new(aaunit));
            // audio can quit for a number of reasons:
            //          request change input/ouput device
            //          change audio anywhere module
            //          exit application
            //          unknown error
            while let Some(message) = Self::audio(
                            aaunit.clone(),
                            input_device,
                            output_device,
                            bundle.clone(), 
                            receive_from_gui.clone(), 
                            send_from_audio.clone()) {
                match message.id {
                    // switch input device
                    MessageID::AddInputDevice => {
                        if let Value::VInt(index) =  message.value {
                            input_device = pa::DeviceIndex(index as u32);
                        }
                    },
                    // switch output device
                    MessageID::AddOutputDevice => {
                        if let Value::VInt(index) =  message.value {
                            output_device = pa::DeviceIndex(index as u32);
                        }
                    },
                    // switch module
                    MessageID::ChangeModule => {
                        if let Value::VString(json) = message.value {
                            if let Ok((au, bundle_new)) = 
                                Self::create_aaunit(&url, &json, send_from_audio.clone()) {
                                comms.send(
                                    Message::change_module(
                                        &([&url[..], 
                                            &bundle_new.gui.url[..]].join("")), 
                                            bundle_new.gui.width, 
                                            bundle_new.gui.height)).unwrap();
                                
                                // set default values for GUI
                                Self::send_params(&comms, &bundle_new.gui.params);
                                Self::set_params(&au, &bundle_new.gui.params);

                                // finally install the auunit and bundle
                                *aaunit.borrow_mut() = au;
                                bundle = bundle_new;
                            }
                        }
                    },
                    MessageID::Exit => {
                        break;
                    },
                    _ => { }
                }
            }
        });

        gui.run();

        // clear up audio thread
        self.send_from_gui.send(Message {
            id: MessageID::Exit,
            index: 0,
            value: Value::VInt(0),
        }).unwrap();
        audio_thread.join().unwrap();
        
        Ok(())
    }
}