mod auth;
mod manager;
#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate prettytable;
use ansi_term::Color::{Green, Purple, Red, Yellow};
use dirs::home_dir;
use manager::Item;
use prettytable::Table;
use rpassword;
use serde_json;
use std::collections::HashMap;

use std::{env, fs};

type DB = HashMap<String, Item>;

fn updatedb(config: &std::path::Path, database: &DB) {
    fs::File::open(&config).expect("Unable to open config file");
    let buffer = serde_json::to_string(&database).unwrap();
    fs::write(&config, &buffer).expect("something bad happend");
}

fn checkpresent(database: &DB, key: &str, error_msg: &str) {
    database
        .get(key)
        .or_else(|| {
            println!("{}", Red.paint(error_msg));
            std::process::exit(0);
        })
        .unwrap();
}

static COMMANDS: [&str; 10] = [
    "help", "new", "get", "destroy", "add", "update", "del", "list", "show", "info",
];
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
    if COMMANDS.contains(&query) == false {
        println!("{}", Red.paint("No such command. Consider this help menu"));
        println!("{}", HELP);
        std::process::exit(0);
    }
    match query {
        "new" => {
            if init == true {
                println!("{}",Red.paint("Looks like yout already have initialized passman. You can try other commands or run ` passman destroy ` to remove the current passwors and start from scratch"));
                std::process::exit(0);
            }
            let master_key = manager::new();
            fs::File::create(&secret).expect(&format!("{}", Red.paint("Unable to create file.")));
            fs::File::create(&config).expect(&format!("{}", Red.paint("Unable to create file.")));
            fs::write(&secret, master_key).expect("Not able to write to file");
            println!("{}", Green.paint("Passman has been successfully initialized. Have a secure day :)"));
        }

        _ => {
            if init == false {
                println!(
                    "{}",
                    Yellow.paint(
                        "You haven't made a init file yet. You can do that with ` passman new `"
                    )
                );
                std::process::exit(0);
            }
            let master = rpassword::prompt_password_stderr(&format!(
                "{}",
                Purple.paint("Enter the master key: "),
            ))
            .unwrap();
            if auth::authenticate(&master, &secret) == false {
                println!("{}", Red.paint("AUTHENTICATION FAILED!!"));
                std::process::exit(0);
            }
            println!("{}", Green.paint("AUTHENTICATION SUCCESS"));

            database = serde_json::from_reader(
                fs::File::open(&config)
                    .expect(&format!("{}", Red.paint("Unable to open config file."))),
            )
            .unwrap_or(DB::new());

            match query {
                "get" => {
                    let name = manager::ask("Name for the entry");
                    checkpresent(&database, &name, "No such entry");
                    let item = database.get(&name).unwrap();
                    let decrypted_pass = auth::decrypt_item(&master, &item.hash);
                    manager::paste_to_clipboard(decrypted_pass);
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
                    checkpresent(&database, &name, "No such entry");
                    let item = database.get(&name).unwrap();
                    let updated_entry = manager::update(item, &master);
                    database.insert(name, updated_entry);
                    println!("{}", Green.paint("The entry was successfully updated"));
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
                    checkpresent(&database, &name, "No such entry");
                    database.remove(&name);
                    println!(
                        "Succesfully removed all the data about entry {}",
                        Green.paint(name)
                    );
                    updatedb(&config, &database);
                }

                "info" => {
                    let name = manager::ask("Name of the entry");
                    checkpresent(&database, &name, "No such entry");
                    let item = database.get(&name).unwrap();
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
                _ => {}
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
