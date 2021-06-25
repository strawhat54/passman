#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

use clipboard;
use dirs;
use orion::{aead, auth};
use rand;
use rgb::RGB8;
use std::fs::{self, File};

pub fn authenticate(pass: &str, key_location: &str) {
    let data = fs::read_to_string(key_location).expect("Unable to read file");
    let key = auth::SecretKey::default();
    let pass = pass.as_bytes();
    let expected_tag = auth::authenticate(&key, pass).unwrap();
    if auth::authenticate_verify(&expected_tag, &key, &pass).is_ok() {
        println!("AUTHENTICATION SUCESSFULL !");
    } else {
        println!("WRONG PASSWORD");
        std::process::exit(0);
    }
}

fn store(master_key: &str, location: &str) {
    File::create(location).expect("Failed to create master_key file!"); // .passman_key
    fs::write(location, master_key); // encrypted data write
}

pub fn new(config_location: &str, secret_location: &str) {
    println!("Enter the masterkey for passman");
    let mut master_key = String::new();

    std::io::stdin()
        .read_line(&mut master_key)
        .unwrap()
        .to_string();

    let encrypted_master_key = encrypt(&mut master_key);
    store(&encrypted_master_key, &secret_location);
    fs::create_dir(config_location).unwrap();
}

pub fn file_check(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn random() -> String {
    (0..15)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char) //idk reddit se mila
        .collect()
}

fn encrypt(pass: &str) -> String {
    let secret_key = aead::SecretKey::default();
    let cipher_text = aead::seal(&secret_key, pass.as_bytes()).unwrap();
    let mut cipher_string = String::new();
    for &val in &cipher_text {
        cipher_string.push(val as char);
    }
    cipher_string
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
