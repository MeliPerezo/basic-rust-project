use ferris_says::say;
use std::io::{stdout,  BufWriter};

fn main() {
    let standard_output = stdout();
    let message: String = String::from("We are learning cryptography");
    let width: usize = message.chars().count();

    let mut writer = BufWriter::new(standard_output.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();

    let ciphertext =  encrypt(String::from("AAAFFFF"), String::from("AAAFFFF"));
}

fn encrypt(message: String, key: String) -> String {
    let message_bytes = message.as_bytes();
}

fn decrypt(ciphertext: String, key: String) -> String {

}


