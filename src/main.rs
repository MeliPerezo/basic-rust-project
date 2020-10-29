use ferris_says::say;
use std::io::{stdout, BufWriter, Error};
use std:: io;

fn main() {
    let mut key_input = String::new();
    let mut message_input = String::new();
    let mut key_input2 = String::new();

    println!("--- Simulation Start ---");
    println!("Please input your message: ");
    match io::stdin().read_line(&mut message_input) {
        Ok(_) => {
            println!("Your message: {}", message_input);
        }
        _ => {}
    }
    println!("Please input your key: ");
    match io::stdin().read_line(&mut key_input) {
        Ok(_) => {
            println!("Your key: {}", key_input);
        }
        _ => {}
    }

    let bytes_message = convert_binary(message_input.as_str());
    let key = convert_binary(key_input.as_str());
    let encrypt_message = xor(bytes_message, key);
    let encrypt_message_xor = convert_binary(encrypt_message.as_str());
    let decrypt_message = xor(encrypt_message_xor, key);

    println!("This is the ciphertext: {:?}", encrypt_message);
    println!("The message: {:?}", decrypt_message);
    println!("--- End of Simulation ---");
}
fn convert_binary(message: &str) -> &[u8]{
    let message_bytes = message.as_bytes();
    return message_bytes;
}
fn xor(message: &[u8], key: &[u8]) -> String{
    let mut xor: Vec<u8> = Vec::new();
        for n in 0.. message.len(){
            xor.push(message[n] ^ key[n]);
    }
    return String::from_utf8(xor).unwrap();
}
