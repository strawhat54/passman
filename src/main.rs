#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
)]

mod manager;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();
    let _query = &arg[0];
}
