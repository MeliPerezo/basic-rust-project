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

    if message.len() > key.len() {
        String::from("The key must be equal than the message.".into());
    }

    if !key
        .chars()
        .all(|chr| (chr.is_alphabetic() && chr.is_uppercase()) || chr == ' ')
    {
        String::from("The key source was malformed.".into());
    }

    if !message
        .chars()
        .all(|chr| (chr.is_alphabetic() && chr.is_uppercase()) || chr == ' ')
    {
        String::from("The plaintext was malformed.".into());
    }

    return  String::from("");
}

fn decrypt(ciphertext: String, key: String) -> String {

    if ciphertext.len() > key.len() {
        String::from("The key must be equal or longer than the ciphertext.".into());
    }
    if !key
        .chars()
        .all(|chr| (chr.is_alphabetic() && chr.is_uppercase()) || chr == ' ')
    {
        String::from("The key source was malformed.".into());
    }

    if !ciphertext
        .chars()
        .all(|chr| (chr.is_alphabetic() && chr.is_uppercase()) || chr == ' ')
    {
        String::from("The ciphertext was malformed.".into());
    }

    return  String::from("");
}


