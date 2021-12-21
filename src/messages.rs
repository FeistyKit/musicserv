use serde::{Deserialize, Serialize};
use unix_ipc::Sender;

#[derive(Debug, Serialize, Deserialize)]
pub enum ToServerMsg {
    Pause,
    ExitCommunication,
    Unpause,
    TogglePause,
    Skip,
    AddFile(String),
    Clear,
    Stop,
    ListRunning,
    CurrentlyPlaying,
    ToggleLoop
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToClientMsg {
    pub var: ClientMsgVariant,
    pub sender: Option<Sender<ToServerMsg>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsgVariant {
    JoinHandle, // This variant is only to queue up a message
    CurrentlyPlaying(String),
    Queue(Vec<String>),
    Error(String),
}
