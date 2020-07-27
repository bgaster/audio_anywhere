use std::sync::mpsc::{channel, Sender, Receiver};


use crate::messages::*;
use crossbeam_channel as cb;

type CommsError = ();

#[derive(Clone, PartialEq, Debug)]
pub enum MessageID {
    /// parameter change
    Param = 0,
    /// control change
    Control = 1,
    /// change current module
    ChangeModule = 2,
    /// add input device
    AddInputDevice = 3,
    AddOutputDevice = 4,
    Loaded = 5,
    Exit = 6,
    AddModule = 7,
}

#[derive(Clone, Debug)]
pub enum Payload {
    SValue(Value),
    VValue(Vec<Value>),
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: MessageID,
    pub index: Index,
    pub value: Value,
}

impl Message {
    #[inline]
    pub fn change_module(url: &str, width: i32, height: i32) -> Self {
        Message {
            id: MessageID::ChangeModule,
            index: 0,
            value: Value::VString([url, &width.to_string(), &height.to_string()].join(" ")),
        }
    }
}

pub trait Send {
    fn send(&self, id: MessageID, index: Index, v: Value) -> Result<(), CommsError>;
}

pub trait Receive {
    fn recv(&self, index: Index) -> Result<(MessageID, Value), CommsError>;
    fn try_recv(&self, index: Index) -> Result<(MessageID, Value), CommsError>;
}

//-----------------------------------------------------------------------------

pub struct LocalSend {
    sender: Sender <Message>,
}

impl LocalSend {
    pub fn new(sender: Sender <Message>) -> Self {
        Self {
            sender,
        }
    }
}

impl Send for LocalSend {
    fn send(&self, id: MessageID, index: Index, value: Value) -> Result<(), ()> {
        self.sender.send(Message { id, index, value }).map_or(Err(()), |_| Ok(()))
    }
}

pub struct LocalSendCB {
    sender: cb::Sender <Message>,
}

impl LocalSendCB {
    pub fn new(sender: cb::Sender <Message>) -> Self {
        Self {
            sender,
        }
    }
}

impl Send for LocalSendCB {
    fn send(&self, id: MessageID, index: Index, value: Value) -> Result<(), ()> {
        self.sender.send(Message { id, index, value }).map_or(Err(()), |_| Ok(()))
    }
}

//TODO:
struct RemoteSend;

//-----------------------------------------------------------------------------
