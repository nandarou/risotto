extern crate openssl;

use util;

fn des_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = openssl::symm::Cipher::des_ecb();
    let encrypted = openssl::symm::encrypt(cipher, key, None, data);

    encrypted
}

pub fn des_decrypt(key: &[u8], encrypted: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = openssl::symm::Cipher::des_cbc();
    let iv = vec![0; 8];
    let mut c = openssl::symm::Crypter::new(cipher, openssl::symm::Mode::Decrypt, key, Some(&iv))
        .unwrap();
    // TODO patch rust-openssl to expose padding option:
    c.pad(false);
    let mut out = vec![0; encrypted.len() + cipher.block_size()];
    let count = try!(c.update(encrypted, &mut out));
    let rest = try!(c.finalize(&mut out[count..]));
    out.truncate(count + rest);
    Ok(out)
}

/// Generate MAC using DUKPT ISO 9797
pub fn generate_mac(pek: &[u8], data: &[u8]) -> [u8; 4] {
    // let key = pek ^ 000000000000FF00000000000000FF00;
    let (left, right) = pek.split_at(8);
    let mut left_mut = left.to_vec();
    left_mut[6] = left_mut[6] ^ 0xFF;
    let mut right_mut = right.to_vec();
    right_mut[6] = right_mut[6] ^ 0xFF;

    let mut iv = vec![0; 8];
    for chunks in data.chunks(8) {
        iv = util::xor(&iv, chunks);
        iv = des_encrypt(&left_mut, &iv).unwrap();
        // TODO figure out why encrypt returns 16 bytes, instead of 8
        iv.split_off(8);
    }

    iv = des_decrypt(&right_mut, &iv).unwrap();
    iv.split_off(8);
    iv = des_encrypt(&left_mut, &iv).unwrap();

    let mut m = [0; 4];
    for i in 0..4 {
        m[i] = iv[i];
    }
    m
}

#[test]
fn test_generate_ansi_pin_block() {
    let pin = vec![1, 2, 3, 4];
    let pan = "5163610055067910";

    let expected = "041202EFFAAF986E";
    let clear_pin_block = generate_ansi_pin_block(pin.as_slice(), pan);
    let actual = util::to_hex(&clear_pin_block);

    assert_eq!(expected, actual);
}

fn generate_ansi_pin_block(pin: &[u8], pan: &str) -> [u8; 8] {
    if pin.len() > 12 {
        panic!("Can't create pin block. Pin is too large");
    }

    let mut pin_block = [0xF; 16];
    pin_block[0] = 0;
    pin_block[1] = pin.len() as u8;
    let mut i = 2;
    for pin_digit in pin.iter() {
        pin_block[i] = *pin_digit;
        i = i + 1;
    }

    let mut pan_bytes = Vec::with_capacity(pan.len());
    for c in pan.chars() {
        let byte = c.to_digit(16).unwrap() as u8;
        pan_bytes.push(byte);
    }

    i = 4;
    for j in pan.len() - 13..pan.len() - 1 {
        pin_block[i] = pin_block[i] ^ pan_bytes[j];
        i = i + 1;
    }

    let mut r = [0; 8];
    for j in 0..pin_block.len() {
        if j % 2 == 0 {
            r[j / 2] = 16 * pin_block[j];
        } else {
            r[j / 2] = r[j / 2] + pin_block[j];
        }
    }
    r
}

pub fn encrypt_ansi_pin_block(pek: &mut [u8], pin: &[u8], pan: &str) -> [u8; 8] {
    let clear_pin_block = generate_ansi_pin_block(pin, pan);
    // let key = pek ^ 00000000000000FF00000000000000FF;
    let (left, right) = pek.split_at(8);
    let mut left = left.to_vec();
    left[7] = left[7] ^ 0xFF;
    let mut right = right.to_vec();
    right[7] = right[7] ^ 0xFF;

    let iv = des_encrypt(&left, &clear_pin_block).unwrap();
    let iv = des_decrypt(&right, &iv).unwrap();
    let iv = des_encrypt(&left, &iv).unwrap();
    let mut r = [0; 8];
    for i in 0..8 {
        r[i] = iv[i];
    }
    r
}
