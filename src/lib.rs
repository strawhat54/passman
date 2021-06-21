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

    pub fn file_check(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn authenticate(pass: &str) {}

    pub fn random() -> String {
        let str: String = (0..12).map(|_| rand::random::<u8>() as char).collect();
        str
    }

    pub fn new() -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        let secret_key = orion::aead::SecretKey::default();
        let ciphertext = orion::aead::seal(&secret_key, line.as_bytes());
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
            let ans = manager::new();
            match ans {
                Ok(i) => print!("{:?}", i),
                _ => panic!("ERROR"),
            };
            std::process::exit(0);
        }

        _ => {
            print!("INVALID!");
        }
    }
}
