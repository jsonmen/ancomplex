use crate::{complex, Complex};
use num_traits::{ConstOne, ConstZero, Num, One, Zero};

impl<T: Num + Clone> Zero for Complex<T> {
    fn zero() -> Self {
        complex(Zero::zero(), Zero::zero())
    }
    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
    fn set_zero(&mut self) {
        self.real.set_zero();
        self.imag.set_zero();
    }
}
impl<T: Num + Clone + Copy> One for Complex<T> {
    fn one() -> Self {
        complex(One::one(), One::one())
    }
    fn is_one(&self) -> bool {
        self.real.is_one() && self.imag.is_zero()
    }
    fn set_one(&mut self) {
        self.real.set_one();
        self.imag.set_one();
    }
}

impl<T: Num + Clone + ConstZero> ConstZero for Complex<T> {
    const ZERO: Self = Complex {
        real: T::ZERO,
        imag: T::ZERO,
    };
}

impl<T: Num + Clone + ConstZero + ConstOne + Copy> ConstOne for Complex<T> {
    const ONE: Self = Complex {
        real: T::ONE,
        imag: T::ZERO,
    };
}
