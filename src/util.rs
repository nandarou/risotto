use std::io::prelude::*;
extern crate libflate;

use self::libflate::gzip::{EncodeOptions, HeaderBuilder, Os};

pub fn from_hex(hex: &str) -> Vec<u8> {
    if hex.len() % 2 != 0 {
        panic!("Is not a valid hex, length should be even");
    }
    let mut result = Vec::with_capacity(hex.len() / 2);
    let mut index = 0;
    let mut value = 0;
    for c in hex.chars() {
        index += 1;
        let byte = c.to_digit(16).unwrap() as u8;

        if index % 2 == 0 {
            value += byte;
            result.push(value);
        } else {
            value = byte * 16;
        }
    }
    result
}

pub fn to_hex(bytes: &[u8]) -> String {
    let bytes_as_hex: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    bytes_as_hex.join("")
}

#[test]
fn test_xor() {
    let a = from_hex("AB");
    let b = from_hex("CD");

    let xor = xor(a.as_slice(), b.as_slice());

    assert_eq!(xor, from_hex("66"));
}

/// bitwise XOR each byte of two arrays
/// assuming they're the same size
pub fn xor(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(op1.len());
    for i in 0..op1.len() {
        result.push(op1[i] ^ op2[i]);
    }
    result
}

#[test]
fn test_gzip() {
    let input = [0, 1, 2, 3];
    let expected = from_hex("1F8B08000000000000006360646206001386B98B04000000");
    let gzipped = gzip(&input);

    assert_eq!(expected, gzipped);
}

pub fn gzip(input: &[u8]) -> Vec<u8> {
    let header = HeaderBuilder::new().modification_time(0).os(Os::Undefined(0)).finish();
    let options = EncodeOptions::new().header(header).fixed_huffman_codes();
    let mut encoder = libflate::gzip::Encoder::with_options(Vec::new(), options).unwrap();

    let _ = encoder.write_all(&input);
    let result = encoder.finish().into_result().unwrap();
    result
}
