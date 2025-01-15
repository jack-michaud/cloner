use git_url_parse::GitUrl;
use std::process::Command;
use std::{env, fs};

pub type GitCloneResult = Result<(), GitCloneError>;

#[derive(Debug)]
pub enum GitCloneError {
    ParseError(String),
    EnvVarError(String),
    DirCreationError(String),
    CommandError(String),
}

fn log_error(error: &GitCloneError) {
    eprintln!("Error: {:?}", error);
}

/// The preferred directory is in the structure:
/// $HOME/Code/{git_host}/{author}/{repository}
pub fn clone_git_url_to_preferred_directory(git_url: String) -> GitCloneResult {
    let parsed_git_url = GitUrl::parse(git_url.as_str()).map_err(|e| {
        let error = GitCloneError::ParseError(e.to_string());
        log_error(&error);
        error
    })?;
    let target_dir = format!(
        "{}/Code/{}/{}",
        env::var("HOME").map_err(|e| {
            let error = GitCloneError::EnvVarError(e.to_string());
            log_error(&error);
            error
        })?,
        parsed_git_url.host.unwrap(),
        parsed_git_url.owner.unwrap()
    );
    clone_git_url_to_directory(git_url, target_dir)
}

pub fn clone_git_url_to_directory(git_url: String, target_dir: String) -> GitCloneResult {
    let parsed_git_url = GitUrl::parse(git_url.as_str()).map_err(|e| {
        let error = GitCloneError::ParseError(e.to_string());
        log_error(&error);
        error
    })?;
    println!("Creating target: {}", target_dir);
    fs::create_dir_all(target_dir.clone()).map_err(|e| {
        let error = GitCloneError::DirCreationError(e.to_string());
        log_error(&error);
        error
    })?;
    let command = Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(format!("{}/{}", target_dir, parsed_git_url.name))
        .output()
        .map_err(|e| {
            let error = GitCloneError::CommandError(e.to_string());
            log_error(&error);
            error
        })?;
    if !command.status.success() {
        let error = GitCloneError::CommandError(String::from_utf8_lossy(&command.stderr).to_string());
        log_error(&error);
        return Err(error);
    }
    println!("Cloned into: {}", parsed_git_url.name);
    Ok(())
}
