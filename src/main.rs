#![allow(unused_imports, unused_must_use, unused_assignments, dead_code)]

mod manager;

use dirs::home_dir;
use manager::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::marker::Sized;
use std::{env, fs, io::Read, io::Write};

type Table = HashMap<String, Item>;

fn updatedb(config: &std::path::Path, database: &Table) {
    fs::File::open(&config).expect("Unable to open config file");
    let buffer = serde_json::to_string(&database).unwrap();
    fs::write(&config, &buffer);
    println!("Your entry was successfully added!");
}

static HELP: &str = r"
USAGE: passman <option>

Currently available options are:

help      display this help message and exit
new       initalize passman with a new master key
destory   destroy the current passman config and hence delete all the data
add       add a new entry to passman
update    updates the password of a registered entry
del       deletes a registered entry
list      lists all the available keys
info      displays information about the queried entry
";

fn perform(query: &str) {
    let home = home_dir().expect("Home folder not found!");

    let config = home.join(".passman.json");
    let secret = home.join(".passman_key");
    let mut database: Table;

    let init = secret.is_file();
    match query {
        "new" => {
            if init == true {
                println!("Looks like yout already have initialized passman. You can try other commands or run passman destroy to remove the current passwors and start from scratch");
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
                panic!("You haven't made a init file yet. You can do that with  passman init ");
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
                }

                "add" => {
                    let name = manager::ask("Name for the entry");
                    let item = manager::create_new_item(&name);
                    database.insert(name, item);
                    println!("key entry suck cess");
                    updatedb(&config, &database);
                }

                "update" => {
                    let name = manager::ask("Name of the entry");
                    let present = database
                        .get(&name)
                        .or_else(|| {
                            println!("No such entry");
                            std::process::exit(0);
                        })
                        .unwrap();
                    let updated_entry = manager::update(present);
                    database.insert(name, updated_entry);
                    updatedb(&config, &database);
                }
                "list" => {
                    println!("The list of stored keys are: ");
                    for (key, _) in database {
                        println!("- {}", key);
                    }
                }

                "del" => {
                    let name = manager::ask("Name of the entry");
                    let _ = database.get(&name).or_else(|| {
                        println!("No such entry");
                        std::process::exit(0);
                    });
                    database.remove(&name);
                    println!("Succesfully removed all the data about entry {}", name);
                    updatedb(&config, &database);
                }

                "info" => {
                    let name = manager::ask("Name of the entry");
                    let item = database
                        .get(&name)
                        .or_else(|| {
                            println!("No such entry");
                            std::process::exit(0);
                        })
                        .unwrap();
                    println!("{}", item);
                }
                _ => {
                    println!("No such option");
                    println!("{}", HELP);
                }
            };
        }
    };
}

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();
    if (arg.len() == 0) || (arg[0].to_lowercase() == "help") {
        println!("{}", HELP);
        std::process::exit(0);
    }
    perform(&arg[0].to_lowercase());
}
