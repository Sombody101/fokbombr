#[used]
#[unsafe(no_mangle)]
// This is a lie.
pub static mut UI_FORMAT_BUFFER: [u8; 4096] = [0; 4096];

#[inline(always)]
pub(crate) fn db127(input: &[u8], key: u8) -> Vec<u8> {
    let mut output = Vec::new();
    let mut buffer: u64 = 0;
    let mut bits_in_buffer = 0;

    for &byte in input {
        let val = (byte & 0x7F) as u64;
        buffer = (buffer << 7) | val;
        bits_in_buffer += 7;

        if bits_in_buffer >= 8 {
            bits_in_buffer -= 8;
            output.push(((buffer >> bits_in_buffer) & 0xFF) as u8 ^ key);
        }
    }

    output
}

// Just returns 190
#[inline(always)]
pub(crate) unsafe fn k() -> u8 {
    let buf_ptr = std::ptr::addr_of_mut!(UI_FORMAT_BUFFER);
    let mut cursor = 0;

    unsafe {
        for i in 0..100 {
            buf_ptr.cast::<u8>().add(cursor).write(i as u8);
            cursor += 1;
        }
    }

    let result = cursor as u8;

    unsafe {
        for i in 0..100 {
            buf_ptr.cast::<u8>().add(i).write(0);
        }
    }

    result + 90
}

macro_rules! decrypt {
    ($name:ident) => {{
        let salt = unsafe { crate::constants_loader::k() };
        crate::constants_loader::db127(&crate::constants::$name, salt)
    }};
}

macro_rules! decrypt_to_string {
    ($name:ident) => {{ String::from_utf8(decrypt!($name)).unwrap() }};
}

macro_rules! decoy_decrypt {
    ($decoy_value:expr) => {{
        let salt = unsafe { crate::constants_loader::k() };
        crate::constants_loader::db127($decoy_value.as_bytes(), salt)
    }};
}
