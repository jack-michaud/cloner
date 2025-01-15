use std::vec::Vec;
use std::{env, fs};

mod cloner_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) {
        println!("No git url provided");
        return;
    };

    let url = &args[1];
    println!("Got url: {}", url);
    match cloner_lib::clone_git_url_to_preferred_directory(url.clone()) {
        Ok(_) => println!("Clone successful"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
