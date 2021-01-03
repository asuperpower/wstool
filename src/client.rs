/* really the business logic for our websocket client */
/* hence, application layer and TIGHTLY coupled to our file parser */

use crate::file_parser;

pub struct Client {
    websocket_path: String,
    send_messages: Vec<file_parser::WsMessage>
}

/* we open a websocket client based on parameters and iterate through */
impl Client {
    pub fn new(websocket_path: String, send_messages: Vec<file_parser::WsMessage>) -> Client
    {
        Client { websocket_path, send_messages }
    }

    pub fn start(&self)
    {
        println!("Opening: {}", self.websocket_path);
        for item in self.send_messages.iter() {
            println!("Sending msg: {}", item.message);
        }
    }
}
