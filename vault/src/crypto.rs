use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, BlockModeError, Cbc, InvalidKeyIvLength};
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Clone)]
pub struct Cipher {
    cipher: Aes256Cbc,
}

impl Cipher {
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, InvalidKeyIvLength> {
        Ok(Self {
            cipher: Aes256Cbc::new_from_slices(&key, &iv)?,
        })
    }

    pub fn encrypt(self, value: &str) -> Vec<u8> {
        self.cipher.encrypt_vec(value.as_bytes())
    }

    pub fn decrypt(self, value: Vec<u8>) -> Result<String, BlockModeError> {
        let value = &mut value.clone()[32..];
        let decrypted_ciphertext = self.cipher.decrypt(value)?;
        Ok(String::from_utf8(decrypted_ciphertext.to_vec()).unwrap())
    }
}
