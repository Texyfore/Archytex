use std::{
    io::stdin,
    sync::mpsc::Sender,
    thread::{spawn, JoinHandle},
};

use app::FromHost;

pub struct AsyncStdin {
    thread: Option<JoinHandle<()>>,
}

impl AsyncStdin {
    pub fn new(sender: Sender<FromHost>) -> Self {
        Self {
            thread: Some(spawn(move || loop {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input {
                    "res" => {
                        sender
                            .send(FromHost::Resolution {
                                width: 100,
                                height: 100,
                            })
                            .unwrap();
                        println!("[native-runner] changed resolution");
                    }
                    "save" => {
                        sender.send(FromHost::SaveScene).unwrap();
                        println!("[native-runner] requested save");
                    }
                    "exit" => {
                        println!("[native-runner] closed stdin");
                        break;
                    }
                    _ => {
                        println!("[native-runner] bad input");
                    }
                }
            })),
        }
    }
}

impl Drop for AsyncStdin {
    fn drop(&mut self) {
        self.thread.take().unwrap().join().unwrap()
    }
}
