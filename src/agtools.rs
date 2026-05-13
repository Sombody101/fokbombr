use std::fs;
use std::path::Path;

pub(crate) fn get_username() -> String {
    if cfg!(windows) {
        std::env::var("USERNAME")
    } else {
        std::env::var("USER")
    }
    .unwrap()
}

pub(crate) fn start_proc(command: &str) {
    if cfg!(debug_assertions) {
        return;
    }

    let parts: Vec<&str> = command.split(' ').collect();
    let cmd = parts[0];
    let args = &parts[1..parts.len()];

    let _ = std::process::Command::new(cmd).args(args).output();
}

pub(crate) fn copy_proc(from: &str, to: &str) {
    if cfg!(debug_assertions) {
        let _ = fs::copy(from, to);
    }
}

pub(crate) fn ensure_dir(target: &str) -> bool {
    let path = Path::new(target);
    if !path.exists() {
        if let Err(_) = fs::create_dir(path) {
            return false;
        }
    }

    true
}
