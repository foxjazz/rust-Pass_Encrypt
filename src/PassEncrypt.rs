
use aes::Aes128;
use aes::cipher::{
     BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};


use std::ops::Add;

trait Encrypt{
    fn encrypt(&self, password : String) -> Self;
}
trait Decrypt{
    fn decrypt(&self, password: String) -> Result<String,AesError>;
}

impl Encrypt for String{
    fn encrypt( &self, password: String) -> Self {
        let key_string = get_key(password);
        let key = GenericArray::clone_from_slice(key_string.as_bytes());
        let cipher = Aes128::new(&key);
        let verify: String ="verify".to_string();
        let pt = self.add(&verify);
        let mut block = GenericArray::clone_from_slice(pt.as_bytes());

        cipher.encrypt_block(&mut block);
        let s = block.to_vec();
        std::str::from_utf8(&s).unwrap().to_string()
    }
}
impl Decrypt for String{
    fn decrypt(&self, password: String) -> Result<String,AesError> {
        let key_string = get_key(password);
        let key = GenericArray::clone_from_slice(key_string.as_bytes());
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::clone_from_slice(self.as_bytes());

        cipher.decrypt_block(&mut block);
        let s = block.to_vec();
        let mut result = std::str::from_utf8(&s).unwrap();
        let check = "verify" as &str;
        if result.ends_with(check) {
            let str_len = result.len() -6;
            let res = result.get(..str_len);
//            result.truncate(result.len() - 6);

            if res.is_some(){
                return Ok(res.unwrap().to_string())
            }

            let truck = AesError::new("aes error");
            Err(truck)

        }
        else {
            let truck = AesError::new("incorrect password");
            Err(truck)
        }
    }
}

fn get_key(password: String) -> String {

    let mut pw = password.clone();
    while pw.len() < 16{
        pw += password.as_str();
    }
    pw
}
struct AesError{
    error:String
}
impl AesError{
    fn new(error: &str) -> AesError{
        AesError{error: error.to_string()}
    }
}
