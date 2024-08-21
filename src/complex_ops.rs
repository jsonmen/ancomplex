use crate::{complex, Complex};
use num_traits::{Num, NumAssign, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

impl<T: Num + Clone> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Self {
            real: self.real.clone() + rhs.real.clone(),
            imag: self.imag.clone() + rhs.imag.clone(),
        }
    }
}
impl<T: NumAssign> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
impl<T: Num + Clone> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Self {
            real: self.real.clone() - rhs.real.clone(),
            imag: self.imag.clone() - rhs.imag.clone(),
        }
    }
}
impl<T: NumAssign> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self, rhs: Complex<T>) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}
impl<T: Num + Clone + Copy> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        let (a, b, c, d) = (self.real, self.imag, rhs.real, rhs.imag);
        Self {
            real: a * c - b * d,
            imag: a * d + b * c,
        }
    }
}
impl<T: NumAssign + Copy + Clone> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self, rhs: Complex<T>) {
        let (a, b, c, d) = (self.real, self.imag, rhs.real, rhs.imag);
        self.real = a * c - b * d;
        self.imag = a * d + d * c;
    }
}

impl<T: Num + Clone + Copy> Div<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let (a, b, c, d) = (self.real, self.imag, rhs.real, rhs.imag);
        let sqs = c * c + d * d;
        Self {
            real: (a * c + b * d) / sqs,
            imag: (b * c - a * d) / sqs,
        }
    }
}
impl<T: NumAssign + Copy + Clone> DivAssign<Complex<T>> for Complex<T> {
    fn div_assign(&mut self, rhs: Complex<T>) {
        let (a, b, c, d) = (self.real, self.imag, rhs.real, rhs.imag);
        let sqs = c * c + d * d;
        self.real = (a * c + b * d) / sqs;
        self.imag = (b * c - a * d) / sqs;
    }
}
impl<T: Num + Clone + Copy> Rem<Complex<T>> for Complex<T> {
    type Output = Complex<T>;
    fn rem(self, rhs: Complex<T>) -> Complex<T> {
        let q = self / rhs;
        return self - q * rhs;
    }
}

impl<T: Clone + Neg<Output = T>> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Self {
            real: -self.real.clone(),
            imag: -self.imag.clone(),
        }
    }
}
impl<'a, T: Clone + Num + Neg<Output = T>> Neg for &'a Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}
impl<T: Clone + Num> Add<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn add(self, other: T) -> Self::Output {
        Complex {
            real: self.real + other,
            imag: self.imag,
        }
    }
}

impl<T: Clone + Num> Sub<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn sub(self, other: T) -> Self::Output {
        Complex {
            real: self.real - other,
            imag: self.imag,
        }
    }
}

impl<T: Clone + Num> Mul<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn mul(self, other: T) -> Self::Output {
        Complex {
            real: self.real * other.clone(),
            imag: self.imag * other,
        }
    }
}

impl<T: Clone + Num> Div<T> for Complex<T> {
    type Output = Complex<T>;

    #[inline]
    fn div(self, other: T) -> Self::Output {
        Complex {
            real: self.real / other.clone(),
            imag: self.imag / other,
        }
    }
}

impl<T: Clone + Num> Rem<T> for Complex<T> {
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
        impl<'a, T: Clone + Num> $imp<&'a T> for Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: &T) -> Self::Output {
                self.$method(other.clone())
            }
        }
        impl<'a, T: Clone + Num> $imp<T> for &'a Complex<T> {
            type Output = Complex<T>;

            #[inline]
            fn $method(self, other: T) -> Self::Output {
                self.clone().$method(other)
            }
        }
        impl<'a, 'b, T: Clone + Num> $imp<&'a T> for &'b Complex<T> {
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
                    Self::Output { real: other.real / self, imag: other.imag / self}
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
