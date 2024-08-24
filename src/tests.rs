#![cfg(test)]

use crate::*;
use std::f64::consts::FRAC_PI_2;
#[test]
fn complex_add_complex() {
    let c = complex(1.0, 4.0);
    let z = complex(17.0, 18.0);
    let correct_result = complex(18., 22.);
    assert_eq!(c + z, correct_result)
}

#[test]
fn complex_sub_complex() {
    let c = complex(1.0, 4.0);
    let z = complex(17.0, 18.0);
    let correct_result = complex(-16., -14.);
    assert_eq!(c - z, correct_result)
}

#[test]
fn complex_mul_complex() {
    let c = complex(1.0, 4.0);
    let z = complex(17.0, 18.0);
    let correct_result = complex(-55., 86.);
    assert_eq!(c * z, correct_result)
}
#[test]
fn complex_div_complex() {
    let c = complex(5.0, 5.0);
    let z = complex(25.0, 30.0);
    let correct_result = complex(5.5, 0.5);
    assert_eq!(z / c, correct_result)
}
#[test]
fn complex_add_f64() {
    let c = complex(1.0, 4.0);
    let z = 2.;
    let correct_result = complex(3., 4.);
    assert_eq!(c + z, correct_result)
}

#[test]
fn complex_sub_f64() {
    let c = complex(1.0, 4.0);
    let z = 8.;
    let correct_result = complex(-7., 4.);
    assert_eq!(c - z, correct_result)
}

#[test]
fn complex_mul_f64() {
    let c = complex(1.0, 4.0);
    let z = 12.;
    let correct_result = complex(12., 48.);
    assert_eq!(c * z, correct_result)
}
#[test]
fn complex_div_f64() {
    let c = complex(5.0, 7.5);
    let z = 2.5;
    let correct_result = complex(2., 3.);
    assert_eq!(c / z, correct_result)
}
#[test]
fn f64_add_complex() {
    let c = complex(1.0, 4.0);
    let z = 2.;
    let correct_result = complex(3., 4.);
    assert_eq!(z + c, correct_result)
}

#[test]
fn f64_sub_complex() {
    let c = complex(1.0, 4.0);
    let z = 8.;
    let correct_result = complex(7., 4.);
    assert_eq!(z - c, correct_result)
}

#[test]
fn f64_mul_complex() {
    let c = complex(1.0, 4.0);
    let z = -12.;
    let correct_result = complex(-12., -48.);
    assert_eq!(z * c, correct_result)
}
#[test]
fn f64_div_complex() {
    let c = complex(5.0, 10.0);
    let z = 2.5;
    let correct_result = complex(0.1, -0.2);
    assert_eq!(z / c, correct_result)
}

#[test]
fn sin_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.sin(), correct_result);
}

#[test]
fn cos_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(1.0, -0.0);
    assert_eq!(c.cos(), correct_result);
}

#[test]
fn tan_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.tan(), correct_result);
}

#[test]
fn asin_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, -0.0);
    assert_eq!(c.asin(), correct_result);
}

#[test]
fn acos_of_complex() {
    let c = complex(1.0, 0.0);
    let correct_result = complex(0.0, -0.0);
    assert_eq!(c.acos(), correct_result);
}
#[test]
fn atan_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.atan(), correct_result);
}

#[test]
fn sinh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.sinh(), correct_result);
}

#[test]
fn cosh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(1.0, 0.0);
    assert_eq!(c.cosh(), correct_result);
}

#[test]
fn tanh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.tanh(), correct_result);
}

#[test]
fn asinh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.asinh(), correct_result);
}

#[test]
fn acosh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.acosh(), correct_result);
}

