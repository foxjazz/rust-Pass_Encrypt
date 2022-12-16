
mod PassEncrypt;
use crate::PassEncrypt::Pass_Encrypt;

fn main() {
    let stbe = "string to be endrypted".to_string();

    let cypher_txt = Pass_Encrypt::enc(stbe, "password".to_string());
    println!("cipher_txt: {}", cypher_txt);
}
