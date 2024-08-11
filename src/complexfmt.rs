use crate::complexc::Complex;
use crate::sign_trait::Sign;
use std::fmt::{self, Display};

/// Implement Fancy printing
impl<T: Sign + fmt::Debug + Clone> Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let im = self.imag.clone();
        if im.is_positive() || im.is_zero() {
            write!(f, "{:?} + {:?}i", self.real, im)
        } else if im.is_negative() {
            write!(f, "{:?} - {:?}i", self.real, im.abs())
        } else {
            write! {f, "{:?} {:?}i", self.real, im}
        }
    }
}
