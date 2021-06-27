#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

use argonautica::{Hasher, Verifier};
use clipboard;
use dirs;
use orion::{aead, auth};
use rand;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub hash: String,
    // created: cration time
    // date: last update time
}

pub fn ask(query: &str) -> String {
    print!("{}: ", query);
    io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    answer
}

pub fn authenticate(pass: &str, key_location: &std::path::PathBuf) -> bool {
    let master_hash = fs::read_to_string(key_location).unwrap();
    let mut verifier = Verifier::default();
    verifier
        .with_hash(&master_hash)
        .with_password(pass)
        .verify()
        .unwrap()
}

fn store(master_key: &str, location: &str) {
    File::create(location).expect("Failed to create master_key file!"); // .passman_key
    fs::write(location, master_key); // encrypted data write
}

pub fn new() -> String {
    let master_key = ask("Please enter master Key");
    encrypt(&master_key)
}

pub fn encrypt(pass: &str) -> String {
    let mut hasher = Hasher::default();
    hasher.opt_out_of_secret_key(true);
    let hash = hasher
        .with_password(pass)
        .hash()
        .unwrap();
    hash
}

pub fn init_check(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn random() -> String {
    (0..15)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char) //idk reddit se mila
        .collect()
}

pub fn add(filename: &str, pass: &str) -> std::io::Result<()> {
    let mut pwd = encrypt(pass);
    let mut file = File::create(filename)?;
    fs::write(filename, pwd)?;
    Ok(())
}

pub fn update(name: &str, pass: &str) {}

pub fn remove(config_dir: &str, secret_key: &str) {
    fs::remove_file(secret_key);
    fs::remove_dir_all(config_dir);
}

pub fn get(name: &str) {}

pub fn list() {}

pub fn get_csv() {}

pub fn destroy() {}
