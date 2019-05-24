
// This file provides methods to transform the entries to the format they use on disk
use openssl::symm::{decrypt, encrypt, Cipher};
extern crate base64;
use std::io::Read;

pub struct EncryptionParams<'a> {
    pub key: &'a [u8],
    pub iv: &'a [u8],
}

//from clear to encrypted
pub fn transform_entry(enc_params: &EncryptionParams, entry: &str)-> String {
    let cipher = Cipher::aes_256_cbc();

    let ciphertext = encrypt(
        cipher,
        enc_params.key,
        Some(enc_params.iv),
        entry.as_bytes(),
    ).unwrap();

    let mac_cipher = Cipher::aes_256_cbc();
    let ciphertext_mac = encrypt(
        mac_cipher,
        enc_params.key,
        Some(enc_params.iv),
        ciphertext.as_slice(),
    ).unwrap();

    let cipher_part = base64::encode_config(&ciphertext, base64::URL_SAFE);
    let mac_part = base64::encode_config(&ciphertext_mac, base64::URL_SAFE);

    let mut content = String::from(cipher_part);
    content.push('~'); // + does not appear in url safe base64 but is till url safe
    content.push_str(mac_part.as_str());

    return content;
}

fn get_random_iv() -> Vec<u8> {
    let mut f = std::fs::File::open("/dev/urandom").unwrap();
    let mut vbuf = vec![0u8;16];
    let buf = vbuf.as_mut_slice();

    f.read_exact(buf).expect("Couldn't read from /dev/urandom");
    
    vbuf
}

pub fn transform_entry_with_random_iv(enc_params: &EncryptionParams, entry: &str)-> String {
    let cipher = Cipher::aes_256_cbc();

    let iv = get_random_iv();

    let ciphertext = encrypt(
        cipher,
        enc_params.key,
        Some(iv.as_slice()),
        entry.as_bytes(),
    ).unwrap();

    let mac_cipher = Cipher::aes_256_cbc();
    let ciphertext_mac = encrypt(
        mac_cipher,
        enc_params.key,
        Some(iv.as_slice()),
        ciphertext.as_slice(),
    ).unwrap();

    let cipher_part = base64::encode_config(&ciphertext, base64::URL_SAFE);
    let mac_part = base64::encode_config(&ciphertext_mac, base64::URL_SAFE);
    let iv_part = base64::encode_config(&iv.as_slice(), base64::URL_SAFE);

    let mut content = String::from(cipher_part);
    content.push('~'); // + does not appear in url safe base64 but is till url safe
    content.push_str(mac_part.as_str());
    content.push('~'); // + does not appear in url safe base64 but is till url safe
    content.push_str(iv_part.as_str());

    return content;
}

//from encrypted to clear
pub fn retransform_entry(enc_params: &EncryptionParams, entry: &str) -> Result<String, String> {
    let cipher = Cipher::aes_256_cbc();

    let parts: Vec<&str> = entry.split("~").collect();
    if parts.len() != 2 {
        return Err("Malformed entry, needs entry and mac".to_owned());
    }

    let cipher_part = parts[0];
    let mac_part = parts[1];
   
    let ciphertext = base64::decode_config(cipher_part, base64::URL_SAFE).unwrap();
    let mactext = base64::decode_config(mac_part, base64::URL_SAFE).unwrap();

    let mac_cipher = Cipher::aes_256_cbc();
    let ciphertext_mac = encrypt(
        mac_cipher,
        enc_params.key,
        Some(enc_params.iv),
        ciphertext.as_slice(),
    ).unwrap();

    match mactext.as_slice().cmp(ciphertext_mac.as_slice()) {
        std::cmp::Ordering::Equal => {
            //nothing
        },
        _ => return Err("Mac did not match with calculated mac. Key is probably wrong or data was corrupted".to_owned()),
    }

    let result = decrypt(
        cipher, 
        enc_params.key, 
        Some(enc_params.iv), 
        ciphertext.as_slice(),
    );

    return match result {
        Ok(r) => Ok(std::str::from_utf8(r.as_slice()).unwrap().to_owned()),
        Err(_) => Err("Could not decrypt. Is the key correct?".to_owned()),
    }
}

pub fn retransform_entry_with_random_iv(enc_params: &EncryptionParams, entry: &str) -> Result<String, String> {
    let cipher = Cipher::aes_256_cbc();

    let parts: Vec<&str> = entry.split("~").collect();
    if parts.len() != 3 {
        return Err("Malformed entry, needs entry, mac, and iv".to_owned());
    }

    let cipher_part = parts[0];
    let mac_part = parts[1];
    let iv_part = parts[2];
   
    let ciphertext = base64::decode_config(cipher_part, base64::URL_SAFE).unwrap();
    let mactext = base64::decode_config(mac_part, base64::URL_SAFE).unwrap();
    let ivtext = base64::decode_config(iv_part, base64::URL_SAFE).unwrap();

    let mac_cipher = Cipher::aes_256_cbc();
    let ciphertext_mac = encrypt(
        mac_cipher,
        enc_params.key,
        Some(ivtext.as_slice()),
        ciphertext.as_slice(),
    ).unwrap();

    match mactext.as_slice().cmp(ciphertext_mac.as_slice()) {
        std::cmp::Ordering::Equal => {
            //happy path
        },
        _ => return Err("Mac did not match with calculated mac. Key is probably wrong or data was corrupted".to_owned()),
    }

    let result = decrypt(
        cipher, 
        enc_params.key, 
        Some(ivtext.as_slice()), 
        ciphertext.as_slice(),
    );

    return match result {
        Ok(r) => Ok(std::str::from_utf8(r.as_slice()).unwrap().to_owned()),
        Err(_) => Err("Could not decrypt. Is the key correct?".to_owned()),
    }
}

//pub fn retransform_path(enc_params: &EncryptionParams, path: &str) -> Vec<String> {
//    let mut vec = Vec::new();
//
//    for part in path.split("/") {
//        vec.push(retransform_entry(enc_params, part))
//    }
//
//    return vec
//}

pub fn transform_path(enc_params: &EncryptionParams, path: &str) -> Vec<String> {
    let mut vec = Vec::new();

    for part in path.split("/") {
        vec.push(transform_entry(enc_params, part));
    }

    return vec
}