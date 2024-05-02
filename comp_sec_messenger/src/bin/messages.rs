// hash generation, encryption & decryption of messsages
mod config;
mod gui;
mod key_manager;
mod tcp_comms;

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
    
    // send via TCP
}
// TODO: encrypting using key stored in key manager
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

// fixed length key (256) + random number 
fn decrypt_message(message: String){
    // retreive current key from key manager
    key = key_manager::key;
    // decryption packages for rust.. looking tomorrow
}




