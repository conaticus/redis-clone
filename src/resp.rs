use std::iter::Peekable;
use std::str::Split;

pub const CRLF: &str = "\r\n";
pub const NULL_BULK_STRING: &[u8; 5] = b"$-1\r\n";

type PeekableStringSplit<'a> = Peekable<Split<'a, &'a str>>;

pub trait Parseable<T> {
    fn serialize(data: T) -> Vec<u8>;
    fn deserialize(split: &mut PeekableStringSplit<'_>) -> T;
}

pub struct SimpleString;
impl Parseable<String> for SimpleString {
    fn serialize(data: String) -> Vec<u8> {
        format!("+{}{CRLF}", data).into_bytes()
    }

    fn deserialize(split: &mut PeekableStringSplit<'_>) -> String {
        split
            .next()
            .expect("Could not parse simple string data")
            .to_string()
    }
}

pub struct BulkString;
impl Parseable<String> for BulkString {
    fn serialize(data: String) -> Vec<u8> {
        format!("${}\r\n{}\r\n", data.len(), data).into_bytes()
    }

    fn deserialize(split: &mut PeekableStringSplit<'_>) -> String {
        split.next().expect("Could not parse bulk string length"); // Not being used for now.
        split
            .next()
            .expect("Could not parse bulk string data")
            .into()
    }
}

// NOTE(conaticus): This only supports bulk strings for now.
pub struct Array;
impl Parseable<Vec<String>> for Array {
    fn serialize(_data: Vec<String>) -> Vec<u8> {
        unimplemented!()
    }

    // Make more efficient later by using regular arrays, as a length is provided
    fn deserialize(split: &mut PeekableStringSplit<'_>) -> Vec<String> {
        split.next().expect("Could not parse array length"); // Not being used for now.

        let mut array = Vec::new();

        while let Some(&item) = split.peek() {
            if item.starts_with('\0') || item.is_empty() {
                break;
            }

            // Later we will need to support more types.
            // Note that this will pass in the type byte as well for now, which is going to lead to the length including that
            let element = BulkString::deserialize(split);
            array.push(element);
        }

        array
    }
}
