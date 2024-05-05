
use ring::digest::{SHA256, Context};
use ring::aead::{AES_256_GCM, UnboundKey, LessSafeKey, NONCE_LEN, Aad, Nonce};
use ring::rand::{SystemRandom, SecureRandom};

/*
This file contains a module of functions necessary to implement perfect forward security
*/

// generates a 32 byte array based on system randomness
pub fn generate_random_key() -> [u8; 32] {
    let sys_random = SystemRandom::new();
    let mut buffer = [0u8; 32];

    sys_random.fill(&mut buffer).unwrap();
    buffer
}

// generates a random number using a source of entropy cound in the operating system.
pub fn generate_random_number() -> i32 {
    let sys_random = SystemRandom::new();
    let mut buffer = [0u8; 4];

    // fill the buffer with random bytes
    sys_random.fill(&mut buffer).unwrap();
    i32::from_be_bytes(buffer)
}


// generates a 32 byte array based on system randomness
pub fn generate_random_key() -> [u8; 32] {
    let sys_random = SystemRandom::new();
    let mut buffer = [0u8; 32];

    sys_random.fill(&mut buffer).unwrap();
    buffer
}

// decrypts a message in place with the key, with tage removed from the encrypted message
pub fn decrypt_message(key: LessSafeKey, message: &mut std::vec::Vec<u8>){
        let buf = &mut [0; NONCE_LEN];
        let nonce = Nonce::try_assume_unique_for_key(buf).unwrap();
        key.open_in_place(nonce, Aad::empty(), message).unwrap();
        let mlen = message.len();
        let tag_length = AES_256_GCM.tag_len();
        message.drain((mlen-tag_length)..mlen); // remove extras generated from encryption 
}

// encrypts a given message in place with the key, with tag appended to encrypted message
pub fn encrypt_message(key: LessSafeKey, message: &mut std::vec::Vec<u8>) {
    let nonce = Nonce::try_assume_unique_for_key(&[0; NONCE_LEN]).unwrap();
    key.seal_in_place_append_tag(nonce, Aad::empty(), message).unwrap();
}

// builds a key from a password and random challenge
pub fn build_key_from_password(password: String, session_id: i32) -> LessSafeKey {
    let mut hasher = Context::new(&SHA256);
    hasher.update(password.as_bytes());
    hasher.update(&session_id.to_be_bytes());

    let hash = hasher.finish();
    LessSafeKey::new(UnboundKey::new(&AES_256_GCM, hash.as_ref()).unwrap())
}

// empty main function so the project compiles...
fn main(){println!("run \"main\" to start the program")}


