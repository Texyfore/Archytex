use std::{
    fs,
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
                let mut tokens = input.split(' ');

                if let Some(token) = tokens.next() {
                    match token {
                        "res" => {
                            sender
                                .send(FromHost::Resolution {
                                    width: tokens.next().unwrap().parse().unwrap(),
                                    height: tokens.next().unwrap().parse().unwrap(),
                                })
                                .unwrap();
                            println!("[native-runner] changed resolution");
                        }
                        "save" => {
                            sender.send(FromHost::SaveScene(0)).unwrap();
                            println!("[native-runner] requested save");
                        }
                        "load" => {
                            let name = tokens.next().unwrap();
                            let path = format!("{}.ascn", name);
                            let buf = fs::read(&path).unwrap();
                            sender.send(FromHost::LoadScene(buf)).unwrap();
                            println!("[native-runner] loading `{}`", path);
                        }
                        "texture" => {
                            let texture = tokens.next().unwrap().parse().unwrap();
                            sender.send(FromHost::Texture(texture)).unwrap();
                            println!("[native-runner] picked texture `{}`", texture);
                        }
                        "prop" => {
                            let prop = tokens.next().unwrap().parse().unwrap();
                            sender.send(FromHost::Prop(prop)).unwrap();
                            println!("[native-runner] picked prop `{}`", prop);
                        }
                        "button" => {
                            let button = tokens.next().unwrap().parse().unwrap();
                            sender.send(FromHost::Button(button)).unwrap();
                            println!("[native-runner] pressed button {}", button);
                        }
                        "exit" => {
                            println!("[native-runner] closed stdin");
                            break;
                        }
                        _ => {
                            println!("[native-runner] bad input");
                        }
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
