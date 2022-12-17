

    use aes::Aes128;
    use aes::cipher::{
        BlockEncrypt, BlockDecrypt, KeyInit,
        generic_array::GenericArray,
    };


   // use std::ops::Add;
    pub trait Encrypt {
        fn encrypt(self, password: String) -> String;
    }

    pub trait Decrypt {
        fn decrypt(self, password: String) -> Result<String, Self::Err>;
    }

    impl Encrypt for String {
        fn encrypt(self, password: String) -> String {
            let key_string = get_key(password);
            let key = GenericArray::clone_from_slice(key_string.as_bytes());
            let cipher = Aes128::new(&key);
            let verify: &str = "verify";
            let pt = self + verify;
//             let pt = self.append(&verify);
            let mut block = GenericArray::clone_from_slice(pt.as_bytes());

            cipher.encrypt_block(&mut block);
            let s = block.to_vec();
            std::str::from_utf8(&s).unwrap().to_string()
        }
    }

    impl Decrypt for String {
        fn decrypt(self, password: String) -> Result<String, Self::Err> {
            let key_string = get_key(password);
            let key = GenericArray::clone_from_slice(key_string.as_bytes());
            let cipher = Aes128::new(&key);
            let mut block = GenericArray::clone_from_slice(self.as_bytes());

            cipher.decrypt_block(&mut block);
            let s = block.to_vec();
            let mut result = std::str::from_utf8(&s).unwrap();
            let check = "verify" as &str;
            if result.ends_with(check) {
                let str_len = result.len() - 6;
                let res = result.get(..str_len);
//            result.truncate(result.len() - 6);

                if res.is_some() {
                    return Ok(res.unwrap().to_string())
                }

                let truck = Err("aes error");
                Err(truck)
            } else {
                let truck = Err("incorrect password");
                Err(truck)
            }
        }
    }

    fn get_key(password: String) -> String {
        let mut pw = password.clone();
        while pw.len() < 16 {
            pw += password.as_str();
        }
        pw
    }
