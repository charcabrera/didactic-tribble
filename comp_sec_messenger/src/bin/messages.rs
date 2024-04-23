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

    let password = "amazing".to_string();
    let session_id = 13;

    let hash = build_key_from_password(password, session_id);
    println!("{}", hex::encode(hash.as_ref()));

    Ok(())
}

fn process_input(handle: &mut StdinLock) -> (i32, String) {
    let mut buffer = String::new();
    // read the line and the message length
    let message_length = handle.read_line(&mut buffer);

    // trim whitespace
    let input = buffer.trim().to_string();
    
    (message_length.unwrap().try_into().unwrap(), input)
}

fn build_key_from_password(password: String, session_id: i32) -> Digest {
    let mut hasher = Context::new(&SHA256);
    hasher.update(password.as_bytes());
    hasher.update(&session_id.to_be_bytes());

    let hash = hasher.finish();

    //println!("{:x}", hash.as_ref());

    hash
}
