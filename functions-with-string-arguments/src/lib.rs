// In Rust, strings are composed of a slice of u8 and are guaranteed to be
// valid UTF-8, which allows for NUL bytes in the interior of the string. In C,
// strings are just pointers to a char and are terminated by a NUL byte (with
// the integer value 0). Some work is needed to convert between these two
// representations.
use libc::c_char;
use std::ffi::CStr;

/// # Safety
///
/// This is unsafe because it wraps the argument raw C string with a safe C
/// string wrapper by using CStr::from_ptr.
#[no_mangle]
pub unsafe extern "C" fn how_many_characters(text: *const c_char) -> i32 {
    if text.is_null() {
        return -1;
    }

    let c_str = CStr::from_ptr(text);
    let text_str = c_str.to_str().unwrap();

    text_str.chars().count() as i32
}

#[cfg(test)]
mod tests {
    use super::how_many_characters;
    use libc::c_char;
    use std::ffi::CString;
    use std::ptr;

    fn get_c_str(text: &str) -> *const c_char {
        CString::new(text)
            .expect("Could not covert to a c_char.")
            .into_raw()
    }

    fn assert_how_many_characters(text: *const c_char, expected_result: i32) {
        let result = unsafe { how_many_characters(text) };

        assert_eq!(expected_result, result);
    }

    #[test]
    fn null_text() {
        assert_how_many_characters(ptr::null(), -1);
    }

    #[test]
    fn empty_text() {
        assert_how_many_characters(get_c_str(""), 0);
    }

    #[test]
    fn text() {
        assert_how_many_characters(get_c_str("rust"), 4);
    }
}
