#![allow(unused_imports, unused_must_use, unused_assignments)]

mod manager;

use dirs::home_dir;
use manager::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{env, fs, io::Read, io::Write};

type Table = HashMap<String, Item>;

fn perform(query: &str) {
    let home = home_dir().expect("Home folder not found!");

    let config = home.join(".passman.json");
    let secret = home.join(".passman_key");
    let mut database: Table;

    let init = secret.is_file();
    match query {
        "new" => {
            if init == true {
                println!("Looks like yout already have initialized passman. You can try other commands or run `passman destroy` to remove the current passwors and start from scratch");
                std::process::exit(0);
            }
            let master_key = manager::new();
            fs::File::create(&secret).expect("Unable to create file.");
            fs::File::create(&config).expect("Unable to create file.");
            fs::write(&secret, master_key);
            database = Table::new();
        }

        _ => {
            if init == false {
                panic!("You haven't made a init file yet. You can do that with ` passman init `");
            }
            let pass = manager::ask("Enter password");
            if manager::authenticate(&pass, &secret) == false {
                println!("AUTH FAILED");
                std::process::exit(0);
            }
            println!("AUTH PASSED!");

            database = serde_json::from_reader(
                fs::File::open(&config).expect("Unable to open config file"),
            )
            .unwrap_or(Table::new());

            match query {
                "destroy" => {
                    fs::remove_file(&secret);
                    fs::remove_file(&config);
                    println!("Succesfully removed the config and password files.");
                    std::process::exit(0);
                }

                "add" => {
                    let name = manager::ask("Name for the entry");
                    let item = manager::create_new_item(&name);
                    database.insert(name, item);
                    println!("key entry suck cess");

                    fs::File::open(&config).expect("Unable to open config file");
                    let buffer = serde_json::to_string(&database).unwrap();
                    fs::write(&config, &buffer);
                    println!("Your entry was successfully added!");
                }
                "update" => {
                    let name = manager::ask("Name of the entry");
                    let present = &database.get(&name).expect("No such entry!");
                    let updated_entry = manager::update(present);
                    database.insert(name, updated_entry);
                    fs::File::open(&config).expect("Unable to open config file");
                    let buffer = serde_json::to_string(&database).unwrap();
                    fs::write(&config, &buffer);
                    println!("Your entry was successfully updated!");
                }
                "list" => {
                    unimplemented!();
                }
                "del" => {
                    unimplemented!();
                }
                "info" => {
                    unimplemented!();
                }
                _ => {
                    println!("{}", "YOOHOOOOOO!!!");
                }
            };
        }
    };
}

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();

    perform(&arg[0]);
}
