use crate::SetRelation;

pub struct NumberSet {
    subsets: Vec<Box<dyn SetRelation<f64, f64>>>,
}

impl NumberSet {
    pub fn new() -> Self {
        NumberSet {
            subsets: Vec::new(),
        }
    }

    pub fn constrain(&mut self, subsets: impl SetRelation<f64, f64> + 'static) {
        self.subsets.push(Box::new(subsets))
    }
}

impl Default for NumberSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SetRelation<f64, f64> for NumberSet {
    fn is_member(&self, elem: &f64) -> bool {
        self.subsets.iter().all(|subset| (*subset).is_member(elem))
    }
}
