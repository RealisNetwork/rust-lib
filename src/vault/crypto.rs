use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypted_mnemonic(mnemonic: &str, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let target = mnemonic;

    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let ciphertext = cipher.encrypt_vec(target.as_bytes());
    ciphertext
}

pub fn decrypted_mnemonic(key: &[u8], iv: &[u8], mnemonic: Vec<u8>) -> String {
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let mnemonic = &mut mnemonic.clone()[32..];
    let decrypted_ciphertext = cipher.decrypt(mnemonic).unwrap();
    let mnemonic_string = String::from_utf8(decrypted_ciphertext.to_vec()).unwrap();
    mnemonic_string
}
