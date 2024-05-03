mod encryption;
mod tcp_comms;
use std::net::{TcpStream};
use std::io::{Write};
use std::str::from_utf8;



/*
This file utilizes the other modules to coordinate the whole application
*/

fn main() -> std::io::Result<()> {

    // negotiate a TCP connection with the other party
    let mut stream : &TcpStream = &tcp_comms::establish_tcp_conn().expect("TCP Connection Could Not Be Established");

    // set the stream to non-blocking so that we can poll it for incoming messages
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    // allow a separate thread to listen to stdin
    let stdin_channel = &tcp_comms::spawn_stdin_channel();

    // create a buffer to contain messages as they're received
    let mut buf: &mut Vec<u8> = &mut vec![];

    // poll the message channel and the stdin channel for sent/received messages
    loop{
        // poll for received tcp messages
        tcp_comms::poll_tcp_stream(buf, stream, &on_message_received);

        // poll for stdin messages to send
        tcp_comms::poll_stdin(stdin_channel, stream, &send_message);





    }


Ok(())
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