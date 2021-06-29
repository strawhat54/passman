#![allow(unused_imports, dead_code)]

use argonautica::{Hasher, Verifier};
use clipboard;
use dirs;
use rand;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use std::fmt;
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

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nDesc: {}", self.name, self.desc)
    }
}

pub fn ask(query: &str) -> String {
    print!("{}: ", query);
    io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    answer.trim().to_string()
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

pub fn new() -> String {
    let master_key = ask("Please enter master Key");
    encrypt(&master_key)
}

pub fn create_new_item(name: &str) -> Item {
    let desc = ask("Desc");
    let pass = ask("Password");
    let hash = encrypt(&pass);
    Item {
        name: name.to_string(),
        desc: desc,
        hash: hash,
    }
}

pub fn update(item: &Item) -> Item {
    let new_hash = encrypt(&ask("Please enter new password"));
    let name = item.name.clone();
    let desc = item.desc.clone();
    Item {
        name: name,
        desc: desc,
        hash: new_hash,
    }
}

pub fn encrypt(pass: &str) -> String {
    let mut hasher = Hasher::default();
    hasher.opt_out_of_secret_key(true);
    let test = hasher.with_password(pass).hash_raw().unwrap();
    print!("encrypted: {:?}", test);
    hasher.with_password(pass).hash().unwrap()
}

pub fn _random() -> String {
    (0..15)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char) //idk reddit se mila
        .collect()
}
