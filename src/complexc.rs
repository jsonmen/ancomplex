use crate::complex;
use num_traits::{Float, FloatConst, Num};

/// struct for Complex numbers (a+bi)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Complex<T> {
    /// real part of complex number
    pub real: T,
    /// imaginary part of complex number
    pub imag: T,
}
impl<T: Num + Clone> Complex<T> {
    /// Function for find square norm of complex number.
    /// Formula: a² + b²
    pub fn square_norm(&self) -> T {
        self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()
    }
    /// Function to complute square (power of 2) of complex number
    /// Formula: (a+bi)² = a² + 2*a*bi - b²
    pub fn square(&self) -> Self {
        let two = T::one() + T::one();
        complex(
            self.real.clone() * self.real.clone() - self.imag.clone() * self.imag.clone(),
            two * self.real.clone() * self.imag.clone(),
        )
    }
    pub fn scale(&self, scallar: T) -> Self {
        self * scallar
    }
    pub fn unscale(&self, scallar: T) -> Self {
        self / scallar
    }
    pub fn inv(&self) -> Self {
        let sqs = self.square_norm();
        Complex {
            real: self.real.clone() / sqs.clone(),
            imag: T::zero() - self.imag.clone() / sqs,
        }
    }
    pub fn conj(&self) -> Self {
        Self {
            real: self.real.clone(),
            imag: T::zero() - self.imag.clone(),
        }
    }
    /// Computes gausian integer for complex number
    pub fn gausian_integer(&self, other: Complex<T>) -> Complex<T> {
        let q = self.clone() / other;
        complex(
            q.real.clone() - q.real % T::one(),
            q.imag.clone() - q.imag % T::one(),
        )
    }
    /// Create complex number only with imaginary number (0+1i)
    pub fn i() -> Self {
        Complex {
            real: T::zero(),
            imag: T::one(),
        }
    }
}

impl<T: Float> Complex<T> {
    pub fn sin(&self) -> Self {
        complex(
            self.real.sin() * self.imag.cosh(),
            self.real.cos() * self.imag.sinh(),
        )
    }
    pub fn cos(&self) -> Self {
        complex(
            self.real.cos() * self.imag.cosh(),
            -self.real.sin() * self.imag.sinh(),
        )
    }
    pub fn tan(&self) -> Self {
        self.sin() / self.cos()
    }
    pub fn asin(&self) -> Self {
        let i = Self::i();
        -i * ((complex(T::one(), T::zero()) - self.clone() * self.clone()).sqrt()
            + self.clone() * i)
            .ln()
    }
    pub fn acos(&self) -> Self {
        let i = Self::i();
        -i * ((self.clone() * self.clone() - complex(T::one(), T::zero())).sqrt() + self.clone())
            .ln()
    }
    pub fn atan(&self) -> Self {
        let i = Self::i();
        let one = complex(T::one(), T::zero());
        let two = one + one;
        let iz = i * self.clone();
        (i / two) * ((one - iz) / (one + iz)).ln()
    }
    pub fn sinh(&self) -> Self {
        complex(
            self.real.sinh() * self.imag.cos(),
            self.real.cosh() * self.imag.sin(),
        )
    }
    pub fn cosh(&self) -> Self {
        complex(
            self.real.cosh() * self.imag.cos(),
            self.real.sinh() * self.imag.sin(),
        )
    }
    pub fn tanh(&self) -> Self {
        let (two_real, two_imag) = (self.real + self.real, self.imag + self.imag);
        complex(two_real.sinh(), two_imag.sin()).unscale(two_real.cosh() + two_imag.cos())
    }

    pub fn asinh(self) -> Self {
        let one = complex(T::one(), T::zero());
        (self + (one + self * self).sqrt()).ln()
    }

    pub fn acosh(self) -> Self {
        let one = complex(T::one(), T::zero());
        let two = one + one;
        two * (((self + one) / two).sqrt() + ((self - one) / two).sqrt()).ln()
    }

    pub fn atanh(self) -> Self {
        let one = complex(T::one(), T::zero());
        let two = one + one;
        if self == one {
            return complex(T::infinity(), T::zero());
        } else if self == -one {
            return complex(-T::infinity(), T::zero());
        }
        ((one + self).ln() - (one - self).ln()) / two
    }
    pub fn to_polar(&self) -> (T, T) {
        (self.norm(), self.arg())
    }
    pub fn from_polar(radius: T, theta: T) -> Self {
        complex(radius * theta.cos(), radius * theta.sin())
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

    pub fn sqrt(&self) -> Self {
        let two = T::one() + T::one();
        let (r, theta) = self.to_polar();
        Self::from_polar(r.sqrt(), theta / two)
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
