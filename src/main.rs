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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

struct Item {
    name: String,
    desc: String,
    salt: Vec<u8>,
    // created: cration time
    // date: last update time
}

type Table = HashMap<Item, bool>;
static HELP: &str = "HELP MESSAGE";

fn perform(query: &str) {
    let home = home_dir()
        .expect("Home folder not found!")
        .join(".passman.json");

    let init = home.is_file();

    let value = match query {
        "new" => {
            if init {
                panic!("Looks like yout already have initialized passman. You can try other commands or run `passman destroy` to remove the current passwors and start from scratch");
            }
            // Store master Key
            // DO SOMETHING
        }

        _ => {
            if init == false {
                panic!("You haven't made a init file yet. You can do that with ` passman init `");
            }

            // AUTHENTICATE

            match query {
                "add" => {
                    unimplemented!()
                }
                "update" => {
                    unimplemented!()
                }
                "list" => {
                    unimplemented!()
                }
                "del" => {
                    unimplemented!()
                }
                "info" => {
                    unimplemented!()
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

    perform(&arg[0]);
}
