use crate::{complex, Complex};
use num_traits::{AsPrimitive, FromPrimitive, Num, NumCast, ToPrimitive};

macro_rules! impl_toprim {
    ($ty:ty, $tofn:ident) => {
        fn $tofn(&self) -> Option<$ty> {
            if self.imag.is_zero() {
                self.real.$tofn()
            } else {
                None
            }
        }
    };
}
macro_rules! impl_fprim {
    ($ty:ty, $fromfn:ident) => {
        fn $fromfn(n: $ty) -> Option<$ty> {
            Some(complex(T::$fromfn(n)?, T::zero()))
        }
    };
}

impl<T: ToPrimitive + Num> ToPrimitive for Complex<T> {
    impl_toprim!(u8, to_u8);
    impl_toprim!(u16, to_u16);
    impl_toprim!(u32, to_u32);
    impl_toprim!(u64, to_u64);
    impl_toprim!(u128, to_u128);
    impl_toprim!(usize, to_usize);

    impl_toprim!(i8, to_i8);
    impl_toprim!(i16, to_i16);
    impl_toprim!(i32, to_i32);
    impl_toprim!(i64, to_i64);
    impl_toprim!(i128, to_i128);
    impl_toprim!(isize, to_isize);

    impl_toprim!(f32, to_f32);
    impl_toprim!(f64, to_f64);
}

impl<T: FromPrimitive + Num> FromPrimitive for Complex<T> {
    impl_fprim!(u8, from_u8);
    impl_fprim!(u16, from_u16);
    impl_fprim!(u32, from_u32);
    impl_fprim!(u64, from_u64);
    impl_fprim!(u128, from_u128);
    impl_fprim!(usize, from_usize);

    impl_fprim!(i8, from_i8);
    impl_fprim!(i16, from_i16);
    impl_fprim!(i32, from_i32);
    impl_fprim!(i64, from_i64);
    impl_fprim!(i128, from_i128);
    impl_fprim!(isize, from_isize);

    impl_fprim!(f32, from_f32);
    impl_fprim!(f64, from_f64);
}

impl<T: NumCast + Num> NumCast for Complex<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        Some(complex(T::from(n)?, T::zero()))
    }
}

impl<T, U> AsPrimitive<U> for Complex<T>
where
    T: AsPrimitive<U>,
    U: 'static + Copy,
{
    fn as_(self) -> U {
        self.real.as_()
    }
}
