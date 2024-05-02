// Key Manager Module: key generation, store curr key, expiration (optional)
mod config;
mod key_manager {
    let password = config::PASSWORD;
    let session_id = config::SESSION_ID;
    pub static key = build_key_from_password(password, sesison_id);

// TODO: generating new key each time a message is sent
pub fn generate_next_key(prev_key){
    // gen random number

    // encrypt (old key + rand #)
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