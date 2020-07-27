use serde::{Deserialize};
use serde_repr::{Deserialize_repr};

use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;

use web_view::*;

extern crate vst;

use std::os::raw::c_void;

use crate::messages::*;
use crate::comms::*;

#[derive(Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u16)]
#[serde(untagged)]
pub enum MsgType {
    Console = 0,
    SendParam = 1,
    ChangeModule = 2,
    ChangeInputDevice = 3,
    ChangeOutputDevice = 4,
    Loaded = 5,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UIMessage  {
    pub msg: MsgType,
    pub index: u32,
    pub value: Option<Value>,
}

type JavascriptCallback = Box<dyn FnMut(&mut web_view::WebView<()>, &str) -> WVResult>;

struct Handler {
    sender: Box<dyn Send>,
    gui_sender: Sender<Message>,
}

impl Handler  {
    pub fn new(sender: Box<dyn Send>, gui_sender: Sender<Message>) -> Self {
        Self {
            sender,
            gui_sender,
        }
    }

    pub fn receive(&mut self, index: Index, value: Value) {
        self.sender.send(MessageID::Param, index, value ).unwrap();
    }

    pub fn console(&self, s: &str) -> () {
        println!("{}", s);
    }

    pub fn change_module(&mut self, value: Value) {
        self.sender.send(MessageID::ChangeModule, 0, value).unwrap();
    }

    pub fn add_input_device(&mut self, value: Value) {
        self.sender.send(MessageID::AddInputDevice, 0, value).unwrap();
    }

    pub fn add_output_device(&mut self, value: Value) {
        self.sender.send(MessageID::AddOutputDevice, 0, value).unwrap();
    }

    pub fn loaded(&mut self) {
        self.gui_sender.send(Message {
            id: MessageID::Loaded,
            index: 0,
            value: Value::VInt(0),
        }).unwrap();
    }
}

pub struct GUI<'a> {
    webview: WebView<'a, ()>,
    // true once webview has loaded, message sent from onLoad
    loaded: bool,
    size: (i32,i32),
    is_open: bool,
    external_sender: Sender<Message>,
    external_receiver: Receiver<Message>,
}

impl <'a> GUI<'a> {
    pub fn new(
        html: &str, 
        audio_sender: Box<dyn Send>,
        params: Vec<Value>,
        title: &'a str,
        size: (i32,i32)) -> Result<Self, ()> {

        let (external_sender, external_receiver) = channel();

        let handler = Handler::new(audio_sender, external_sender.clone());

        match web_view::builder()
            .title(title)
            .content(Content::Html(html))
            .size(size.0, size.1)
            .resizable(false)
            .debug(true)
            .user_data(())
            .invoke_handler(Self::create_javascript_callback(handler))
            .build() {
                Ok(webview) => {
                    Ok(GUI {
                        webview,
                        loaded: false,
                        size,
                        is_open: false,
                        external_sender,
                        external_receiver
                    })
                },
                _ => {
                    Err(())
                }
            }
    }

    pub fn comms_sender(&self) -> Sender<Message> {
        self.external_sender.clone()
    }

    pub fn comms(&self) -> Box<dyn Send> {
        Box::new(LocalSend::new(self.external_sender.clone()))
    } 

    pub fn _comms_arc(&self) -> Arc<dyn Send> {
        Arc::new(LocalSend::new(self.external_sender.clone()))
    }

    pub fn close(&mut self) {
        self.webview.exit();
    } 

