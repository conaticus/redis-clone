use std::str::from_utf8;

use crate::{
    resp::{Array, BulkString, Parseable, SimpleString, CRLF, NULL},
    tcp::{TCPBuffer, CACHE},
    util::extract_first_char,
};

/// TODO(conaticus): Make error handling more elegant,
/// there are panics and expects that shouldn't be used,
/// instead we should log the error or return it to the client

/// Parses a command, the command name will be set to lowercase
fn parse_command(buffer: TCPBuffer) -> (String, Vec<String>) {
    let raw_command = from_utf8(&buffer).expect("Could not convert buffer to string");
    let (type_byte, raw_data) = extract_first_char(raw_command);
    // This must be peekable so that the array parser knows when to stop iterating
    let raw_data_split = &mut raw_data.split(CRLF).peekable();

    match type_byte {
        '+' => (
            SimpleString::deserialize(raw_data_split).to_lowercase(),
            Vec::new(),
        ),
        '$' => (
            BulkString::deserialize(raw_data_split).to_lowercase(),
            Vec::new(),
        ),
        '*' => {
            let mut args = Array::deserialize(raw_data_split);
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
            let echo_string = args.get(0).expect("Missing echo parameter");
            BulkString::serialize(echo_string.into())
        }
        "set" => set_command(args),
        "get" => get_command(args),
        _ => panic!("Invalid command name specified"),
    }
}

fn set_command(mut args: Vec<String>) -> Vec<u8> {
    // We can remove these items from the vector because we only need them here
    // If an argument is missing we will get a runtime error
    let key = args.remove(0);
    let value = args.remove(0);

    let mut cache = CACHE.lock().unwrap();
    let old_value = cache.insert(key, value);

    match old_value {
        Some(old_value) => SimpleString::serialize(old_value),
        None => SimpleString::serialize(String::from("OK")),
    }
}

fn get_command(args: Vec<String>) -> Vec<u8> {
    let key = args.get(0).expect("No key specified.");

    let cache = CACHE.lock().unwrap();
    let value = cache.get(key);

    match value {
        Some(value) => BulkString::serialize(value.clone()), // Not a fan of cloning here
        None => NULL.to_vec(),                               // Not a fan of using to_vec every time
    }
}
