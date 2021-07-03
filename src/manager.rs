#![allow(unused_imports, unused_must_use, dead_code)]

use super::auth::{decrypt_item, encrypt_item, encrypt_master};
use ansi_term::Color::{Green, Purple, Red, Yellow};
use clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};
use dirs;
use rand;
use rpassword;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::{self, prelude::*, Read};
use std::thread;
use std::time::Duration;

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
pub fn paste_to_clipboard(value: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value);
    println!(
        "{}",
        Green.paint("Password copied to clipbpoard for 30 seconds!")
    );
    thread::sleep(Duration::from_secs(30));
    println!("{}", Red.paint("Time's Up!"));
}

pub fn pass_ask(query: &str) -> String {
    let pass = rpassword::prompt_password_stdout(&format!("{}: ", Purple.paint(query))).unwrap();
    pass
}

pub fn ask(query: &str) -> String {
    print!("{}: ", Yellow.paint(query));
    io::stdout().flush();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    answer.trim().to_string()
}

pub fn new() -> String {
    let master_key = pass_ask("Please enter master Key");
    encrypt_master(&master_key)
}

pub fn create_new_item(name: &str, master: &str) -> Item {
    let desc = ask("Desc");
    let pass = pass_ask("Password");
    let hash = encrypt_item(master, &pass);
    Item {
        name: name.to_string(),
        desc: desc,
        hash: hash,
    }
}

pub fn update(item: &Item, master: &str) -> Item {
    let new_hash = encrypt_item(master, &pass_ask("Please enter new password"));
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
