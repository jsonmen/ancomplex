use crate::Complex;
use num_traits::{Num, Zero};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

impl<T: Num> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T: Num> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl<T: Num + Clone> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Self {
            real: self.real.clone() * rhs.real.clone() - self.imag.clone() * rhs.imag.clone(),
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl<T: Num + Clone> Div<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let sqs = rhs.real.clone() * rhs.real.clone() + rhs.imag.clone() * rhs.imag.clone();
        Self {
            real: (self.real.clone() * rhs.real.clone() + self.imag.clone() * rhs.imag.clone())
                / sqs.clone(),
            imag: (self.imag * rhs.real - self.real * rhs.imag) / sqs,
        }
    }
}

impl<T: Num + Clone> Rem<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn rem(self, rhs: Complex<T>) -> Complex<T> {
        let gausian_int = self.gausian_integer(rhs.clone());
        self - rhs * gausian_int
    }
}

impl<T: Num + Clone> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Self {
            real: T::zero() - self.real.clone(),
            imag: T::zero() - self.imag.clone(),
        }
    }
}
impl<'a, T: Num + Clone> Neg for &'a Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}
impl<T: Num + Clone> Add<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn add(self, other: T) -> Self::Output {
        Complex {
            real: self.real + other,
            imag: self.imag,
        }
    }
}

impl<T: Num + Clone> Sub<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn sub(self, other: T) -> Self::Output {
        Complex {
            real: self.real - other,
            imag: self.imag,
        }
    }
}

impl<T: Num + Clone> Mul<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn mul(self, other: T) -> Self::Output {
        Complex {
            real: self.real * other.clone(),
            imag: self.imag * other,
        }
    }
}

impl<T: Num + Clone> Div<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn div(self, other: T) -> Self::Output {
        Complex {
            real: self.real / other.clone(),
            imag: self.imag / other,
        }
    }
}

impl<T: Num + Clone> Rem<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn rem(self, other: T) -> Self::Output {
        Complex {
            real: self.real % other.clone(),
            imag: self.imag % other,
        }
    }
}

macro_rules! impl_real_ops {
    (@forward $imp:ident::$method:ident for $($real:ident),*) => (
        impl<'a, T: Num + Clone> $imp<&'a T> for Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.$method(other.clone())
            }
        }
        impl<'a, T: Num + Clone> $imp<T> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                self.clone().$method(other)
            }
        }
        impl<'a, 'b, T: Num + Clone> $imp<&'a T> for &'b Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.clone().$method(other.clone())
            }
        }
        $(
            impl<'a> $imp<&'a Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: &Complex<$real>) -> Complex<$real> {
                    self.$method(other.clone())
                }
            }
            impl<'a> $imp<Complex<$real>> for &'a $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: Complex<$real>) -> Complex<$real> {
                    self.clone().$method(other)
                }
            }
            impl<'a, 'b> $imp<&'a Complex<$real>> for &'b $real {
                type Output = Complex<$real>;

                #[inline]
                fn $method(self, other: &Complex<$real>) -> Complex<$real> {
                    self.clone().$method(other.clone())
                }
            }
        )*
    );
    ($($real:ident),*) => (
        impl_real_ops!(@forward Add::add for $($real),*);
        impl_real_ops!(@forward Sub::sub for $($real),*);
        impl_real_ops!(@forward Mul::mul for $($real),*);
        impl_real_ops!(@forward Div::div for $($real),*);
        impl_real_ops!(@forward Rem::rem for $($real),*);

        $(
            impl Add<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn add(self, other: Complex<$real>) -> Self::Output {
                    Self::Output {real: self + other.real, imag: other.imag}
                }
            }

            impl Sub<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn sub(self, other: Complex<$real>) -> Self::Output  {
                    Self::Output {
                        real: self - other.real, imag: other.imag
                    }
                }
            }

            impl Mul<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn mul(self, other: Complex<$real>) -> Self::Output {
                    Self::Output { real: self * other.real, imag: self * other.imag}
                }
            }

            impl Div<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn div(self, other: Complex<$real>) -> Self::Output {
                    let norm_sqr = other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();
                    Self::Output{real: self * other.real / norm_sqr.clone(),
                                      imag: $real::zero() - self * other.imag / norm_sqr}
                }
            }

            impl Rem<Complex<$real>> for $real {
                type Output = Complex<$real>;

                #[inline]
                fn rem(self, other: Complex<$real>) -> Self::Output {
                    Self::Output{real: self, imag: Self::zero()} % other
                }
            }
        )*
    );
}

impl_real_ops!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
