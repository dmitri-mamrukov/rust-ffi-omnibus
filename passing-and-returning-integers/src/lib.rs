// Integers are the “hello world!” of FFI, as they are generally much easier to
// pass across the boundary.

enum ErrCode {
    Success,
    AdditionError,
}

impl ErrCode {
    fn value(&self) -> u32 {
        match *self {
            ErrCode::Success => 0,
            ErrCode::AdditionError => 1,
        }
    }
}

// If addition succeeds, return (a + b, 0); otherwise, return (0, 1).
#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> (u32, u32) {
    let result = a.checked_add(b);

    if let Some(sum) = result {
        (sum, ErrCode::Success.value())
    } else {
        (0, ErrCode::AdditionError.value())
    }
}

#[cfg(test)]
mod tests {
    use super::addition;

    fn assert_addition(a: u32, b: u32, expected_result: (u32, u32)) {
        let result = addition(a, b);

        assert_eq!(expected_result.0, result.0);
        assert_eq!(expected_result.1, result.1);
    }

    #[test]
    fn zero_and_zero() {
        assert_addition(0, 0, (0, 0));
    }

    #[test]
    fn zero_and_positive() {
        assert_addition(0, 1, (1, 0));
    }

    #[test]
    fn zero_and_max() {
        assert_addition(0, std::u32::MAX, (std::u32::MAX, 0));
    }

    #[test]
    fn positive_and_zero() {
        assert_addition(1, 0, (1, 0));
    }

    #[test]
    fn positive_and_positive() {
        assert_addition(1, 1, (2, 0));
    }

    #[test]
    fn one_and_max_minus_one() {
        assert_addition(1, std::u32::MAX - 1, (std::u32::MAX, 0));
    }

    #[test]
    fn one_and_max() {
        assert_addition(1, std::u32::MAX, (0, 1));
    }
}
