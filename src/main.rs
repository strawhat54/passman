#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

mod manager;

use bincode;
use dirs::home_dir;
use manager::Item;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{env, fs, io::Read, io::Write};
type Table = HashMap<String, Lol>;
static HELP: &str = "HELP MESSAGE";



// TEST: READ/WRITE HASHMAPS (SERDE_JSON)
#[derive(Serialize, Deserialize, Debug)]
struct Lol {
    name: String,
}

fn jj() {
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
    let secret = home.join(".passman_key.json");

    let init = secret.is_file();
    match query {
        "new" => {
            if init == true {
                println!("Looks like yout already have initialized passman. You can try other commands or run `passman destroy` to remove the current passwors and start from scratch");
                std::process::exit(0);
            } else {
                let val = manager::new();
                let json = serde_json::to_string(&val).unwrap();
                fs::write(&secret, json);
                let packed = fs::read_to_string(&secret).unwrap();
                let v: Vec<u8> = serde_json::from_str(&packed).unwrap();

                println!("{}", v == val);
            }
        }

        _ => {
            if init == false {
                panic!("You haven't made a init file yet. You can do that with ` passman init `");
            }

            // AUTHENTICATE

            match query {
                "destroy" => {
                    unimplemented!();
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
                    print!("{}", HELP);
                }
            };
        }
    };

    // Write to file (seperate file for each key:val in $HOME/.passman/json)
}

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();

    // perform(&arg[0]);
    jj();
}