    pub fn run(&mut self) {
        self.is_open = true;
        
        let mut msgs = Vec::new();

        loop {
            
            // process any incoming messages
            loop {
                match self.external_receiver.try_recv() {
                    Ok(m) => {
                        match m.id {
                            MessageID::Loaded => {
                                self.loaded = true;
                            },
                            _ => {
                                msgs.push(m.clone());
                            }
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }

            if self.loaded {
                for m in msgs.iter() {
                    match (*m).id {
                        MessageID::Param => {
                            Self::param_change(&mut self.webview, (*m).index, (*m).value.to_string()).unwrap();
                        }
                        MessageID::Control => {
                            Self::control_change(&mut self.webview, (*m).index, (*m).value.to_string()).unwrap();
                        }
                        MessageID::ChangeModule => {
                            let str = (*m).value.to_string().clone();
                            let args: Vec<&str> = str.split_whitespace().collect();
                            Self::change_module(&mut self.webview, args[0], args[1], args[2]).unwrap();
                        },
                        MessageID::AddInputDevice => {
                            let str = (*m).value.to_string().clone();
                            let args: Vec<&str> = str.split("=").collect();
                            Self::add_input_device(&mut self.webview, args[0], args[1]).unwrap();
                        },
                        MessageID::AddOutputDevice => {
                            let str = (*m).value.to_string().clone();
                            let args: Vec<&str> = str.split("=").collect();
                            Self::add_output_device(&mut self.webview, args[0], args[1]).unwrap();
                        },
                        MessageID::Exit => {
                            // TODO: add Exit message?
                        },
                        MessageID::AddModule => {
                            let str = (*m).value.to_string().clone();
                            let args: Vec<&str> = str.split("=").collect();
                            Self::add_module(&mut self.webview, args[0], args[1]).unwrap();
                        },
                        _ => {
                            // no need to handle loaded
                        }
                    }
                }
                msgs.clear();
            }

            // step the webview
            match self.webview.step() {
                Some(Ok(_)) => (),
                Some(e) => e.unwrap(),
                None => {
                    break;
                },
            }
        }
    }

    fn create_javascript_callback(mut handler: Handler) -> JavascriptCallback {
        Box::new(move |webview: &mut web_view::WebView<()>, args: &str| {
            let message : serde_json::Result<UIMessage> = serde_json::from_str(args);
           //println!("{:?} {}", message, args);
            match message {
                Ok(message) => {
                    match message.msg {
                            MsgType::SendParam => {
                                return message.value.clone()
                                    .map_or(Ok(()), |v| { handler.receive(message.index, v); Ok(()) });
                            },
                            MsgType::Console => {
                                return message.value.clone()
                                    .map_or(Ok(()), |v| { handler.console(&v.to_string()[..]); Ok(()) });
                            },
                            MsgType::ChangeModule => {
                                return message.value.clone()
                                    .map_or(Ok(()), |v| { handler.change_module(v); Ok(()) });
                            },
                            MsgType::ChangeInputDevice => {
                                return message.value.clone()
                                    .map_or(Ok(()), |v| { handler.add_input_device(v); Ok(()) });
                            },
                            MsgType::ChangeOutputDevice => {
                                return message.value.clone()
                                    .map_or(Ok(()), |v| { handler.add_output_device(v); Ok(()) });
                            },
                            MsgType::Loaded => {
                                handler.loaded();
                            }
                        }
                },
                _ => {}
            }
            Ok(())
        })
    }

    fn param_change(webview: &mut WebView<()>, param_index: u32, data: String) -> WVResult {
        webview.eval(&format!("OnParamChange({},{})", param_index, data)).unwrap();
        Ok(())
    }

    fn control_change(webview: &mut WebView<()>, ctrl_index: u32, data: String) -> WVResult {
        webview.eval(&format!("OnControlChange({},{})", ctrl_index, data)).unwrap();
        Ok(())
    }

    fn change_module(webview: &mut WebView<()>, url: &str, width: &str, height: &str) -> WVResult {
        let eval = format!("OnModuleChange(\"{}\",\"{}\",\"{}\")", url, width, height);
        webview.eval(&eval).unwrap();
        Ok(())
    }

    fn add_input_device(webview: &mut WebView<()>, name: &str, index: &str) -> WVResult {
        webview.eval(&format!("OnAddInputDevice({},\"{}\")", index, name)).unwrap();
        Ok(())
    }

    fn add_output_device(webview: &mut WebView<()>, name: &str, index: &str) -> WVResult {
        webview.eval(&format!("OnAddOutputDevice({},\"{}\")", index, name)).unwrap();
        Ok(())
    }

    fn add_module(webview: &mut WebView<()>, name: &str, json_url: &str) -> WVResult {
        webview.eval(&format!("OnAddModule(\"{}\",\"{}\")", name, json_url)).unwrap();
        Ok(())
    }
}

impl <'a> vst::editor::Editor for GUI<'a> {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        (0,0)
    }

    fn close(&mut self) {
        self.close()
    }

    fn open(&mut self, _parent_handle: *mut c_void) -> bool {
        self.run();
        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }
}