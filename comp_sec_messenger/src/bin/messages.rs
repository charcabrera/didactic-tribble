use std::io::{self, BufRead, StdinLock};
use ring::digest;
//{SHA256, Context, SHA256_OUTPUT_LEN, Digest};
use ring::aead;
//{AES_256_GCM, UnboundKey, LessSafeKey, Nonce};

fn main() -> io::Result<()> {
    /*
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let (message_length, input) = process_input(&mut handle);
        if message_length == 0 || input == "exit" {
            break;
        }
        println!("{message_length}: {input}");
    }
    */

    let password = "amazing".to_string();
    let session_id = 13;

    //let mut nonce: &[u8] = &[0; aead::NONCE_LEN];
    let mut message = b"epic".to_vec();
    println!("{:?}", message);
    let key = build_key_from_password(password, session_id);
    
    encrypt_message(key.clone(), &mut message);
    println!("{:?}", message);
    let buf = &mut [0; aead::NONCE_LEN];
    let nonce = aead::Nonce::try_assume_unique_for_key(buf).unwrap();
    key.open_in_place(nonce, aead::Aad::empty(), &mut message).unwrap();
    let mlen = message.len();
    let tag_length = aead::AES_256_GCM.tag_len();
    message.drain((mlen-tag_length)..mlen);
    println!("{}", std::str::from_utf8(&message).unwrap());
    //println!("{}", hex::encode(hash.as_ref()));

    Ok(())
}

// encrypts a given message in place with the key, with tag appended to encrypted message
fn encrypt_message(key: aead::LessSafeKey, message: &mut std::vec::Vec<u8>) {
    let nonce = aead::Nonce::try_assume_unique_for_key(&[0; aead::NONCE_LEN]).unwrap();
    key.seal_in_place_append_tag(nonce, aead::Aad::empty(), message).unwrap();
}

// reads a line and returns the message length and trimmed string
fn process_input(handle: &mut StdinLock) -> (i32, String) {
    let mut buffer = String::new();
    // read the line and the message length
    let message_length = handle.read_line(&mut buffer);

    // trim whitespace
    let input = buffer.trim().to_string();
    
    (message_length.unwrap().try_into().unwrap(), input)
}

// builds a key from a password and random challenge
fn build_key_from_password(password: String, session_id: i32) -> aead::LessSafeKey {
    let mut hasher = digest::Context::new(&digest::SHA256);
    hasher.update(password.as_bytes());
    hasher.update(&session_id.to_be_bytes());

    let hash = hasher.finish();

    //println!("{:x}", hash.as_ref());
//    aead::UnboundKey::new(&aead::AES_256_GCM, hash.as_ref()).unwrap()
    aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, hash.as_ref()).unwrap())
}


