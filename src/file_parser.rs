/* Get the contents of a file and parse it into commands */
extern crate timespan;
extern crate chrono;

use chrono::NaiveTime;
use timespan::NaiveTimeSpan;

pub enum WaitCommand {
    WAITMESSAGE,
    WAITTIME, 
    END
}

struct Command {
    command: WaitCommand,
    wait_time: Option<NaiveTimeSpan>
}

pub struct WsMessage {
    message: String,
    command: Command
}

impl WsMessage {
    pub fn new(file_string: &String) -> Vec<WsMessage>
    {
        let string_vector = file_string.lines();
        return WsMessage::create_from_vector(string_vector.map(str::to_string).collect());
    }

    fn create_from_vector(cmds: Vec<String>) -> Vec<WsMessage> {
        let msg_vector = Vec::<WsMessage>::with_capacity(cmds.len());
        for (x,y) in cmds.enumerate() {
            // create a singular msg_cmd
            let message = WsMessage::create_from_string(x,y);
            msg_vector .push(message);
        }
        return msg_vector;
    }

    fn create_from_string(string: &String, line: u32) -> WsMessage {
        let sv: Vec<&str> = string.split_whitespace();
        /* x    y --> validate that we have at least the message and command */
        assert!(sv.le() >= 2, "Unable to parse at line {}", line);
        let end = get_end(string);
        let message = string.chars().take(end).collect();
        let command = Command::new(sv.last(), line);
        WsMessage { message, command }
    }
}

/* get end of a in string resembling a  b, */
fn get_end(string: &String) -> u32
{
    let found_whitespace = false;
    for (c,i) in string.chars().rev() {
        if c.is_whitespace()
        {
            found_whitespace = true;
        }
        else if found_whitespace
        {
            // return string position as final msg in idx 
            return i;
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
        match string {
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
                let wait_time = NaiveTime::parse_from_str(string, "%hh%mm%ss");
                println!("Parsed time input: {}", wait_time);
                // panic!("Unable to parse command at line {}", line);
                Command { command, wait_time: Some(wait_time) }
            }
        }
    }
}
