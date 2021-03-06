use super::auth::{encrypt_item, encrypt_master};
use ansi_term::Color::{Green, Purple, Red, Yellow};
use clipboard::{ClipboardContext, ClipboardProvider};
use rand;
use rpassword;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self, prelude::*};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub hash: String,
    pub date: String,
    pub modified: bool,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let date_string = if self.modified == false {
            "Created"
        } else {
            "Last modified on"
        };

        write!(
            f,
            "\nName: {}\n\
            Desc: {}\n\
            {}: {} (yyyy-mm-dd)",
            self.name, self.desc, date_string, self.date
        )
    }
}
pub fn paste_to_clipboard(value: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value)
        .expect("Not able to copy to clipboard");
    println!(
        "{}",
        Green.paint("Password copied to clipbpoard for 30 seconds!")
    );
    thread::sleep(Duration::from_secs(30));
    println!("{}", Red.paint("Time's Up!"));
}

pub fn pass_ask(query: &str) -> String {
    loop {
        let pass =
            rpassword::prompt_password_stdout(&format!("{}: ", Purple.paint(query))).unwrap();
        let confirm =
            rpassword::prompt_password_stdout(&format!("{}: ", Purple.paint("Please enter again")))
                .unwrap();

        if pass == confirm {
            return pass;
        } else {
            println!("{}", Red.paint("Passwords do not match! Try again"))
        }
    }
}

pub fn ask(query: &str) -> String {
    print!("{}: ", Yellow.paint(query));
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().to_string()
}

pub fn new() -> String {
    let master_key = pass_ask("Please enter master Key");
    encrypt_master(&master_key)
}

pub fn create_new_item(name: &str, master: &str) -> Item {
    let desc = ask("Desc (Optional)");

    let mut option = ask("Create a random password? [Y/n]").to_ascii_lowercase();
    if option.is_empty() {
        option = "y".to_string();
    }
    let pass = if option == "y" {
        let length = ask("Enter the length (default: 15)");
        let len = if length.is_empty() {
            15
        } else {
            length.parse::<i32>().unwrap()
        };
        random(len)
    } else {
        pass_ask("Please enter the password")
    };

    let hash = encrypt_item(master, &pass);
    Item {
        name: name.to_string(),
        desc: desc,
        hash: hash,
        date: chrono::offset::Local::now().date().to_string(),
        modified: false,
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
        date: chrono::offset::Local::now().date().to_string(),
        modified: true,
    }
}

pub fn random(len: i32) -> String {
    (0..len)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
        .collect()
}
