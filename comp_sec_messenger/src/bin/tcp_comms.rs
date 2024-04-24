use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{Read, Write, self};
use std::str::from_utf8;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread};



fn main() -> std::io::Result<()> {
    // no clue how to negotiate changing IPs without a dns server
    const OTHER_IP : [u8; 4] = [127,0,0,1];
    const MY_IP : [u8; 4] = [127,0,0,1];
    const PORT : u16= 8080;

    // target other ip
    let addrs = [SocketAddr::from((OTHER_IP, PORT))];

    // establish a connection
    if let Ok(stream) = TcpStream::connect(&addrs[..]) {
        println!("Opposite Party is online, connecting");
        return Ok(conn_established(stream)?)
    }else{
        println!("Opposite Party is not logged on, creating TCP Listener");
        let listener = TcpListener::bind(SocketAddr::from((MY_IP, PORT)))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            // initial connection established, switch to non-blocking and listening for input and messages sequentially
            return Ok(conn_established(stream?)?)
        }
        Ok(())
    }
}

fn conn_established(mut stream: TcpStream) -> std::io::Result<()>{
    println!("Connected to opposite party");
    // set the stream to non-blocking so that we can poll it for incoming messages
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let stdin_channel = spawn_stdin_channel();
    let mut buf = vec![];
    loop {
        // poll stdin for new messages to send
        match stdin_channel.try_recv() {
            Ok(key) => send_message(&key, &stream),
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => {break}
        }

        // poll the stream for new messages to be received
        match stream.read_to_end(&mut buf) {
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(_e) => panic!("{}", "encountered IO error: {e}"),
        };
        if buf.len() > 0{
            receive_messages(&mut buf)
        }
    }

    Ok(())
}

fn receive_messages(messages : &mut Vec<u8>){
    let text = from_utf8(messages).unwrap();
    println!("Received Message: {}", text);
    (*messages).clear();
}

fn send_message(msg : &str, mut stream: &TcpStream){
    let _result = stream.write(msg.as_bytes());
}

// credit to https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}