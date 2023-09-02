//! Operators for comparing via "ordering" relationships such as >=, <, ==
//!
use std::{borrow::Borrow, cmp::Ordering};

use crate::Operator;

pub struct GreaterEqual;

impl<Elem, BorrowedElem: ?Sized> Operator<Elem, BorrowedElem> for GreaterEqual
where
    Elem: Borrow<BorrowedElem>,
    BorrowedElem: PartialOrd,
{
    fn compare(&self, a: &BorrowedElem, b: &BorrowedElem) -> bool {
        matches!(a.partial_cmp(b), Some(Ordering::Greater | Ordering::Equal))
    }
}

pub struct Less;

impl<Elem, BorrowedElem: ?Sized> Operator<Elem, BorrowedElem> for Less
where
    Elem: Borrow<BorrowedElem>,
    BorrowedElem: PartialOrd,
{
    fn compare(&self, a: &BorrowedElem, b: &BorrowedElem) -> bool {
        matches!(a.partial_cmp(b), Some(Ordering::Less))
    }
}

pub struct Equal;

impl<Elem, BorrowedElem: ?Sized> Operator<Elem, BorrowedElem> for Equal
where
    Elem: Borrow<BorrowedElem>,
    BorrowedElem: PartialOrd,
{
    fn compare(&self, a: &BorrowedElem, b: &BorrowedElem) -> bool {
        matches!(a.partial_cmp(b), Some(Ordering::Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greater_equal_works() {
        assert!(!<GreaterEqual as Operator<i32, i32>>::compare(
            &GreaterEqual,
            &7,
            &8
        ));
        assert!(<GreaterEqual as Operator<i32, i32>>::compare(
            &GreaterEqual,
            &8,
            &7
        ));
    }
}
