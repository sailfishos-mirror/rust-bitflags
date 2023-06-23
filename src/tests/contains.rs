use super::*;

use crate::Flags;

fn case<T: Flags + Copy>(value: T, inputs: &[(T, bool)], mut inherent: impl FnMut(&T, T) -> bool) {
    for (input, expected) in inputs {
        assert_eq!(*expected, inherent(&value, *input));
        assert_eq!(*expected, value.contains(*input));
    }
}

#[test]
fn cases() {
    case(
        TestFlags::empty(),
        &[
            (TestFlags::empty(), true),
            (TestFlags::A, false),
            (TestFlags::B, false),
            (TestFlags::C, false),
            (TestFlags::from_bits_retain(1 << 3), false),
        ],
        TestFlags::contains,
    );

    case(
        TestFlags::A,
        &[
            (TestFlags::empty(), true),
            (TestFlags::A, true),
            (TestFlags::B, false),
            (TestFlags::C, false),
            (TestFlags::from_bits_retain(1 << 3), false),
        ],
        TestFlags::contains,
    );

    case(
        TestFlags::ABC,
        &[
            (TestFlags::empty(), true),
            (TestFlags::A, true),
            (TestFlags::B, true),
            (TestFlags::C, true),
            (TestFlags::from_bits_retain(1 << 3), false),
        ],
        TestFlags::contains,
    );

    case(
        TestFlags::from_bits_retain(1 << 3),
        &[
            (TestFlags::empty(), true),
            (TestFlags::A, false),
            (TestFlags::B, false),
            (TestFlags::C, false),
            (TestFlags::from_bits_retain(1 << 3), true),
        ],
        TestFlags::contains,
    );

    case(
        TestZero::ZERO,
        &[(TestZero::ZERO, true)],
        TestZero::contains,
    );
}
