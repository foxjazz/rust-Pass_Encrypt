mod pass_encrypt;
use pass_encrypt::Encrypt;
use pass_encrypt::Decrypt;

fn main() {
    let stbe = "string to be endrypted".to_string().encrypt("password".to_string());
    let plain_text = stbe.decrypt("password".to_string());
    if plain_text.is_ok() {
        println!("plain text: {}", plain_text.unwrap());
    }
    else {
        println!("error: {}", plain_text.err().unwrap());
    }
}
