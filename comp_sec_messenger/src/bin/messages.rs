// hash generation, encryption & decryption of messsages
mod config;
mod gui;
mod key_manager;

use std::io::{self, BufRead, StdinLock};
use ring::digest::{SHA256, Context, SHA256_OUTPUT_LEN, Digest};

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

    // let password = "amazing".to_string();
    // let session_id = 13;
    let password = config::PASSWORD;
    let session_id = config::SESSION_ID;

    let hash = key_manger::build_key_from_password(password, session_id);

    // generate K1

    // store curr key as private class variable so it's accessible to decrypt()..? idk how this works in rust :)

    println!("{}", hex::encode(hash.as_ref()));

    Ok(())
}
// helper for reading test messages from stdin
fn process_input(handle: &mut StdinLock) -> (i32, String) {
    let mut buffer = String::new();
    // read the line and the message length
    let message_length = handle.read_line(&mut buffer);

    // trim whitespace
    let input = buffer.trim().to_string();
    
    (message_length.unwrap().try_into().unwrap(), input)
}

fn handle_sent_message(message: String){
    // generate new encryption key & store as currKey, which is accessible to both parties 
    key_manager::generate_next_key();

    // encrypt message using new key 

}

fn encrypt_message(message: String){
    let key = key_manager::key;
}

// decrypt message & call UI to display message (need to indicate to UI who it is from)
// message: (encrypted message) + userId (1 = Alice, 2 = Bob)
fn handle_received_message(message: String){

    // get user id (last char of string)
    let user_id = message.chars().last().unwrap();
    
    // remove user id from message
    let mut message_without_userid = message.to_string(); // create mutable string 
    message_without_userid.pop(); // removes last char in place

    let username = "Alice";
    if(user_id == 2){
        username = "Bob";
    }
    let decrypted_message = decrypt_message(message_without_userid);
    //gui::display_message(message_without_userid, username)
}

// fixed length key + random number 
fn decrypt_message(message: String){
    // retreive current key from key manager
    key = key_manager::key;
    // decryption packages for rust.. looking tomorrow
}