#[test]
fn atanh_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.atanh(), correct_result);
}
#[test]
fn conj_of_complex() {
    let c = complex(5.2, -6.8);
    let correct_result = complex(5.2, 6.8);
    assert_eq!(c.conj(), correct_result);
}
#[test]
fn square_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.square(), correct_result);
}
#[test]
fn l1_norm_of_complex() {
    let c = complex(2.0, -2.0);
    let correct_result = 4.;
    assert_eq!(c.l1_norm(), correct_result);
}
#[test]
fn norm_of_complex() {
    let c = complex(2.0, 0.0);
    let correct_result = 2.;
    assert_eq!(c.norm(), correct_result);
}
#[test]
fn square_norm_of_complex() {
    let c = complex(2.0, 2.0);
    let correct_result = 8.;
    assert_eq!(c.square_norm(), correct_result);
}
#[test]
fn arg_of_complex() {
    let c = complex(0.0, 1.0);
    let correct_result = FRAC_PI_2;
    assert_eq!(c.arg(), correct_result);
}

#[test]
fn powc_of_complex() {
    let c = complex(1.0, 0.0);
    let z = complex(1.0, 0.0);
    let correct_result = complex(1.0, 0.0);
    println!("{}", correct_result);
    assert_eq!(c.powc(z), correct_result);
}
#[test]
fn powf_of_complex() {
    let c = complex(0.0, 0.0);
    let z = 4.0;
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.powf(z), correct_result);
}
#[test]
fn powi_of_complex() {
    let c = complex(0.0, 0.0);
    let z = 2;
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.powi(z), correct_result);
}
#[test]
fn exp_of_complex() {
    let c = complex(0.0, 0.0);
    let correct_result = complex(1.0, 0.0);
    assert_eq!(c.exp(), correct_result);
}
#[test]
fn ln_of_complex() {
    let c = complex(1.0, 0.0);
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c.ln(), correct_result);
}

#[test]
fn sqrt_of_complex() {
    let c = complex(1.0, 0.0);
    let correct_result = complex(1.0, 0.0);
    assert_eq!(c.sqrt(), correct_result);
}

#[test]
fn complex_add_assign_complex() {
    let mut c = complex(6.0, 16.0);
    let z = complex(2.0, 8.0);
    c += z;
    let correct_result = complex(8.0, 24.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_sub_assign_complex() {
    let mut c = complex(6.0, 16.0);
    let z = complex(2.0, 8.0);
    c -= z;
    let correct_result = complex(4.0, 8.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_mul_assign_complex() {
    let mut c = complex(6.0, 16.0);
    let z = complex(2.0, 8.0);
    c *= z;
    let correct_result = complex(-116.0, 80.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_div_assign_complex() {
    let mut c = complex(2.0, 2.0);
    let z = complex(2.0, 2.0);
    c /= z;
    let correct_result = complex(1.0, 0.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_rem_assing_complex() {
    let mut c = complex(6.0, 16.0);
    let z = complex(2.0, 8.0);
    c %= z;
    let correct_result = complex(2.0, 0.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_add_assign_f64() {
    let mut c = complex(2.0, 2.0);
    let z = 2.0;
    c += z;
    let correct_result = complex(4.0, 2.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_sub_assign_f64() {
    let mut c = complex(2.0, 2.0);
    let z = 2.0;
    c -= z;
    let correct_result = complex(0.0, 2.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_mul_assign_f64() {
    let mut c = complex(2.0, 2.0);
    let z = 2.0;
    c *= z;
    let correct_result = complex(4.0, 4.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_div_assign_f64() {
    let mut c = complex(2.0, 2.0);
    let z = 2.0;
    c /= z;
    let correct_result = complex(1.0, 1.0);
    assert_eq!(c, correct_result);
}

#[test]
fn complex_rem_assing_f64() {
    let mut c = complex(2.0, 2.0);
    let z = 2.0;
    c %= z;
    let correct_result = complex(0.0, 0.0);
    assert_eq!(c, correct_result);
}
#[test]
fn neg_of_complex() {
    let c = complex(2.0, 2.0);
    let correct_result = complex(-2.0, -2.0);
    assert_eq!(-c, correct_result);
}
