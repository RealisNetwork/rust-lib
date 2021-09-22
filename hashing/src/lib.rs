use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use hex_literal::hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// pub fn encrypted_mnemonic(mnemonic: &str, key: &str, iv: &str) -> Vec<u8> {
//     let target = mnemonic;
//
//     let key = hex!(key);
//     let iv = hex!(iv);
//     let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
//
//     let ciphertext = cipher.encrypt_vec(target.as_bytes());
//     ciphertext
// }
//
// pub fn decrypted_mnemonic(key: String, iv: String, mnemonic: Vec<u8>) -> String {
//     let key_format = format!("{:?}", key);
//     let key = hex!(key_format);
//     let iv = hex!(iv);
//
//     // re-create cipher mode instance and decrypt the message
//     let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
//     let mut buf = mnemonic.to_vec();
//     let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
//     let mnemonic_string =
// String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
//     mnemonic_string
// }
