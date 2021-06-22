use passman::perform;
use std::env;

fn main() {
    let arg: Vec<String> = env::args().skip(1).collect();
    let query = &arg[0];
    perform(&query);
}
