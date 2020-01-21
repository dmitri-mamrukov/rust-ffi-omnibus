// Let’s create a Rust object that will tell us how many people live in each
// USA ZIP code. We want to be able to use this logic in other languages, but
// we only need to pass simple primitives like integers or strings across the
// FFI boundary. The object will have both mutable and immutable methods.
// Because we can not look inside the object, this is often referred to as an
// opaque object or an opaque pointer.
use libc::c_char;
use std::collections::HashMap;
use std::ffi::CStr;

pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
}

impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100_000 {
            let zip_code = format!("{:05}", i);
            self.population.insert(zip_code, i);
        }
    }

    fn population_of(&self, zip_code: &str) -> u32 {
        self.population.get(zip_code).cloned().unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

/// # Safety
///
/// This is unsafe because it calls Box::from_raw.
#[no_mangle]
pub unsafe extern "C" fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() {
        return;
    }

    Box::from_raw(ptr);
}

/// # Safety
///
/// This is unsafe because it dereferences the ZipCodeDatabase pointer.
#[no_mangle]
pub unsafe extern "C" fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    assert!(!ptr.is_null());

    let database = &mut *ptr;

    database.populate();
}

/// # Safety
///
/// This is unsafe because it dereferences the ZipCodeDatabase and string
/// pointers.
#[no_mangle]
pub unsafe extern "C" fn zip_code_database_population_of(
    ptr: *const ZipCodeDatabase,
    zip_code: *const c_char,
) -> u32 {
    assert!(!ptr.is_null());
    assert!(!zip_code.is_null());

    let database = &*ptr;
    let zip_code = CStr::from_ptr(zip_code);
    let zip_code = zip_code.to_str().expect("Could not covert to &str.");

    database.population_of(zip_code)
}

#[cfg(test)]
mod ffi_tests {
    use super::{
        zip_code_database_free, zip_code_database_new, zip_code_database_populate,
        zip_code_database_population_of, ZipCodeDatabase,
    };
    use libc::c_char;
    use std::ffi::CString;

    #[test]
    fn new_and_populate_and_free() {
        let db_ptr: *mut ZipCodeDatabase = zip_code_database_new();
        unsafe {
            zip_code_database_populate(db_ptr);
            zip_code_database_free(db_ptr);
        }

        assert!(!db_ptr.is_null());
    }

    #[test]
    fn test_zip_code_database_population_of() {
        let db_ptr: *mut ZipCodeDatabase = zip_code_database_new();
        unsafe {
            zip_code_database_populate(db_ptr);
        }

        for i in 0..100_000 {
            let zip_code = CString::new(format!("{:05}", i).as_str()).unwrap();
            let zip_code_ptr = zip_code.as_ptr() as *const c_char;

            let result = unsafe { zip_code_database_population_of(db_ptr, zip_code_ptr) };

            assert_eq!(i, result);
        }

        let zip_code = CString::new("non-existing-zip-code").unwrap();
        let zip_code_ptr = zip_code.as_ptr() as *const c_char;

        let result = unsafe { zip_code_database_population_of(db_ptr, zip_code_ptr) };

        assert_eq!(0, result);

        unsafe {
            zip_code_database_free(db_ptr);
        }
    }
}
