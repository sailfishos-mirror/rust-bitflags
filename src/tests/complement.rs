use super::*;

use crate::Flags;

fn case<T: Flags + std::ops::Not<Output = T> + Copy>(
    expected: T::Bits,
    value: T,
    inherent: impl FnOnce(T) -> T,
) where
    T::Bits: std::fmt::Debug + PartialEq,
{
    assert_eq!(expected, inherent(value).bits());
    assert_eq!(expected, Flags::complement(value).bits());
    assert_eq!(expected, (!value).bits());
}

#[test]
fn cases() {
    case(0, TestFlags::all(), TestFlags::complement);
    case(0, TestFlags::from_bits_retain(!0), TestFlags::complement);

    case(1 | 1 << 1, TestFlags::C, TestFlags::complement);
    case(
        1 | 1 << 1,
        TestFlags::C | TestFlags::from_bits_retain(1 << 3),
        TestFlags::complement,
    );

    case(
        1 | 1 << 1 | 1 << 2,
        TestFlags::empty(),
        TestFlags::complement,
    );
    case(
        1 | 1 << 1 | 1 << 2,
        TestFlags::from_bits_retain(1 << 3),
        TestFlags::complement,
    );

    case(0, TestZero::empty(), TestZero::complement);

    case(0, TestEmpty::empty(), TestEmpty::complement);
}
