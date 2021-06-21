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
use serde::{Deserialize, Serialize};
use std;

mod manager {

    pub fn authenticate(pass: &str, key_location: &str) {
        let data = std::fs::read_to_string(key_location).expect("Unable to read file");
        let key = orion::auth::SecretKey::default();
        let pass = pass.as_bytes();
        let expected_tag = orion::auth::authenticate(&key, pass).unwrap();
        if orion::auth::authenticate_verify(&expected_tag, &key, &pass).is_ok() {
            println!("AUTHENTICATION SUCESSFULL !");
        } else {
            println!("WRONG PASSWORD");
            std::process::exit(0);
        }
    }

    fn store(master_key: &str, location: &str) {
        std::fs::File::create(location).expect("Failed to create init file!");
        std::fs::write(location, master_key);
    }

    pub fn new(secret_location: &str) {
        println!("Enter the masterkey for passman");
        let mut master_key = String::new();

        std::io::stdin()
            .read_line(&mut master_key)
            .unwrap()
            .to_string();

        let ans = encrypt(&mut master_key);
        let mut encrypted_master_key = String::new();
        for &val in &ans {
            encrypted_master_key.push(val as char);
        }

        store(&encrypted_master_key, &secret_location)
    }

    pub fn file_check(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn random() -> String {
        let str: String = (0..12).map(|_| rand::random::<u8>() as char).collect();
        str
    }

    fn encrypt(pass: &mut str) -> Vec<u8> {
        let secret_key = orion::aead::SecretKey::default();
        let ciphertext = orion::aead::seal(&secret_key, pass.as_bytes()).unwrap();
        ciphertext
    }

    pub fn add(name: &str, pass: &str) {}

    pub fn update(name: &str, pass: &str) {}

    pub fn remove(name: &str) {}

    pub fn get(name: &str) {}

    pub fn list() {}

    pub fn get_csv() {}

    pub fn destroy() {}
}

pub fn perform(task: &str) {
    let home: String = dirs::home_dir().unwrap().display().to_string();
    let config: String = format! {"{}{}", home,"/.passman"};
    let secret: String = format! {"{}{}", home,"/.passman_key"};

    match task {
        "new" => {
            let present = manager::file_check(&home);
            if present {
                panic!("Looks like you already have initialized passman config. Try other options or destroy the current config with `passman destroy`");
            }

            manager::new(&secret)
        }

        _ => {
            print!("INVALID!");
        }
    }
}
