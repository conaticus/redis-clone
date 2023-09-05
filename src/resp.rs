use crate::util::extract_first_char;

const CRLF: &str = "\r\n";

pub trait Parseable<T> {
    fn serialize(data: T) -> Vec<u8>;
    fn deserialize(raw_data: String) -> T;
}

pub struct SimpleString;
impl Parseable<String> for SimpleString {
    fn serialize(data: String) -> Vec<u8> {
        format!("+{}{CRLF}", data).into()
    }

    fn deserialize(raw_data: String) -> String {
        let mut split = raw_data.split(CRLF);
        split
            .next()
            .expect("Could not parse simple string data")
            .to_string()
    }
}

pub struct BulkString;
impl Parseable<String> for BulkString {
    fn serialize(data: String) -> Vec<u8> {
        format!("${}\r\n{}\r\n", data.len(), data).into()
    }

    fn deserialize(raw_data: String) -> String {
        let mut split = raw_data.split(CRLF);
        split.next().expect("Could not parse bulk string length");
        split
            .next()
            .expect("Could not parse bulk string data")
            .into()
    }
}

// NOTE(conaticus): This only supports bulk strings for now.
pub struct Array;
impl Parseable<Vec<String>> for Array {
    fn serialize(data: Vec<String>) -> Vec<u8> {
        todo!()
    }

    // Make more efficient later by using regular arrays, as a length is provided
    fn deserialize(raw_data: String) -> Vec<String> {
        let mut split = raw_data.split(CRLF).peekable();
        let mut current_idx = 0;

        // We add two to account for the CRLF characters
        current_idx += split.next().expect("Could not parse array length").len() + 2;

        let mut array = Vec::new();
        let mut chars = raw_data.chars();

        while chars.nth(current_idx).unwrap() != '\0' {
            let start_idx = current_idx;

            // We call .next() twice for the two expected CRLFs in a bulk string
            // When more types are supported, we should map out the CLRFs required for each type
            // And loop the .next() accordingly
            // That could also be used in the deserialize functions so it has multiple uses
            current_idx += split.next().unwrap().len() + 2;
            current_idx += split.next().unwrap().len() + 2;

            let element_raw = raw_data[start_idx..current_idx].to_string();
            array.push(BulkString::deserialize(element_raw))
        }

        array
    }
}
