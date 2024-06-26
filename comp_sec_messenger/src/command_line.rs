mod encryption;
mod tcp_comms;
use std::net::{TcpStream};
use std::io::{Write};
use std::str::from_utf8;
use ring::aead::{LessSafeKey, UnboundKey, AES_256_GCM};

/*
This file utilizes the other modules to coordinate the whole application
*/

fn main() -> std::io::Result<()> {
    // generate a random number to seed encryption
    // this number will be used if it is the second instance of the app started, or overwritten if it is the first
    let mut seed: i32 = encryption::generate_random_number();
    // shared password between Alice and Bob
    let password: &str = "password";

    // create a buffer to contain messages as they're received
    let buf: &mut Vec<u8> = &mut vec![];

    // negotiate a TCP connection with the other party
    let stream : &TcpStream = &tcp_comms::establish_tcp_conn(&mut seed, buf).expect("TCP Connection Could Not Be Established");

    // allow a separate thread to listen to stdin
    let stdin_channel = &tcp_comms::spawn_stdin_channel();

    // generate a key based on the password and the now shared seed
    let mut key = encryption::build_key_from_password(password.to_owned(), seed);

    // poll the message channel and the stdin channel for sent/received messages
    loop{
        // poll for received tcp messages
        let omr = |msg: &mut Vec<u8>|{
            on_message_received(msg, &mut key, &seed);
        };
        tcp_comms::poll_tcp_stream(buf, stream, omr);

        // // poll for stdin messages to send
        let sm = |msg: &String, stream: &TcpStream|{
            send_message(msg, stream, &mut key, &seed);
        };
        tcp_comms::poll_stdin(stdin_channel, stream, sm);
    }
}

// called whenever a message is received...
fn on_message_received(messages : &mut Vec<u8>, k: &mut LessSafeKey, _seed: &i32){
    // decrypt the message
//    println!("{:?}", messages);
    encryption::decrypt_message(k.clone(), messages);

    // remove the last 32 bytes for the key
    let mlen = messages.len();
    let buf = messages.as_slice()[mlen-32..].to_vec();
    messages.drain(mlen-32..);
//    println!("{:?}", buf);

    // display message
    let text = from_utf8(messages).unwrap();
    println!("Received Message: {}", text);

    // generate a new key based on the last 32 bits of the ciphertext
    *k = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &buf).unwrap());

    // clear the buffer
    (*messages).clear();
}

fn send_message(msg: &String, mut stream: &TcpStream, k: &mut LessSafeKey, _seed: &i32){
    // encrypt the message
    let message : String = (*msg).clone();
    // generate a 32 bit random key
    let buf = encryption::generate_random_key();

    let ciphertext: &mut Vec<u8> = &mut message.clone().into_bytes();
    
    // append the key to the plaintext before encryption
    ciphertext.append(&mut buf.to_vec());
    encryption::encrypt_message(k.clone(), ciphertext);

    // generate a new key based on the random generator
    *k = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &buf).unwrap());

    // write the message to the TCP Stream
    let _ = stream.write(ciphertext);
}
