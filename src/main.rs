#![allow(unused_imports, unused_must_use)]

mod manager;

use dirs::home_dir;
use manager::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{env, fs, io::Read, io::Write};

fn perform(query: &str) {
    let home = home_dir().expect("Home folder not found!");

    let config = home.join(".passman.json");
    let secret = home.join(".passman_key");

    let init = secret.is_file();
    match query {
        "new" => {
            if init == true {
                println!("Looks like yout already have initialized passman. You can try other commands or run `passman destroy` to remove the current passwors and start from scratch");
                std::process::exit(0);
            }
            let master_key = manager::new();
            fs::File::create(&secret).expect("Unable to create file.");
            fs::write(&secret, master_key);
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

            match query {
                "destroy" => {
                    fs::remove_file(&secret);
                    fs::remove_file(&config);
                    println!("Succesfully removed the config and password files.");
                    std::process::exit(0);
                }

                "add" => {
                    unimplemented!();
                }
                "update" => {
                    unimplemented!();
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
