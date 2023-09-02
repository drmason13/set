use std::{
    borrow::Borrow,
    cmp::Ordering,
    path::{Path, PathBuf},
};

// BorrowedElem is required so that we know what type to use for comparisons, which borrow from the set (otherwise they'd have to own it!)
//
// By using a separate type parameter, a Set of Strings can compare using &str (a proper borrowed string slice) rather than &String (double indirection, awkward to use)
pub trait Set<
    Elem,
    BorrowedElem: ?Sized, /* needed since str (our BorrowedElem when Elem=String) is not Sized) */
>
{
    fn is_member(&self, elem: &BorrowedElem) -> bool
    where
        // this means "Elem" can be borrowed as &BorrowedElem
        Elem: Borrow<BorrowedElem>;
}

// You could add where bounds to this, you'll get errors when trying to call is_member if you make an OperatorConstraint with types that "don't work".
// If you add the bounds here, you'll get compile errors as soon as you make try to make the type.
pub struct OperatorConstraint<Elem, Op> {
    op: Op,
    elem: Elem,
}

impl<Elem, Op> OperatorConstraint<Elem, Op> {
    pub fn new(op: Op, elem: Elem) -> Self {
        OperatorConstraint { op, elem }
    }
}

// Operator also needs the whole Elem -> BorrowedElem thing again here.
pub trait Operator<Elem, BorrowedElem: ?Sized>
where
    Elem: Borrow<BorrowedElem>,
{
    // Note we still have references here, BorrowedElem is something like `str`, but we need `&str`
    fn compare(&self, us: &BorrowedElem, them: &BorrowedElem) -> bool;
}

impl<Elem, BorrowedElem: ?Sized, Op> Set<Elem, BorrowedElem> for OperatorConstraint<Elem, Op>
where
    // this means that Operator is implemented for Elem (and BorrowedElem)
    // i.e the Operator knows how to compare the given Elements
    Op: Operator<Elem, BorrowedElem>,

    // this is just the BorrowedElem thing again
    Elem: Borrow<BorrowedElem>,
{
    fn is_member(&self, elem: &BorrowedElem) -> bool
    where
        Elem: Borrow<BorrowedElem>,
    {
        self.op.compare(self.elem.borrow(), elem)
        //      ^^^^^^^           ^^^^^^^^ we have to call borrow so we get the type `&BorrowedElem` that compare() uses
        //      \ compare is from the Operator trait
    }
}

// The "operator" itself is dead simple (to parse!), the behavior is all defined in its impl(s) of Operator :)
pub struct GreaterEqual;

impl<Elem, BorrowedElem: ?Sized> Operator<Elem, BorrowedElem> for GreaterEqual
where
    // the usual BorrowedElem stuff again...
    Elem: Borrow<BorrowedElem>,

    // this means when we borrow Elem, the type we get implements PartialOrd
    BorrowedElem: PartialOrd,
{
    fn compare(&self, us: &BorrowedElem, them: &BorrowedElem) -> bool {
        matches!(
            them.partial_cmp(us),
            Some(Ordering::Greater | Ordering::Equal)
        )
        // matches! is shorthand for
        /*
            match them.partial_cmp(us) {
                Some(Ordering::Greater | Ordering::Equal) => true,
                _anything_else => false
            }
        */
        // it's awesome :)
    }
}

pub struct Contains;

// Here we can impl Contains for String (which we want to borrow as str)
// using concrete types.
//
// What a breath of fresh air! No generic type parameters needed.
impl Operator<String, str> for Contains {
    fn compare(&self, us: &str, them: &str) -> bool {
        // Because we're explicitly dealing with &str we can use all the methods that &str provides. Too easy!
        them.contains(us)
    }
}

pub struct StartsWith;

// Same idea here
impl Operator<String, str> for StartsWith {
    fn compare(&self, us: &str, them: &str) -> bool {
        them.starts_with(us)
    }
}

// And here we can use another Elem, BorrowedElem pair to implement StartsWith for Operator<PathBuf, StartsWith>.
// We might not even have thought of doing this when we made the trait definition, but it's flexible enough to do this!
impl Operator<PathBuf, Path> for StartsWith {
    fn compare(&self, us: &Path, them: &Path) -> bool {
        // Because we're explicitly dealing with &Path we can use all the methods that &Path provides. Too easy!
        them.starts_with(us)
    }
}

#[cfg(test)]
mod tests {
    // these were important, I had everything back to front (us.cmp(them) instead of them.cmp(us)) until these surprised me by failing :D

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
        let set = OperatorConstraint::new(StartsWith, PathBuf::from("/paths/"));
        assert!(!set.is_member(&PathBuf::from("nope")));
        assert!(set.is_member(&PathBuf::from("/paths/start/")));
        assert!(set.is_member(&PathBuf::from("/paths")));
        assert!(set.is_member(&PathBuf::from("/paths/")));
        assert!(!set.is_member(&PathBuf::from("/paths_are_special")));
    }
}
