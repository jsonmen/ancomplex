//!Provides easy to use, begginer frendly and python like complex numbers
//!## Examples
//!Create complex number
//! ```rust
//! use ancomplex::*;
//!        
//! fn main() {
//!     let c = complex(1, 2);
//!     //               \  \
//!     //                \  \
//!     //                 \  Imaginary Part of complex number
//!     //                  \
//!     //                   \
//!     //                    Real Part of complex number
//! }   
//! ```
//!Math operations
//! ```rust
//! use ancomplex::*;
//!        
//! fn main() {
//!     let c = complex(1, 2);
//!     let k = complex(3, 4);
//!
//!     println!("Sum of two complex numbers: {}", c+k);
//!     println!("Product of two complex numbers: {}", c*k);
//!     println!("Sub of two complex numbers: {}", c-k);
//!     println!("Div of two complex numbers: {}", c/k);
//! }
//! ```
//!More complex functions
//! ```rust
//! use ancomplex::*;
//!
//! fn main() {
//!     let c = complex(4, 5);
//!     let k = complex(6, 7);
//!
//!     println!("Conjugate of complex number: {}", c.conj());
//! }
//! ```
//!Fancy printing
//! ```rust
//! use ancomplex::*;
//!
//! fn main() {
//!     let c = complex(4, 2);
//!
//!     println!("{}", c); // Output: 4+2i
//! }
//! ```
//!
#[cfg(test)]
mod tests;

extern crate num_traits;

pub(crate) mod cast;
pub(crate) mod complex_assing_ops;
pub(crate) mod complex_ops;
pub(crate) mod complexc;
pub(crate) mod complexfmt;
pub(crate) mod num_traits_impl;
pub mod traits;
pub use complexc::Complex;
pub use traits::*;

/// Function to init `Complex` struct (*Recomed way to init complex numbers*)
pub fn complex<T>(real: T, imag: T) -> Complex<T> {
    Complex { real, imag }
}
#[allow(non_camel_case_types)]
/// type alias for Float32 Complex number
pub type c32 = Complex<f32>;

#[allow(non_camel_case_types)]
/// type alias for Float64 Complex number
pub type c64 = Complex<f64>;

/// Function for init float32 complex number
pub fn c32<T: Into<f32>>(real: T, imag: T) -> Complex<f32> {
    Complex {
        real: real.into(),
        imag: imag.into(),
    }
}
/// Function for init float64 complex number
pub fn c64<T: Into<f64>>(real: T, imag: T) -> Complex<f64> {
    Complex {
        real: real.into(),
        imag: imag.into(),
    }
}
