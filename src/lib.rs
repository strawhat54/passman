#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

use clipboard;
use dirs;
use orion;
use rand::{self, Rng};
use rgb::RGB8;
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
        std::fs::File::create(location).expect("Failed to create master_key file!"); // .passman_key
        std::fs::write(location, master_key); // encrypted data write
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
        std::fs::create_dir(config_location).unwrap();
    }

    pub fn file_check(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn random() -> String {
        (0..15)
            .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char) //idk reddit se mila
            .collect()
    }

    fn encrypt(pass: &str) -> String {
        let secret_key = orion::aead::SecretKey::default();
        let cipher_text = orion::aead::seal(&secret_key, pass.as_bytes()).unwrap();
        let mut cipher_string = String::new();
        for &val in &cipher_text {
            cipher_string.push(val as char);
        }
        cipher_string
    }

   pub fn add(filename: &str, pass: &str) -> std::io::Result<()> {
        let mut pwd = encrypt(pass);
        let mut file = std::fs::File::create(filename)?;
        std::fs::write(filename, pwd)?;
        Ok(())
    }

    pub fn update(name: &str, pass: &str) {}

    pub fn remove(filename: &str) -> std::io::Result<()>  {
        std::fs::remove_file(filename)?;
        Ok(())
    }

    pub fn get(name: &str) {}

    pub fn list() {}

    pub fn get_csv() {}

    pub fn destroy() {}
}

pub fn perform(task: &str) {
    let home: String = dirs::home_dir().unwrap().display().to_string();
    let config_loc: String = format! {"{}{}", home,"/.passman"};
    let secret_loc: String = format! {"{}{}", home,"/.passman_key"};

    match task {
        "new" => {
            let present = manager::file_check(&secret_loc);
            if present {
                panic!("Looks like you already have initialized passman config. Try other options or destroy the current config with `passman destroy`");
            }
            manager::new(&config_loc, &secret_loc);
            manager::authenticate("test", &secret_loc);
        }

        _ => {
            print!("INVALID!");
        }
    }
}
