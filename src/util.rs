use std::io::prelude::*;
extern crate libflate;

use self::libflate::gzip::{EncodeOptions, HeaderBuilder, Os};

use IsoMsg;

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
        if op2.len() <= i {
            result.push(op1[i]);
        } else {
            result.push(op1[i] ^ op2[i]);
        }
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

// temp
pub fn build_echo(hwid: i32) -> String {
    let echo = format!("<isomsg><f id=\"0\" v='0800'/><isomsg id=\"48\"><f id=\"1\" \
                        v=\"123b0000000000{}\"/><f id=\"2\" v=\"16.00.00\"/></isomsg><f \
                        id=\"70\" v=\"301\"/></isomsg>\n",
                       hwid);
    echo.to_string()
}

#[test]
fn test_build_purchase() {
    let purchase_xml = build_purchase();
    println!("Purchase:\n{}", purchase_xml);
    // assert_eq!(1, 2);
}

pub fn build_purchase() -> String {
    let mut purchase = IsoMsg::new_with_mti("0200");
    purchase.set_string(3, "001000");
    purchase.set_string(4, "1400");
    purchase.set_string(7, "0731174750");
    purchase.set_string(11, "946812");
    purchase.set_string(12, "20170731174750");
    purchase.set_string(15, "310717");
    purchase.set_string(18, "5973");
    purchase.set_string(22, "90");
    purchase.set_string(25, "00");
    purchase.set_string(35, "5602549990000160=18011201006500009380");
    purchase.set_string(40, "120");
    purchase.set_string(41, "2");

    let mut f42 = IsoMsg::sub_field(42);
    f42.set_string(1, "769");
    f42.set_string(2, "");
    f42.set_string(3, "");
    f42.set_string(4, "769");
    purchase.set_field(42, f42);

    let mut f43 = IsoMsg::sub_field(43);
    f43.set_string(1, "Paul's Services");
    f43.set_string(2, "Nook");
    f43.set_string(3, "036");
    f43.set_string(4, "7306");
    purchase.set_field(43, f43);

    let mut f48 = IsoMsg::sub_field(48);
    f48.set_string(1, "424b000000000002");
    f48.set_string(2, "17.02.00");
    purchase.set_field(48, f48);

    let mut f61 = IsoMsg::sub_field(61);
    f61.set_string(1, "0");
    f61.set_string(2, "1");
    f61.set_string(3, "0");
    f61.set_string(4, "0");
    f61.set_string(5, "0");
    f61.set_string(6, "0");
    f61.set_string(7, "0");
    f61.set_string(8, "0");
    f61.set_string(10, "0");
    f61.set_string(11, "5");
    f61.set_string(12, "0");
    purchase.set_field(61, f61);

    purchase.pack()
}
