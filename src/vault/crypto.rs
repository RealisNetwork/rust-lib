use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use hex_literal::hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypted_mnemonic(mnemonic: &str, _key: &str, _iv: &str) -> Vec<u8> {
    let target = mnemonic;

    let key = hex!("1cde61f165f5867d3de6ed6554770680df3b47d0d5ca5537c379569124ab4bec");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let ciphertext = cipher.encrypt_vec(target.as_bytes());
    ciphertext
}

pub fn decrypted_mnemonic(_key: String, _iv: String, mnemonic: Vec<u8>) -> String {
    // let key_format = format!("{:?}", key);
    let key = hex!("1cde61f165f5867d3de6ed6554770680df3b47d0d5ca5537c379569124ab4bec");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");

    // re-create cipher mode instance and decrypt the message
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = mnemonic.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
    let mnemonic_string = String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
    mnemonic_string
}
