use std::borrow::Borrow;

mod contains;
pub use contains::*;
mod cmp;
pub use cmp::*;

use crate::SetRelation;

pub trait Operator<Elem, BorrowedElem: ?Sized>
where
    Elem: Borrow<BorrowedElem>,
{
    fn compare(&self, a: &BorrowedElem, b: &BorrowedElem) -> bool;
}

pub struct OperatorConstraint<Elem, Op> {
    op: Op,
    elem: Elem,
}

impl<Elem, Op> OperatorConstraint<Elem, Op> {
    #[allow(unused_variables)]
    pub fn new(op: Op, elem: Elem) -> Self {
        OperatorConstraint { op, elem }
    }
}

impl<Elem, BorrowedElem: ?Sized, Op> SetRelation<Elem, BorrowedElem>
    for OperatorConstraint<Elem, Op>
where
    Elem: Borrow<BorrowedElem>,
    Op: Operator<Elem, BorrowedElem>,
{
    fn is_member(&self, elem: &BorrowedElem) -> bool
    where
        Elem: Borrow<BorrowedElem>,
    {
        self.op.compare(elem, self.elem.borrow())
    }
}

impl<Elem, BorrowedElem> Operator<Elem, BorrowedElem> for Box<dyn Operator<Elem, BorrowedElem>>
where
    Elem: Borrow<BorrowedElem>,
{
    fn compare(&self, a: &BorrowedElem, b: &BorrowedElem) -> bool {
        (**self).compare(a, b)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn greater_equal_works() {
        let set = OperatorConstraint::new(GreaterEqual, 5_u64);
        assert!(set.is_member(&8));
        assert!(set.is_member(&5));
        assert!(!set.is_member(&3));
    }

    #[test]
    fn contains_works() {
        let set = OperatorConstraint::new(Contains, String::from("_"));
        assert!(set.is_member("snake_case"));
        assert!(set.is_member("_var"));
        assert!(!set.is_member("camelCase"));
    }

    #[test]
    fn starts_with_works() {
        let set = OperatorConstraint::new(StartsWith, String::from("_"));
        assert!(!set.is_member("snake_case"));
        assert!(set.is_member("_var"));
        assert!(!set.is_member("camelCase"));
    }

    #[test]
    fn starts_with_works_with_path() {
        let set = OperatorConstraint::new(StartsWith, PathBuf::from("/paths"));
        assert!(!set.is_member(&PathBuf::from("nope")));
        assert!(set.is_member(&PathBuf::from("/paths/start/")));
        assert!(set.is_member(&PathBuf::from("/paths")));
        assert!(set.is_member(&PathBuf::from("/paths/")));
        assert!(!set.is_member(&PathBuf::from("/paths_are_special")));
    }
}
