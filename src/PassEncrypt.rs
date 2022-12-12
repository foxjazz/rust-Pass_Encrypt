use aes::Aes256;
use block_modes::{BlockMode, Cfb};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
use std::env;
use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use sha2::Sha256;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::num::ParseIntError;
type Aes128ECfb = Cfb<Aes128, Pkcs7>;
pub struct Pass_Encrypt {
    // ...
    // Path: src\monitor\monitor.rs
    iv: [i32;16]  ,
    key: Vec<u8>,
    // ...
}
fn generate_hash(password: &str) -> Vec<u8> {
    // Create a new hasher
    let mut hasher = DefaultHasher::new();

    // Hash the password and retrieve the resulting hash value
    password.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert the hash value to a vector of bytes
    let mut hash_bytes: Vec<u8> = Vec::new();
    for b in hash.to_le_bytes().iter() {
        hash_bytes.push(*b);
    }

    // Return the vector of bytes
    hash_bytes
}
impl Pass_Encrypt{
    pub fn new (password: &str) -> Pass_Encrypt {
        //let mut hasher = Sha256::digest::new();
//        let key = generate_hash(password) ;
        let key = Sha256::digest(password.as_bytes());
        let iv = [0; 16];
        Pass_Encrypt {key, iv}

    }
    pub fn aes256(&self,plaintext: String) -> String {
        let plaintext = plaintext + &"verify".to_string();
        let cipher = Aes256::new(&self.key);
        let mut encryptor = Cbc::new_var(&cipher, &self.iv).unwrap();
        let mut buffer = plaintext.to_vec();
        encryptor.encrypt(&mut buffer, plaintext.len() + 16, Pkcs7);
        buffer.to_string()
    }

    pub fn enc(&self, plain_text : String) -> String {
        let verify = plain_text + &"verify".to_string();

        let cipher = Aes128ECfb::new_from_slices(&self.key, &self.iv).unwrap();
        let pos = verify.len();
        let mut buffer = [0u8; 128];
        buffer[..pos].copy_from_slice(verify.as_bytes());
        let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
        ciphertext.to_string()
    }
    pub fn dec(&self, cipher_text : String) -> Result<String,ParseIntError> {

        let cipher = Aes128ECfb::new_from_slices(&self.key, &self.iv).unwrap();
        let mut buf = cipher_text.to_vec();
        let v_result = cipher.decrypt(&mut buf).unwrap();
        let result = v_result.to_string();
        if (result.ends_with("verify")) {
            let plain_text = result.truncate(result.len() - 6);
            Ok(plain_text)
        }
        else {
            Err(E::new("error"))
        }
    }
}
