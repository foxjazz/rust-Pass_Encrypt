
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

trait Encrypt{
    fn encrypt(&self, password : String) -> String;
}
trait Decrypt{
    fn decrypt(&self, password: String) -> Result<String,E>;
}

impl Encrypt for String{
    fn encrypt( &self, password: String) -> String {
        let key_string = Pass_Encrypt::get_key(password);
        let key = GenericArray::clone_from_slice(key_string.as_bytes());
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::clone_from_slice((plain_text + "verify".to_string()).as_bytes());

        cipher.encrypt_block(&mut block);
        let s = block.to_vec();
        std::str::from_utf8(&s).unwrap().to_string()
    }
}
impl Decrypt for String{
    fn decrypt(&self, password: String) -> Result<String,E> {
        let key_string = Pass_Encrypt::get_key(password);
        let key = GenericArray::clone_from_slice(key_string.as_bytes());
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::clone_from_slice(cypher_text.as_bytes());

        cipher.decrypt_block(&mut block);
        let s = block.to_vec();
        let mut result = std::str::from_utf8(&s).unwrap().to_string();
        if (result.ends_with("verify")) {
            let plain_text = result.truncate(result.len() - 6).to_string();
            Ok(plain_text)
        }
        else {
            Err(E::new("error"))
        }
    }
}

fn get_key(password: String) -> String {

    let mut pw = password.clone();
    while (pw.len() < 16){
        pw += password.as_str();
    }
    pw
}


