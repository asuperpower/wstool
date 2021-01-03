/* Websocket tooling */

extern crate chrono;

use std::env;
use std::fs;

mod file_parser;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let input_args =  InputArgs::new(&cli_args);

    println!("Filename: {}, File args: {}, Websocket path: {}",
        input_args.file_name, input_args.file_args, input_args.websocket_path);

    println!("Reading filename {}", input_args.file_name);
    let contents = fs::read_to_string(input_args.file_name)
        .expect("Unable to read file");

    println!("With text: \n{}", contents);

    let parsed_file = file_parser::WsMessage::new(&contents);

    for item in parsed_file.iter() {
        println!("Message: {}", item.message);
        println!("Command: {}", item.command.command.to_string());
    }
}

struct InputArgs {
    file_name: String,
    file_args: String,
    websocket_path: String,
}

impl InputArgs {
    fn new(args: &[String]) -> InputArgs {
        let file_name = args[1].clone();
        let file_args = args[2].clone();
        let websocket_path = args[3].clone();
        
        InputArgs { file_name, file_args, websocket_path }
    }
}

