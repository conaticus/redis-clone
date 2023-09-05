use std::str::from_utf8;

use crate::{
    resp::{Array, BulkString, Parseable, SimpleString},
    tcp::TCPBuffer,
    util::extract_first_char,
};

// TODO(conaticus): Make error handling more elegant in this file, there are panics and expects that shouldn't be used, instead we should log the error or return it to the client

/// Parses a command, the command name will be set to lowercase
fn parse_command(buffer: TCPBuffer) -> (String, Vec<String>) {
    let raw_command = from_utf8(&buffer).expect("Could not convert buffer to string");
    let (type_byte, raw_data) = extract_first_char(raw_command);

    match type_byte {
        '+' => (
            SimpleString::deserialize(raw_data).to_lowercase(),
            Vec::new(),
        ),
        '$' => (BulkString::deserialize(raw_data).to_lowercase(), Vec::new()),
        '*' => {
            let mut args = Array::deserialize(raw_data);
            let command_name = args.remove(0).to_lowercase();
            (command_name, args)
        }
        _ => panic!("No valid type byte was provided"),
    }
}

pub fn execute_command(buffer: TCPBuffer) -> Vec<u8> {
    let (command_name, args) = parse_command(buffer);

    match command_name.as_str() {
        "ping" => SimpleString::serialize("PONG".into()),
        "echo" => {
            let echo_string = args.get(0).unwrap();
            BulkString::serialize(echo_string.into())
        }
        _ => panic!("Invalid command name specified"),
    }
}
