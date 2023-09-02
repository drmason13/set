//! Operators for comparing via "containing" relationships such as contains and starts_with

use crate::Operator;
use std::path::{Path, PathBuf};

pub struct Contains;

impl Operator<String, str> for Contains {
    fn compare(&self, a: &str, b: &str) -> bool {
        a.contains(b)
    }
}

pub struct StartsWith;

impl Operator<String, str> for StartsWith {
    fn compare(&self, a: &str, b: &str) -> bool {
        a.starts_with(b)
    }
}

impl Operator<PathBuf, Path> for StartsWith {
    fn compare(&self, a: &Path, b: &Path) -> bool {
        a.starts_with(b)
    }
}

pub struct EndsWith;

impl Operator<String, str> for EndsWith {
    fn compare(&self, a: &str, b: &str) -> bool {
        a.ends_with(b)
    }
}

impl Operator<PathBuf, Path> for EndsWith {
    fn compare(&self, a: &Path, b: &Path) -> bool {
        a.ends_with(b)
    }
}

impl Operator<f64, f64> for EndsWith {
    fn compare(&self, a: &f64, b: &f64) -> bool {
        let a = format!("{a}");
        let b = format!("{b}");

        a.as_str().ends_with(b.as_str())
    }
}
