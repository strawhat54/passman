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

    pub fn check(location: &str) {
        let data = std::fs::read_to_string(location).expect("Unable to read file");
        let key = orion::auth::SecretKey::default();

        let msg = "test".as_bytes();
        let expected_tag = orion::auth::authenticate(&key, msg).unwrap();

        assert!(orion::auth::authenticate_verify(&expected_tag, &key, &msg).is_ok());
    }

    pub fn init(master_key: &str, location: &str) {
        std::fs::File::create(location).expect("Failed to create init file!");
        std::fs::write(location, master_key);
    }

    pub fn file_check(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn authenticate(pass: &str) {}

    pub fn random() -> String {
        let str: String = (0..12).map(|_| rand::random::<u8>() as char).collect();
        str
    }

    pub fn get_secret(pass: &mut str) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
        let secret_key = orion::aead::SecretKey::default();
        let ciphertext = orion::aead::seal(&secret_key, pass.as_bytes());
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
    let mut home = match dirs::home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("No home folder found!"),
    };
    home.push_str("/.passman");

    match task {
        "new" => {
            let present = manager::file_check(&home);
            if present {
                panic!("Looks like you already have initialized passman config. Try other options or destroy the current config with `passman destroy`");
            }

            println!("Enter the masterkey for passman");
            let mut master_key = String::new();

            std::io::stdin()
                .read_line(&mut master_key)
                .unwrap()
                .to_string();

            let ans = manager::get_secret(&mut master_key);
            let mut encrypted_master_key = String::new();
            match ans {
                Ok(i) => {
                    for &val in &i {
                        encrypted_master_key.push(val as char);
                    }
                    manager::init(&encrypted_master_key, &home);
                    manager::check(&home);
                }

                _ => panic!("Error encrypting the masterkey!"),
            };
            std::process::exit(0);
        }

        _ => {
            print!("INVALID!");
        }
    }
}
