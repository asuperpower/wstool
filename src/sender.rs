/* really the business logic for our websocket client */
/* hence, application layer and TIGHTLY coupled to our file parser */

extern crate scoped_threadpool;
extern crate url;
extern crate websocket;

use std::sync::mpsc::channel;
use std::thread;

use url::Url;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

use scoped_threadpool::Pool; // 0.1.9

use crate::file_parser;

pub struct Sender {
    websocket_path: Url,
    send_messages: Vec<file_parser::WsMessage>,
}

/* we open a websocket client based on parameters and iterate through */
impl Sender {
    pub fn new(websocket_path: &String, send_messages: Vec<file_parser::WsMessage>) -> Sender {
        let url = Url::parse(websocket_path).unwrap();
        Sender {
            websocket_path: url,
            send_messages,
        }
    }

    pub fn start(&self) {
        /* The websocket bits are lifted from:
         * https://github.com/websockets-rs/rust-websocket */

        // Init
        let mut block_wait_response = false;

        // Connect
        println!("Connecting to {}", self.websocket_path);

        let client = ClientBuilder::new(&self.websocket_path.to_string())
            .unwrap()
            // .connect_secure(None)
            // .add_protocol("rust-websocket")
            .connect_insecure()
            .unwrap();

        println!("Connected!");

        let (mut receiver, mut sender) = client.split().unwrap();

        let (tx, rx) = channel();

        let tx_1 = tx.clone();

        let send_loop = thread::spawn(move || {
            loop {
                // Send loop
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        // If it's a close message, just send it and then return.
                        return;
                    }
                    _ => (),
                }
                // Send the message
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });

        let mut pool = Pool::new(2);

        pool.scoped(|scoped| {
            // Receive loop
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        // Got a close message, so send a close message and return
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        }
                    }
                    // Say what we received and unset block wait_receive block
                    _ => {
                        println!("Receive Loop: {:?}", message);
                        block_wait_response = false;
                        println!("unblocked...");
                    }
                }
            }
        });

        block_wait_response = false;

        for item in self.send_messages.iter() {
            while block_wait_response {}
            println!("sending... {}", item.message);

            match tx.send(OwnedMessage::Text(item.message.clone())) {
                Ok(()) => (),
                Err(e) => {
                    println!("Main Loop: {:?}", e);
                    break;
                }
            }
            match item.command.command {
                file_parser::WaitCommand::WAITMESSAGE => {
                    block_wait_response = true;
                    println!("blocking...");
                }
                file_parser::WaitCommand::WAITTIME => {
                    println!("sleeping...");
                    match item.command.wait_time {
                        None => panic!("time not defined for time cmd"),
                        Some(time_duration) => thread::sleep(time_duration),
                    }
                }
                file_parser::WaitCommand::END => {
                    println!("closing...");
                    break; // Will close ws after break
                }
            }

            // Send item
            // switch cmd
            //  Timeout:
            //      Wait for timeout
            //  NextMessage:
            //      Set flag and block until unset by receive
            //  End:
            //      Return from function
            println!("Sending msg: {}", item.message);
        }
        // Close the connection
        let _ = tx.send(OwnedMessage::Close(None));

        // We're exiting
        println!("Waiting for child threads to exit");
        let _ = send_loop.join();
        // let _ = receive_loop.join(); >> done differently now (using thread pools)

        println!("Exited");
    }
}
