use std::{
    collections::VecDeque,
    str::from_utf8,
    time::{Duration, SystemTime},
};

use crate::{
    resp::{Array, BulkString, Parseable, SimpleString, CRLF, NULL_BULK_STRING},
    tcp::{CacheValue, TCPBuffer, CACHE},
    util::extract_first_char,
};

/// TODO(conaticus): Make error handling more elegant,
/// there are panics and expects that shouldn't be used,
/// instead we should log the error or return it to the client

/// Parses a command, the command name will be set to lowercase
fn parse_command(buffer: TCPBuffer) -> (String, VecDeque<String>) {
    let raw_command = from_utf8(&buffer).expect("Could not convert buffer to string");
    let (type_byte, raw_data) = extract_first_char(raw_command);
    // This must be peekable so that the array parser knows when to stop iterating
    let raw_data_split = &mut raw_data.split(CRLF).peekable();

    match type_byte {
        '+' => (
            SimpleString::deserialize(raw_data_split).to_lowercase(),
            VecDeque::new(),
        ),
        '$' => (
            BulkString::deserialize(raw_data_split).to_lowercase(),
            VecDeque::new(),
        ),
        '*' => {
            let args = Array::deserialize(raw_data_split);
            let mut args_queue = VecDeque::from(args);

            let command_name = args_queue
                .pop_front()
                .expect("Missing command name")
                .to_lowercase();

            (command_name, args_queue)
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

fn set_command(mut args: VecDeque<String>) -> Vec<u8> {
    // Make as accurate as possible as we could lose time the more lines we run
    let current_time = SystemTime::now();

    // We can remove these items from the vector because we only need them here
    // If an argument is missing we will get a runtime error
    let key = args.pop_front().expect("No key provided");
    let value = args.pop_front().expect("No value provided");

    let mut cache_value = CacheValue::from(value);
    while let Some(command_option) = args.pop_front() {
        // Only supports PX option for now
        if command_option != "PX" {
            break;
        }

        let ms_raw = args
            .pop_front()
            .expect("Missing millseconds option for set command");
        let ms = ms_raw
            .parse::<u64>()
            .expect("Could not parse millseconds as uint64");
        let duration = Duration::from_millis(ms);

        let expire_time = current_time + duration;
        cache_value.expires_at = Some(expire_time);
    }

    let mut cache = CACHE.lock().unwrap();
    let old_value = cache.insert(key, cache_value);

    match old_value {
        Some(old_value) => SimpleString::serialize(old_value.value),
        None => SimpleString::serialize(String::from("OK")),
    }
}

fn get_command(args: VecDeque<String>) -> Vec<u8> {
    let key = args.get(0).expect("No key specified");

    let mut cache = CACHE.lock().unwrap();
    let value = cache.get(key);

    match value {
        Some(cache_value) => {
            if let Some(expire_time) = cache_value.expires_at {
                if SystemTime::now() > expire_time {
                    cache.remove(key);
                    return NULL_BULK_STRING.to_vec();
                }
            }

            BulkString::serialize(cache_value.value.clone()) // Not a fan of cloning here
        }
        None => NULL_BULK_STRING.to_vec(), // Not a fan of using to_vec every time
    }
}
