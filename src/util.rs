use std::io::prelude::*;
extern crate libflate;
extern crate flate2;
extern crate deflate;
extern crate gzip_header;

use self::libflate::lz77::{DefaultLz77Encoder, CompressionLevel};
use self::libflate::gzip::{EncodeOptions, HeaderBuilder, Os};

use self::flate2::{GzBuilder, Compression};
use self::flate2::write::GzEncoder;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::prelude::*;
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

fn gzip_libflate(input: &[u8]) -> Vec<u8> {
    let header = HeaderBuilder::new().modification_time(0).os(Os::Undefined(0)).finish();
    // let options = EncodeOptions::with_lz77(DefaultLz77Encoder::new()).header(header).fixed_huffman_codes();
    let lz77 = DefaultLz77Encoder::new();
    let options = EncodeOptions::with_lz77(lz77).header(header).fixed_huffman_codes();
    let mut encoder = libflate::gzip::Encoder::with_options(Vec::new(), options).unwrap();

    let _ = encoder.write_all(&input);
    let result = encoder.finish().into_result().unwrap();
    result
}

fn gzip_flate2(input: &[u8]) -> Vec<u8> {
    let mut e = GzBuilder::new().write(Vec::new(), Compression::Default);
    // let mut e = GzEncoder::new(Vec::new(), Compression::Default);
    let _ = e.write(input);
    let mut compressed_bytes = e.finish().unwrap();
    compressed_bytes[9] = 0; // hack

    compressed_bytes
}

fn gzip_deflate(input: &[u8]) -> Vec<u8> {
    let mut compressed_data = deflate::deflate_bytes_gzip_conf(input,
                                                               deflate::Compression::Default,
                                                               gzip_header::GzBuilder::new());
    // let mut compressed_data = deflate::deflate_bytes(input);
    compressed_data[9] = 0; // hack

    compressed_data
}

fn gzip_using_gzip(input: &[u8]) -> Vec<u8> {
    let process = match Command::new("gzip")
        .arg("-c")
        .arg("-n")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(p) => p,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    match process.stdin.unwrap().write_all(input) {
        Ok(_) => {}
        Err(e) => println!("oops {}", e),
    }

    let mut v = vec![];
    match process.stdout.unwrap().read_to_end(&mut v) {
        Err(why) => panic!("couldn't read  stdout: {}", why.description()),
        Ok(_) => {} //println!("gzip responded with:\n{}", to_hex(&v)),
    }

    v[9] = 0;
    v
}

pub fn gzip(input: &[u8]) -> Vec<u8> {
    // gzip_deflate(input)
    // gzip_libflate(input)
    gzip_using_gzip(input)
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
    let purchase_xml = build_purchase().pack();
    println!("Purchase:\n{}", purchase_xml);
    // assert_eq!(1, 2);
}

pub fn build_purchase() -> IsoMsg {
    let mut purchase = IsoMsg::new_with_mti("0200");
    purchase.set_string(3, "001000");
    purchase.set_string(4, "2234");
    purchase.set_string(7, "0731174750");
    purchase.set_string(11, "946811");
    purchase.set_string(12, "20170815174750");
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
    f48.set_string(1, "424B1000000001");
    f48.set_string(2, "17.21.00");
    purchase.set_field(48, f48);

    purchase.set_binary(52, "8F8B18FE0FBD5434");
    purchase.set_binary(53, "FFFF9876543210E00001");

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

    purchase
}
