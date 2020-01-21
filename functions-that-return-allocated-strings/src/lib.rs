// Returning an allocated string via FFI is complicated for the same reason that
// returning an object is: the Rust allocator can be different from the
// allocator on the other side of the FFI boundary. It also has the same
// restrictions dealing with NUL-terminated strings as passing a string
// argument.
use libc::c_char;
use std::ffi::CString;
use std::iter;

// Here we use a pair of methods into_raw and from_raw. These convert a CString
// into a raw pointer that may be passed across the FFI boundary. Ownership of
// the string is transferred to the caller, but the caller must return the
// string to Rust in order to properly deallocate the memory.

#[no_mangle]
pub extern "C" fn generate_theme_song(length: u8) -> *mut c_char {
    let mut song = String::from("💣 ");
    song.extend(iter::repeat("na ").take(length as usize));
    song.push_str("Batman! 💣");

    let c_str_song = CString::new(song).expect("CString::new failed.");

    c_str_song.into_raw()
}

/// # Safety
///
/// This is unsafe because it wraps the argument raw C string with a safe C
/// string wrapper by using CStr::from_raw.
#[no_mangle]
pub unsafe extern "C" fn free_theme_song(text: *mut c_char) {
    if text.is_null() {
        return;
    }

    CString::from_raw(text);
}

#[cfg(test)]
mod tests {
    use super::{free_theme_song, generate_theme_song};
    use std::ffi::CString;

    fn assert_theme_song(length: u8, expected_result: String) {
        let result_ptr = generate_theme_song(length);

        let song = unsafe {
            CString::from_raw(result_ptr)
                .into_string()
                .expect("Could not covert to String.")
        };
        assert_eq!(expected_result, song);

        unsafe {
            free_theme_song(result_ptr);
        }

        assert!(!result_ptr.is_null());
    }

    #[test]
    fn length_as_zero() {
        assert_theme_song(0, "💣 Batman! 💣".to_string());
    }

    #[test]
    fn length_as_one() {
        assert_theme_song(1, "💣 na Batman! 💣".to_string());
    }

    #[test]
    fn length_as_two() {
        assert_theme_song(2, "💣 na na Batman! 💣".to_string());
    }
}
