
// This file provides methods to transform the entries to the format they use on disk
use openssl::symm::{decrypt, encrypt, Cipher};
extern crate base64;

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

    return base64::encode_config(&ciphertext, base64::URL_SAFE);
}

//from encrypted to clear
pub fn retransform_entry(enc_params: &EncryptionParams, entry: &str) -> String {
    let cipher = Cipher::aes_256_cbc();
   
    let ciphertext = base64::decode_config(entry, base64::URL_SAFE).unwrap();

    let result = decrypt(
        cipher, 
        enc_params.key, 
        Some(enc_params.iv), 
        ciphertext.as_slice(),
    ).unwrap();

    return std::str::from_utf8(result.as_slice()).unwrap().to_owned()
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