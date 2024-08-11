/// trait for sign and unsign types
pub trait Sign {
    fn is_negative(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn abs(&self) -> Self;
    fn signumi32(&self) -> i32;
}
macro_rules! impl_signed_types {
    ($zero_value:expr, $($t:ty)*) => ($(
        impl Sign for $t {
            fn is_positive(&self) -> bool {
                *self > $zero_value
            }

            fn is_negative(&self) -> bool {
                *self < $zero_value
            }

            fn is_zero(&self) -> bool {
                *self == $zero_value
            }

            fn abs(&self) -> $t {
                if self.is_negative() {
                    *self * (self.signumi32() as $t)
                } else {
                    *self
                }
            }

            fn signumi32(&self) -> i32 {
                if *self > $zero_value {
                    1
                } else if *self < $zero_value {
                    -1
                } else {
                    0
                }
            }
        }
        )*
    )
}
macro_rules! impl_unsigned_types {
    ($zero_value:expr, $($t:ty)*) => ($(
        impl Sign for $t {
            fn is_positive(&self) -> bool {
                *self != $zero_value
            }

            fn is_negative(&self) -> bool {
                false
            }

            fn is_zero(&self) -> bool {
                *self == $zero_value
            }

            fn abs(&self) -> $t {
                *self
            }

            fn signumi32(&self) -> i32 {
                if *self > $zero_value {
                    1
                } else {
                    0
                }
            }
        }
        )*
    )
}
impl_signed_types! {0, i8 i16 i32 i64 i128 isize}
impl_unsigned_types! {0, u8 u16 u32 u64 u128 usize}
impl_signed_types! {0.0, f32 f64}
