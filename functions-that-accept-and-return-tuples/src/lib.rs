// C has no notion of tuples, but the closest analog is a plain struct. You
// will need to create individual structs for each unique combination of types.
// Here, we create a structure that represents two 32-bit unsigned integers.
use std::convert::From;

fn x(tup: (u32, u32)) -> (u32, u32) {
    let (a, b) = tup;

    (b.wrapping_add(1), a.wrapping_sub(1))
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Tuple {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Tuple {
    fn from(tup: (u32, u32)) -> Tuple {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (u32, u32) {
    fn from(tup: Tuple) -> (u32, u32) {
        (tup.x, tup.y)
    }
}

#[no_mangle]
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple {
    x(tup.into()).into()
}

#[cfg(test)]
mod tests {
    use super::{flip_things_around, Tuple};

    fn assert_flip_things_around(tup: Tuple, expected_result: Tuple) {
        let result = flip_things_around(tup);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn zero_and_zero() {
        assert_flip_things_around((0, 0).into(), (1, std::u32::MAX).into());
    }

    #[test]
    fn zero_and_two() {
        assert_flip_things_around((0, 2).into(), (3, std::u32::MAX).into());
    }

    #[test]
    fn zero_and_max() {
        assert_flip_things_around((0, std::u32::MAX).into(), (0, std::u32::MAX).into());
    }

    #[test]
    fn one_and_zero() {
        assert_flip_things_around((1, std::u32::MAX).into(), (0, 0).into());
    }

    #[test]
    fn one_and_two() {
        assert_flip_things_around((1, 2).into(), (3, 0).into());
    }

    #[test]
    fn one_and_max() {
        assert_flip_things_around((1, std::u32::MAX).into(), (0, 0).into());
    }

    #[test]
    fn max_and_zero() {
        assert_flip_things_around((std::u32::MAX, 0).into(), (1, std::u32::MAX - 1).into());
    }

    #[test]
    fn max_and_two() {
        assert_flip_things_around((std::u32::MAX, 2).into(), (3, std::u32::MAX - 1).into());
    }

    #[test]
    fn max_and_max() {
        assert_flip_things_around(
            (std::u32::MAX, std::u32::MAX).into(),
            (0, std::u32::MAX - 1).into(),
        );
    }
}
