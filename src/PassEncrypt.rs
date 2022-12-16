
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

pub struct Pass_Encrypt {

}
/*fn generate_hash(password: &str) -> Vec<u8> {
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
}*/
impl Pass_Encrypt{
    /*pub fn new (password: &str) -> Pass_Encrypt {
        //let mut hasher = Sha256::digest::new();
//        let key = generate_hash(password) ;
        let key = Sha256::digest(password.as_bytes());
        let iv = [0; 16];
        Pass_Encrypt {key, iv}
    }*/

    fn get_key(password: String) -> String {

        let mut pw = password.clone();
        while (pw.len() < 16){
            pw += password.as_str();
        }
            pw
    }
    pub fn enc( plain_text : String, password: String) -> String {
        let key_string = Pass_Encrypt::get_key(password);
        let key = GenericArray::clone_from_slice(key_string.as_bytes());
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::clone_from_slice(plain_text.as_bytes());

        cipher.encrypt_block(&mut block);
        let s = block.to_vec();
        std::str::from_utf8(&s).unwrap().to_string()

    }
   /* pub fn dec(&self, cipher_text : String) -> Result<String,ParseIntError> {

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
    }*/
}
