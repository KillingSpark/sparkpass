use std::fs::File;
use std::io::Read;
extern crate base64;

pub fn generate_passwd(length: usize) -> String {
    let mut f = File::open("/dev/urandom").unwrap();
    let mut vbuf = vec![0u8;length];
    let buf = vbuf.as_mut_slice();

    f.read_exact(buf).expect("Couldnt read from /dev/urandom");

    let passwd = base64::encode_config(buf, base64::URL_SAFE_NO_PAD);

    return passwd[..length].to_owned();
}