use git_url_parse::GitUrl;
use std::process::Command;
use std::{env, fs};

pub type GitCloneResult = Result<(), ()>;

/// The preferred directory is in the structure:
/// $HOME/Code/{git_host}/{author}/{repository}
pub fn clone_git_url_to_preferred_directory(git_url: String) -> GitCloneResult {
    let parsed_git_url = GitUrl::parse(git_url.as_str()).unwrap();
    let target_dir = format!(
        "{}/Code/{}/{}",
        env::var("HOME").unwrap(),
        parsed_git_url.host.unwrap(),
        parsed_git_url.owner.unwrap()
    );
    return clone_git_url_to_directory(git_url, target_dir);
}

pub fn clone_git_url_to_directory(git_url: String, target_dir: String) -> GitCloneResult {
    if let Ok(parsed_git_url) = GitUrl::parse(git_url.as_str()) {
        println!("Creating target: {}", target_dir);
        fs::create_dir_all(target_dir.clone()).unwrap();
        let command = Command::new("git")
            .arg("clone")
            .arg(git_url)
            .arg(format!("{}/{}", target_dir, parsed_git_url.name))
            .output()
            .expect("Unable to clone url");
        println!("Cloned into: {}", parsed_git_url.name);
        return Ok(());
    } else {
        eprintln!("Unable to parse url");
        return Err(());
    };
}
