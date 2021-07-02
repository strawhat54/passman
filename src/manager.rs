#![allow(unused_imports, unused_must_use, dead_code)]

use super::auth::{decrypt_item, encrypt_item, encrypt_master};
use ansi_term::Color::{Red, Yellow, Green};
use clipboard;
use dirs;
use rand;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::{self, prelude::*, Read};

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
    print!("{}: ", Yellow.paint(query));
    io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    answer.trim().to_string()
}

pub fn new() -> String {
    let master_key = ask("Please enter master Key");
    encrypt_master(&master_key)
}

pub fn create_new_item(name: &str, master: &str) -> Item {
    let desc = ask("Desc");
    let pass = ask("Password");
    let hash = encrypt_item(master, &pass);
    Item {
        name: name.to_string(),
        desc: desc,
        hash: hash,
    }
}

pub fn update(item: &Item, master: &str) -> Item {
    let new_hash = encrypt_item(master, &ask("Please enter new password"));
    let name = item.name.clone();
    let desc = item.desc.clone();
    Item {
        name: name,
        desc: desc,
        hash: new_hash,
    }
}

pub fn _random() -> String {
    (0..15)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
        .collect()
}
