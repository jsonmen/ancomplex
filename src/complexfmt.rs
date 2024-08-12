use crate::complexc::Complex;
use std::fmt::{self, Display};

/// Implement Fancy printing
impl<T: fmt::Debug + Clone> Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.real, self.imag)
    }
}
