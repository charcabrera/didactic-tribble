// Key Manager Module: key generation, store curr key, expiration (optional)
mod config;
mod key_manager {
    let password = config::PASSWORD;
    let session_id = config::SESSION_ID;
    pub static key = "None"; // may need to change this
    
pub fn print_key(){
    println!("{}", hex::encode(key.as_ref()));
}

// TODO: generate new key each time a message is sent
pub fn generate_next_key(){
    let prev_key = key;

    // if we are generating the first key, build from password
    if(prev_key == "None"){
        key = build_key_from_password(password, session_id);
        return;
    }

    // else, generate new key
    
}

pub fn build_key_from_password(password: String, session_id: i32) -> Digest {
    let mut hasher = Context::new(&SHA256);
    hasher.update(password.as_bytes());
    hasher.update(&session_id.to_be_bytes());

    let hash = hasher.finish();

    //println!("{:x}", hash.as_ref());

    hash
}
}