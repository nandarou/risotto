extern crate openssl;

use util;

#[test]
fn test_ansi_example() {
    let expected_mac = util::from_hex("9CCC7817");

    let pek = util::from_hex("042666B49184CFA368DE9628D0397BC9");
    let data = util::from_hex("343031323334353637383930394439383700000000000000");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}

fn des_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let cipher = openssl::symm::Cipher::des_ecb();
    let encrypted = openssl::symm::encrypt(cipher, key, None, data);

    encrypted
}

#[test]
fn test_decrypt() {
    let key = util::from_hex("68DE9628D03984C9");
    let encrypted = util::from_hex("C5BC8EC1621EF5EE");
    let expected = util::from_hex("F720DC72F93339DF");

    let decrypted = des_decrypt(&key, &encrypted).unwrap();

    assert_eq!(expected, decrypted);
}

fn des_decrypt(key: &[u8], encrypted: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
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
    println!("iv = {:?}", util::to_hex(&iv));

    iv = des_decrypt(&right_mut, &iv).unwrap();
    iv.split_off(8);
    iv = des_encrypt(&left_mut, &iv).unwrap();

    let mut m = [0; 4];
    for i in 0..4 {
        m[i] = iv[i];
    }
    m
}
