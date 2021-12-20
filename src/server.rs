use std::{collections::VecDeque, fmt::Debug, fs::File, io::BufReader, thread::sleep, time::Duration};

use rodio::Sink;
use unix_ipc::{channel, Bootstrapper, Receiver, Sender};

use crate::messages::{ClientMsgVariant, ToClientMsg, ToServerMsg};

pub fn server_entry_point(mut boot: Bootstrapper<ToClientMsg>) -> i32 {
    let queue: VecDeque<BufReader<File>> = VecDeque::new();
    let (sink, stream) = Sink::new_idle();
    loop {
        let (tx, rx) = channel().unwrap();
        boot.send(ToClientMsg {
            var: ClientMsgVariant::JoinHandle,
            sender: tx,
        })
        .unwrap();
        let received = rx.recv().unwrap();
        match received {
            ToServerMsg::Stop => {
                return 0;
            },
            _ => {
                println!("{:?}", received);
            }
        }
        boot = fix_strapper(boot);
    }
}

// If a new Bootstrapper is not created, then it will panic because the client receiver has gone
// out of scope
fn fix_strapper(boot: Bootstrapper<ToClientMsg>) -> Bootstrapper<ToClientMsg> {
    let path = boot.path().to_owned();
    drop(boot);
    Bootstrapper::bind(path).unwrap()
}
