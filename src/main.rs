mod PassEncrypt;

use crate::PassEncrypt::Pass_Encrypt;

fn main() {
    let stbe = "string to be endrypted".to_string();
    let enc = Pass_Encrypt::new("password");
    let cypher = enc.enc(stbe);
    let result = enc.dec(cypher);
    if Some(result) {
        println!("result: {}", result);
    }
    else {
        println!("error");
    }
}
