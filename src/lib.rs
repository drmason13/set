use std::borrow::Borrow;

pub mod operator;

pub use operator::Operator;
pub use operator::OperatorConstraint;

pub use operator::*;

pub trait SetRelation<Elem, BorrowedElem: ?Sized> {
    fn is_member(&self, elem: &BorrowedElem) -> bool
    where
        Elem: Borrow<BorrowedElem>;
}

mod sets;
pub use sets::*;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn constraint_as_set_works() {
        let set = OperatorConstraint::new(GreaterEqual, 5_u64);
        assert!(set.is_member(&8));
        assert!(set.is_member(&5));
        assert!(!set.is_member(&3));
    }

    #[test]
    fn number_set_works() {
        let mut set = NumberSet::new();
        let gte5 = OperatorConstraint::new(GreaterEqual, 5_f64);
        let lt10 = OperatorConstraint::new(Less, 10_f64);

        set.constrain(gte5);
        set.constrain(lt10);

        assert!(set.is_member(&8_f64));
        assert!(set.is_member(&5_f64));
        assert!(!set.is_member(&3_f64));
        // check it!
        assert!(!set.is_member(&13_f64));
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
