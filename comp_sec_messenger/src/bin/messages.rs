use std::io;
use ring::digest::{SHA256, Context};
use ring::aead::{AES_256_GCM, UnboundKey, LessSafeKey, NONCE_LEN, Aad, Nonce};
use ring::rand::{SystemRandom, SecureRandom};

fn main() -> io::Result<()> {
    let password = "amazing".to_string();
//    let session_id = generate_random_number();
    let session_id = 1337;

    //let mut nonce: &[u8] = &[0; aead::NONCE_LEN];
    let mut message = b"epic".to_vec();
    println!("{:?}", message);
    let key = build_key_from_password(password, session_id);
//    let key = generate_random_key();
    
    encrypt_message(key.clone(), &mut message);
    println!("{:?}", message);

    decrypt_message(key.clone(), &mut message);
    println!("{:?}", message);
    
    
    //println!("{}", hex::encode(hash.as_ref()));

    Ok(())
}

// generates a random 32 bit integer
fn generate_random_number() -> i32 {
    let sys_random = SystemRandom::new();
    let mut buffer = [0u8; 4];

    // fill the buffer with random bytes
    sys_random.fill(&mut buffer).unwrap();
    i32::from_be_bytes(buffer)
}

// decrypt message & call UI to display message (need to indicate to UI who it is from)
// message: (encrypted message) + userId (1 = Alice, 2 = Bob)
pub fn handle_received_message(message: String){

    // get user id (last char of string)
    let user_id = message.chars().last().unwrap();
    
    // remove user id from message
    let mut message_without_userid = message.to_string(); // create mutable string 
    message_without_userid.pop(); // removes last char in place
    

    // let username = "Alice";
    // if user_id == 2{
    //     username = "Bob";
    // }
    // TOOD: pass in actual key 
    //let decrypted_message = decrypt_message(message_without_userid, key);
    //gui::display_message(message_without_userid, username)
}

// UI calls this method and passes message from text input field
fn handle_sent_message(message: String){
    // generate new key, store new key as curr_key & encrypt

    // call tcp::send_message()
}

// generates a random key based on system randomness
fn generate_random_key() -> LessSafeKey {
    let sys_random = SystemRandom::new();
    let mut buffer = [0u8; 32];

    sys_random.fill(&mut buffer).unwrap();
    LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &buffer).unwrap())
}

// decrypts a message in place given a key, removes appended tag
fn decrypt_message(key: LessSafeKey, message: &mut std::vec::Vec<u8>){
        let buf = &mut [0; NONCE_LEN];
        let nonce = Nonce::try_assume_unique_for_key(buf).unwrap();
        key.open_in_place(nonce, Aad::empty(), message).unwrap();
        let mlen = message.len();
        let tag_length = AES_256_GCM.tag_len();
        message.drain((mlen-tag_length)..mlen); // remove extras generated from encryption 
}

// encrypts a given message in place with the key, with tag appended to encrypted message
fn encrypt_message(key: LessSafeKey, message: &mut std::vec::Vec<u8>) {
    let nonce = Nonce::try_assume_unique_for_key(&[0; NONCE_LEN]).unwrap();
    key.seal_in_place_append_tag(nonce, Aad::empty(), message).unwrap();
}

// reads a line and returns the message length and trimmed string
// fn process_input(handle: &mut StdinLock) -> (i32, String) {
//     let mut buffer = String::new();
//     // read the line and the message length
//     let message_length = handle.read_line(&mut buffer);

//     // trim whitespace
//     let input = buffer.trim().to_string();
    
//     (message_length.unwrap().try_into().unwrap(), input)
// }

// builds a key from a password and random challenge
fn build_key_from_password(password: String, session_id: i32) -> LessSafeKey {
    let mut hasher = Context::new(&SHA256);
    hasher.update(password.as_bytes());
    hasher.update(&session_id.to_be_bytes());

    let hash = hasher.finish();

    //println!("{:x}", hash.as_ref());
//    aead::UnboundKey::new(&aead::AES_256_GCM, hash.as_ref()).unwrap()
    LessSafeKey::new(UnboundKey::new(&AES_256_GCM, hash.as_ref()).unwrap())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let key = build_key_from_password(password, session_id);
        let password = "amazing".to_string();
        let mut message = b"epic".to_vec();
        let session_id = 1337;

        let expected_encryption = [174, 222, 161, 240, 9, 17, 198, 22, 200, 49, 248, 150, 134, 233, 242, 130, 53, 153, 5, 10];
        encrypt_message(key.clone(), &mut message);
        assert_eq!(&expected_encryption, &message, "\nExpected\n{:?}\nFound\n{:?}", 
    }
}
*/
