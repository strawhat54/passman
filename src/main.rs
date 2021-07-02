#![allow(unused_imports, unused_must_use, unused_assignments, dead_code)]
mod auth;
mod manager;
#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate prettytable;
use ansi_term::Color::{Green, Purple, Red, Yellow};
use clipboard::{ClipboardContext, ClipboardProvider};
use dirs::home_dir;
use manager::Item;
use prettytable::{Cell, Row, Table};
use serde_json;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::{env, fs};

type DB = HashMap<String, Item>;

fn updatedb(config: &std::path::Path, database: &DB) {
    fs::File::open(&config).expect("Unable to open config file");
    let buffer = serde_json::to_string(&database).unwrap();
    fs::write(&config, &buffer).expect("something bad happend");
    println!("Your entry was successfully added!");
}

static HELP: &str = r"
USAGE: passman <option>

Currently available options are:
--------------------------------

help      display this help message and exit
new       initalize passman with a new master key
get       to copy the password of the given query to clipboard for 30 seconds
destory   destroy the current passman config and hence delete all the data
add       add a new entry to passman
update    updates the password of a registered entry
del       deletes a registered entry
list      lists all the available keys
show      displays a table consisting of entries and their passwords(for exporting)
info      displays information about the queried entry
";

fn perform(query: &str) {
    let home = home_dir().expect("Home folder not found!");
    let config = home.join(".passman.json");
    let secret = home.join(".passman_key");
    let mut database: DB;

    let init = secret.is_file();
    match query {
        "new" => {
            if init == true {
                println!("{}",Red.paint("Looks like yout already have initialized passman. You can try other commands or run passman destroy to remove the current passwors and start from scratch"));
                std::process::exit(0);
            }
            let master_key = manager::new();
            fs::File::create(&secret).expect(format!("{}", Red.paint("Unable to create file.")));
            fs::File::create(&config).expect(format!("{}", Red.paint("Unable to create file.")));
            fs::write(&secret, master_key);
        }

        _ => {
            if init == false {
                panic!(format!(
                    "{}",
                    Yellow.paint(
                        "You haven't made a init file yet. You can do that with  passman init"
                    )
                ));
            }
            let master = manager::ask("Enter password");
            if auth::authenticate(&master, &secret) == false {
                println!("{}", Red.paint("AUTH FAILED!!"));
                std::process::exit(0);
            }
            println!("{}", Green.paint("AUTH PASSED!!, Greetings Master"));

            database = serde_json::from_reader(
                fs::File::open(&config)
                    .expect(format!("{}", Red.paint("Unable to open config file."))),
            )
            .unwrap_or(DB::new());

            match query {
                "get" => {
                    let name = manager::ask("Name for the entry");
                    let item = database
                        .get(&name)
                        .or_else(|| {
                            println!("{}", Red.paint("No such entry"));
                            std::process::exit(0);
                        })
                        .unwrap();

                    let decrypted_pass = auth::decrypt_item(&master, &item.hash);
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(decrypted_pass);
                    println!(
                        "{}",
                        Green.paint("Password copied to clipbpoard for 30 seconds!")
                    );
                    thread::sleep(Duration::from_secs(30));
                    println!("{}", Red.paint("Time's Up!"));
                }

                "destroy" => {
                    fs::remove_file(&secret).expect("someone deleted the secret file already");
                    fs::remove_file(&config).expect("someone deleted the config file already");
                    println!(
                        "{}",
                        Green.paint("Succesfully removed the config and password files.")
                    );
                }

                "add" => {
                    let name = manager::ask("Name for the entry");
                    let item = manager::create_new_item(&name, &master);
                    database.insert(name, item);
                    println!("{}", Green.paint("The entry was successfully registered"));
                    updatedb(&config, &database);
                }

                "update" => {
                    let name = manager::ask("Name of the entry");
                    let present = database
                        .get(&name)
                        .or_else(|| {
                            println!("{}", Red.paint("No such entry"));
                            std::process::exit(0);
                        })
                        .unwrap();
                    let updated_entry = manager::update(present, &master);
                    database.insert(name, updated_entry);
                    updatedb(&config, &database);
                }
                "list" => {
                    println!("{}", Purple.paint("The list of stored keys are: "));
                    for (key, _) in database {
                        println!("- {}", key);
                    }
                }

                "del" => {
                    let name = manager::ask("Name of the entry");
                    let _ = database.get(&name).or_else(|| {
                        println!("{}", Red.paint("No such entry"));
                        std::process::exit(0);
                    });
                    database.remove(&name);
                    println!(
                        "Succesfully removed all the data about entry {}",
                        Green.paint(name)
                    );
                    updatedb(&config, &database);
                }

                "info" => {
                    let name = manager::ask("Name of the entry");
                    let item = database
                        .get(&name)
                        .or_else(|| {
                            println!("{}", Red.paint("No such entry"));
                            std::process::exit(0);
                        })
                        .unwrap();
                    println!("{}", item);
                }

                "show" => {
                    let mut table = Table::new();
                    table.add_row(row!["name", "password"]);
                    for (name, item) in database {
                        table.add_row(row![name, auth::decrypt_item(&master, &item.hash)]);
                    }
                    table.printstd();
                }
                _ => {
                    println!("{}", Red.paint("No such option"));
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
