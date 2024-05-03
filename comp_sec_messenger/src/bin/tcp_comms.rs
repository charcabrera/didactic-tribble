use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write, self, ErrorKind};
use std::str::from_utf8;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread};

/*
This file contains a module of functions useful for creating and managing a TCP connection between Alice and Bob
*/

pub fn establish_tcp_conn(seed: &mut i32, buf: &mut Vec<u8>) -> Option<TcpStream> {
    // no clue how to negotiate changing IPs without a dns server
    const OTHER_IP : [u8; 4] = [127,0,0,1];
    const MY_IP : [u8; 4] = [127,0,0,1];
    const PORT : u16= 8080;

    // target other ip
    let addrs = [SocketAddr::from((OTHER_IP, PORT))];

    // establish a connection
    if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
        // connect to opposite party, second instance

        // send the seed
        //stream.set_nonblocking(true).expect("set_nonblocking call failed");
        let test = stream.write_all(&seed.to_be_bytes());
        println!("{:?}", test);

        return Some(stream);
    }else{
        println!("Opposite Party is not logged on, creating TCP Listener");
        let listener = TcpListener::bind(SocketAddr::from((MY_IP, PORT))).ok()?;

        // accept connections and process them serially
        for stream_res in listener.incoming() {
            // initial connection established
            let mut stream = stream_res.unwrap();
            stream.set_nonblocking(true).expect("set_nonblocking call failed");
            
            // poll until the opposite party sends the seed
            let mut seed_received: bool = false;
            fn pass(_: &mut Vec<u8>){}

            while !seed_received{
                poll_tcp_stream(buf, &stream, &pass);
                if buf.len() > 0 {
                    // process the seed
                    let input: &[u8] = buf.as_slice();
                    let num: &[u8; 4] = &<&[u8] as TryInto<[u8; 4]>>::try_into(&input[0..4]).unwrap();
                    *seed = i32::from_be_bytes(*num);

                    // clear the buffer, and exit the loop
                    (*buf).clear();
                    seed_received = true
                }
            }

            return Some(stream)
        }
        None
    }
}


pub fn poll_stdin(stdin_channel: &Receiver<String>, mut stream: &TcpStream, send_msg: &dyn Fn(&String, &TcpStream)){
    // poll stdin for new messages to send
    match stdin_channel.try_recv() {
        Ok(mess) => send_msg(&mess, &stream),
        Err(_) => {}
    }
}

pub fn poll_tcp_stream(buf: &mut Vec<u8>, mut stream: &TcpStream, mess_received: &dyn Fn(&mut Vec<u8>)) {
    // poll the stream for new messages to be received
    match stream.read_to_end(buf) {
        Ok(_) => {},
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
        Err(_) => {}
    };
    if buf.len() > 0{
        mess_received(buf);
    }
}

// credit to https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

// empty main function so the project compiles
fn main(){}