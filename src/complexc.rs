use num_traits::{Float, FloatConst, NumOps, One, Zero};
use std::ops::Neg;

/// struct for Complex numbers (a+bi)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Complex<T> {
    /// real part of complex number
    pub real: T,
    /// imaginary part of complex number
    pub imag: T,
}

impl<T: NumOps + One + Zero + Neg<Output = T> + Clone> Complex<T> {
    pub fn conj(&self) -> Self {
        Self {
            real: self.real.clone(),
            imag: -self.imag.clone(),
        }
    }

    /// Function for find square norm of complex number.
    /// Formula: $z^2 = a^2 + b^2$
    pub fn square_norm(&self) -> T {
        self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()
    }

    pub fn i() -> Self {
        Complex {
            real: T::zero(),
            imag: T::one(),
        }
    }
}
impl<T: Float> Complex<T> {
    pub fn to_polar(&self) -> (T, T) {
        (self.norm(), self.arg())
    }
    pub fn from_polar(radius: T, theta: T) -> Self {
        Complex {
            real: radius * theta.cos(),
            imag: radius * theta.sin(),
        }
    }

    pub fn powc(&self, power: Complex<T>) -> Self {
        let (r, theta) = self.to_polar();
        let rhs = Complex {
            real: r.ln(),
            imag: theta,
        };
        (power * rhs).exp()
    }
    pub fn powi(&self, power: i32) -> Self {
        let (r, theta) = self.to_polar();
        Self::from_polar(r.powi(power), theta * T::from(power).expect("NaN"))
    }
    pub fn powf(&self, power: T) -> Self {
        let (r, theta) = self.to_polar();
        Self::from_polar(r.powf(power), theta * power)
    }
    pub fn exp(&self) -> Self {
        Self::from_polar(self.real.exp(), self.imag)
    }
    pub fn expf(&self, base: T) -> Self {
        Self::from_polar(base.powf(self.real), self.imag * base.ln())
    }
    pub fn ln(&self) -> Self {
        let (r, theta) = self.to_polar();
        Self::from_polar(r.ln(), theta)
    }
    pub fn log(&self, base: T) -> Self {
        let (r, theta) = self.to_polar();
        Self {
            real: r.log(base),
            imag: theta / base.ln(),
        }
    }

    pub fn square(&self) -> Self {
        self.powi(2)
    }
    pub fn scale(&self, scallar: T) -> Self {
        self * scallar
    }
    pub fn unscale(&self, scallar: T) -> Self {
        self / scallar
    }
    pub fn inv(&self) -> Self {
        Complex {
            real: T::one(),
            imag: T::zero(),
        } / self.clone()
    }

    pub fn l1_norm(&self) -> T {
        self.real.abs() + self.imag.abs()
    }
    pub fn norm(&self) -> T {
        self.real.hypot(self.imag)
    }

    pub fn arg(&self) -> T {
        self.imag.atan2(self.real)
    }
}

impl<T: Float + FloatConst> Complex<T> {
    pub fn exp2(&self) -> Self {
        Self::from_polar(self.real.exp2(), self.imag * T::LN_2())
    }

    pub fn log2(&self) -> Self {
        Self::ln(&self) / T::LN_2()
    }

    pub fn log10(&self) -> Self {
        Self::ln(&self) / T::LN_10()
    }
}
