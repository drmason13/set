use crate::{Operator, OperatorConstraint, SetRelation};

pub struct NumberSet {
    subsets: Vec<Box<dyn SetRelation<f64, f64>>>,
    constraint: ConstraintKind,
}

pub enum ConstraintKind {
    Intersection,
    Union,
}

impl NumberSet {
    pub fn new() -> Self {
        NumberSet {
            subsets: Vec::new(),
            constraint: ConstraintKind::Intersection,
        }
    }

    pub fn constrain(&mut self, subset: impl SetRelation<f64, f64> + 'static) {
        self.subsets.push(Box::new(subset))
    }

    pub fn intersect(self, other: impl SetRelation<f64, f64> + 'static) -> NumberSet {
        let mut set = NumberSet::new();
        set.constraint = ConstraintKind::Intersection;

        set.constrain(self);
        set.constrain(other);

        set
    }

    pub fn union(self, other: impl SetRelation<f64, f64> + 'static) -> NumberSet {
        let mut set = NumberSet::new();
        set.constraint = ConstraintKind::Union;

        set.constrain(self);
        set.constrain(other);

        set
    }
}

impl<Op> From<OperatorConstraint<f64, Op>> for NumberSet
where
    Op: Operator<f64, f64> + 'static,
{
    fn from(constraint: OperatorConstraint<f64, Op>) -> Self {
        let mut set = NumberSet::new();
        set.constrain(constraint);
        set
    }
}

impl Default for NumberSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SetRelation<f64, f64> for NumberSet {
    fn is_member(&self, elem: &f64) -> bool {
        match self.constraint {
            ConstraintKind::Intersection => {
                self.subsets.iter().all(|subset| (*subset).is_member(elem))
            }
            ConstraintKind::Union => self.subsets.iter().any(|subset| (*subset).is_member(elem)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{EndsWith, GreaterEqual, Less, OperatorConstraint};

    use super::*;

    #[test]
    fn number_set_union_constraint_operator() {
        let gte5 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 5_f64));
        let lt10 = OperatorConstraint::new(Less, 10_f64);

        let union = gte5.union(lt10);

        assert!(union.is_member(&8_f64));
        assert!(union.is_member(&5_f64));

        // 3 is lt 10
        assert!(union.is_member(&3_f64));

        // 13 is gte 5
        assert!(union.is_member(&13_f64));
    }

    #[test]
    fn union() {
        let gte5 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 5_f64));
        let lt10 = NumberSet::from(OperatorConstraint::new(Less, 10_f64));

        let union = gte5.union(lt10);

        assert!(union.is_member(&8_f64));
        assert!(union.is_member(&5_f64));

        // 3 is lt 10
        assert!(union.is_member(&3_f64));

        // 13 is gte 5
        assert!(union.is_member(&13_f64));
    }

    #[test]
    fn union_of_intersection() {
        let gte5_and_lt10 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 5_f64))
            .intersect(OperatorConstraint::new(Less, 10_f64));

        let gte15_and_lt20 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 15_f64))
            .intersect(OperatorConstraint::new(Less, 20_f64));

        let set = gte5_and_lt10.union(gte15_and_lt20);

        // now set "contains" numbers within two distinct regions:
        // 5 <= x < 10
        // 15 <= x < 20

        assert!(set.is_member(&8_f64));
        assert!(set.is_member(&5_f64));

        assert!(!set.is_member(&3_f64));
        assert!(!set.is_member(&13_f64));

        assert!(set.is_member(&18_f64));
        assert!(set.is_member(&15_f64));
    }

    #[test]
    fn union_of_union_of_intersection() {
        let gte5_and_lt10 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 5_f64))
            .intersect(OperatorConstraint::new(Less, 10_f64));

        let gte15_and_lt20 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 15_f64))
            .intersect(OperatorConstraint::new(Less, 20_f64));

        let gte25_and_lt30 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 25_f64))
            .intersect(OperatorConstraint::new(Less, 30_f64));

        let set = gte5_and_lt10.union(gte15_and_lt20).union(gte25_and_lt30);

        // now set "contains" numbers within 3 distinct regions:
        // 5 <= x < 10
        // 15 <= x < 20
        // 25 <= x < 30

        assert!(set.is_member(&8_f64));
        assert!(set.is_member(&5_f64));

        assert!(!set.is_member(&3_f64));
        assert!(!set.is_member(&13_f64));

        assert!(set.is_member(&18_f64));
        assert!(set.is_member(&15_f64));

        assert!(!set.is_member(&23_f64));
        assert!(!set.is_member(&33_f64));

        assert!(set.is_member(&28_f64));
        assert!(set.is_member(&25_f64));
    }

    #[test]
    fn intersection_of_union_of_intersection() {
        let gte5_and_lt10 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 5_f64))
            .intersect(OperatorConstraint::new(Less, 10_f64));

        let gte15_and_lt20 = NumberSet::from(OperatorConstraint::new(GreaterEqual, 15_f64))
            .intersect(OperatorConstraint::new(Less, 99_f64));

        let ends7 = OperatorConstraint::new(EndsWith, 3_f64);

        let set = gte5_and_lt10.union(gte15_and_lt20).intersect(ends7);

        // now set "contains" numbers within 2 distinct regions:
        // 5 <= x < 10
        // 15 <= x < 99
        // AND they have to "end in 3"

        assert!(!set.is_member(&8_f64));
        assert!(!set.is_member(&5_f64));

        assert!(!set.is_member(&3_f64));
        assert!(!set.is_member(&13_f64));

        assert!(!set.is_member(&15_f64));
        assert!(!set.is_member(&20_f64));
        assert!(!set.is_member(&21_f64));
        assert!(!set.is_member(&22_f64));
        assert!(set.is_member(&23_f64));
        assert!(!set.is_member(&24_f64));
        assert!(!set.is_member(&25_f64));
        assert!(!set.is_member(&26_f64));
        assert!(!set.is_member(&27_f64));
        assert!(!set.is_member(&28_f64));

        assert!(set.is_member(&33_f64));
        assert!(set.is_member(&43_f64));
        // ...
        assert!(set.is_member(&83_f64));
        assert!(set.is_member(&93_f64));

        assert!(!set.is_member(&103_f64));
    }
}
