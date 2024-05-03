mod encryption;
mod tcp_comms;
use std::net::{TcpStream};
use std::io::{Write};
use std::str::from_utf8;



/*
This file utilizes the other modules to coordinate the whole application
*/

fn main() -> std::io::Result<()> {
    // shared password between Bob and Alice
    let password = "password";

    // generate a random number to seed encryption IF this instance is the first instance started
    let mut seed: i32 = encryption::generate_random_number();

    // create a buffer to contain messages as they're received
    let mut buf: &mut Vec<u8> = &mut vec![];

    // negotiate a TCP connection with the other party
    let mut stream : &TcpStream = &tcp_comms::establish_tcp_conn(&mut seed, buf).expect("TCP Connection Could Not Be Established");

    // allow a separate thread to listen to stdin
    let stdin_channel = &tcp_comms::spawn_stdin_channel();

    // poll the message channel and the stdin channel for sent/received messages
    loop{
        // poll for received tcp messages
        tcp_comms::poll_tcp_stream(buf, stream, &on_message_received);

        // poll for stdin messages to send
        tcp_comms::poll_stdin(stdin_channel, stream, &send_message);
    }
}

// called whenever a message is received...
fn on_message_received(messages : &mut Vec<u8>){
    // display message
    let text = from_utf8(messages).unwrap();
    println!("Received Message: {}", text);

    // clear the buffer
    (*messages).clear();
}

fn send_message(msg: &String, mut stream: &TcpStream){
    // write the message to the TCP Stream
    stream.write(msg.as_bytes());
}