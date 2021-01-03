/* Get the contents of a file and parse it into commands */
// extern crate timespan;
extern crate chrono;

// use chrono::NaiveTime;
// use timespan::NaiveTimeSpan;
use parse_duration::parse;
use std::time::Duration;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
pub enum WaitCommand {
    WAITMESSAGE,
    WAITTIME, 
    END
}

pub struct Command {
    pub command: WaitCommand,
    pub wait_time: Option<Duration>
}

pub struct WsMessage {
    pub message: String,
    pub command: Command
}

impl WsMessage {
    pub fn new(file_string: &String) -> Vec<WsMessage>
    {
        let string_vector = file_string.lines();
        return WsMessage::create_from_vector(&string_vector.map(str::to_string).collect());
    }

    fn create_from_vector(cmds: &Vec<String>) -> Vec<WsMessage> {
        let mut msg_vector = Vec::<WsMessage>::with_capacity(cmds.len());
        for (i,item) in cmds.into_iter().enumerate() {
            // create a singular msg_cmd
            let message = WsMessage::create_from_string(item,i.try_into().unwrap());
            msg_vector.push(message);
        }
        return msg_vector;
    }

    fn create_from_string(string: &String, line: u32) -> WsMessage {
        let sv: Vec<String> = string.split_whitespace().map(str::to_string).collect();
        /* x    y --> validate that we have at least the message and command */
        assert!(sv.len() >= 2, "Unable to parse at line {}", line);
        let end = get_end(string);
        let message: String = string.chars().take(end.try_into().unwrap()).collect();
        match sv.last() {
            None => panic!("nothing in array!"),
            Some(cmd_string) => {
                let command = Command::new(cmd_string, line);
                WsMessage { message, command }
            }
        }
    }
}

/* get end of a in string resembling a  b, */
fn get_end(string: &String) -> u32
{
    let mut found_whitespace = false;
    for (i, c) in string.chars().rev().enumerate() {
        if c.is_whitespace()
        {
            found_whitespace = true;
        }
        else if found_whitespace
        {
            // return string position as final msg in idx 
            // i != string position
            return (string.len() - i).try_into().unwrap();
        }
    }
    panic!("failed to parse"); // bad software
}

impl Command {
    fn new(string: &String, line: u32) -> Command
    {
        // parse string into message command
        // can be a time of <x>h<y>m<z>s
        // can be W (wait)
        // can be E (end/finish)
        match string.as_str() {
            "W" => {
                let command = WaitCommand::WAITMESSAGE;
                Command { command, wait_time: None }
            },
            "E" => {
                let command = WaitCommand::END;
                Command { command, wait_time: None }
            },
            _ => {
                // Process hms timespan format
                let command = WaitCommand::WAITTIME;
                let wait_time = parse(string).unwrap();
                // panic!("Unable to parse command at line {}", line);
                Command { command, wait_time: Some(wait_time) }
            }
        }
    }
}

impl fmt::Display for WaitCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{:?}", self)
        // or, alternatively:
        fmt::Debug::fmt(self, f)
    }
}
