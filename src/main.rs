use std::process::exit;

use clap::{load_yaml, App};
use messages::ToClientMsg;
use nix::unistd::daemon;
use server::server_entry_point;
use unix_ipc::Receiver;
use unix_ipc::Bootstrapper;

use crate::messages::ToServerMsg;

mod messages;
mod server;

fn main() {
    let ret = logic();
    exit(ret);
}

fn logic() -> i32 {
    let args = load_yaml!("./cli.yml");
    let matches = App::from_yaml(args).get_matches();
    let socket_serv = matches.value_of("socket").unwrap(); // It will always have a value
    match matches.subcommand() {
        ("start", _) => {
            if !matches.is_present("foreground") {
                if let Err(_) = daemon(true, false) {
                    eprintln!("Could not start the server program!");
                    return 1;
                }
            }
            match Bootstrapper::bind(socket_serv) {
                Err(_) => {
                    eprintln!("Could not connect to the server socket at {}!", socket_serv);
                    return 1;
                }
                Ok(boot) => server_entry_point(boot),
            }
        }
        ("stop", _) => {
            send_message(socket_serv, ToServerMsg::Stop);
            return 0;
        },
        ("pause", _) => {
            send_message(socket_serv, ToServerMsg::Pause);
            return 0;
        },
        ("unpause", _) => {
            send_message(socket_serv, ToServerMsg::Unpause);
            return 0;
        },
        ("toggle", _) => {
            send_message(socket_serv, ToServerMsg::TogglePause);
            return 0;
        },
        ("skip", _) => {
            send_message(socket_serv, ToServerMsg::Skip);
            return 0;
        },
        ("clear", _) => {
            send_message(socket_serv, ToServerMsg::Clear);
            return 0;
        },
        ("loop", _) => {
            send_message(socket_serv, ToServerMsg::ToggleLoop);
            return 0;
        },
        _ => {
            unreachable!()
        }
    }
}

fn send_message(serv_path: &str, msg: ToServerMsg) {
    let recv = get_receiver(serv_path);
    let t: ToClientMsg = recv.recv().unwrap();
    t.sender.unwrap().send(msg).unwrap();
}

fn get_receiver(serv_path: &str) -> Receiver<ToClientMsg> {
    Receiver::<ToClientMsg>::connect(serv_path)
        .expect(&format!("Could not connect to socket {}", serv_path))
}
