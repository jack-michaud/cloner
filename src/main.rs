use std::{env, fs};
use std::process::Command;
use std::vec::Vec;
use git_url_parse::GitUrl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) {
        println!("No git url provided");
        return
    };

    let url = &args[1];
    println!("Got url: {}", url);

    if let Ok(git_url) = GitUrl::parse(url.as_str()) {
        let target_dir = format!("{}/Code/{}/{}", env::var("HOME").unwrap(), git_url.host.unwrap(), git_url.owner.unwrap());
        println!("Creating target: {}", target_dir);
        fs::create_dir_all(target_dir.clone()).unwrap();
        let command = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(format!("{}/{}", target_dir, git_url.name))
            .output()
            .expect("Unable to clone url");
        println!("{:?}", command);
        println!("Cloned into: {}", git_url.name);
    } else {
        eprintln!("Unable to parse url");
    };
}
