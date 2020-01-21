use libc::size_t;
use std::slice;

/// # Safety
///
/// This is unsafe because it calls slice::from_raw_parts.
#[no_mangle]
pub unsafe extern "C" fn sum_of_even(numbers_ptr: *const u32, len: size_t) -> i32 {
    if numbers_ptr.is_null() {
        return -1;
    }

    let numbers = slice::from_raw_parts(numbers_ptr, len as usize);
    let sum: u32 = numbers.iter().filter(|&x| x % 2 == 0).sum();

    sum as i32
}

#[cfg(test)]
mod tests {
    use super::sum_of_even;
    use libc::size_t;
    use std::ptr;

    fn assert_sum_of_even(numbers: *const u32, len: size_t, expected_result: i32) {
        let result = unsafe { sum_of_even(numbers, len) };

        assert_eq!(expected_result, result);
    }

    #[test]
    fn null_numbers() {
        assert_sum_of_even(ptr::null(), 0, -1);
    }

    #[test]
    fn empty_numbers() {
        let numbers: [u32; 0] = [];
        assert_sum_of_even(numbers.as_ptr(), 0, 0);
    }

    #[test]
    fn numbers_as_zero() {
        let numbers: [u32; 1] = [0];
        assert_sum_of_even(numbers.as_ptr(), 1, 0);
    }

    #[test]
    fn numbers_as_one() {
        let numbers: [u32; 1] = [1];
        assert_sum_of_even(numbers.as_ptr(), 1, 0);
    }

    #[test]
    fn numbers_as_two() {
        let numbers: [u32; 1] = [2];
        assert_sum_of_even(numbers.as_ptr(), 1, 2);
    }

    #[test]
    fn numbers_as_several_elements() {
        let numbers: [u32; 7] = [0, 1, 2, 3, 4, 5, 6];
        assert_sum_of_even(numbers.as_ptr(), 7, 12);
    }
}
