#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

mod manager;

use dirs::home_dir;
use manager::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{env, fs, io::Read, io::Write};
type Table = HashMap<String, Lol>;
static HELP: &str = "HELP MESSAGE";
use orion::auth::SecretKey;

// TEST: READ/WRITE HASHMAPS (SERDE_JSON)
#[derive(Serialize, Deserialize, Debug)]
struct Lol {
    name: String,
}

#[test]
fn test_func() {
    let home = home_dir().expect("Home folder not found!");
    let mut hmap = HashMap::new();
    let test = home.join(".test_pass.json");
    let x = Lol {
        name: "pass".to_string(),
    };
    hmap.insert("pass".to_string(), x);
    let file = fs::File::create(&test).unwrap();
    let json = serde_json::to_writer(file, &hmap);

    let f = std::fs::File::open(&test).unwrap();
    let z: HashMap<String, Lol> = serde_json::from_reader(f).unwrap();

    println!("{:?}", z);
}
// --------------END-----------------

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
                print!("AUTH FAILED");
                std::process::exit(0);
            }
            println!("AUTH PASSED!");

            match query {
                "destroy" => {
                    fs::remove_file(&secret);
                    fs::remove_file(&config);
                    println!("Succesfully removed the config and password files.");
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

    // Write to file (seperate file for each key:val in $HOME/.passman/json)
}

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();

    perform(&arg[0]);
}
