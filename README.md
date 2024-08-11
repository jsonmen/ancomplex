# ancomplex

Provides easy to use, begginer frendly and python like complex numbers
## Examples
Create complex number
```rust
use ancomplex::*;
       
fn main() {
    let c = complex(1, 2);
    //               \  \
    //                \  \
    //                 \  Imaginary Part of complex number
    //                  \
    //                   \
    //                    Real Part of complex number
}   
```
Math operations
```rust
use ancomplex::*;
       
fn main() {
    let c = complex(1, 2);
    let k = complex(3, 4);

    println!("Sum of two complex numbers: {}", c+k);
    println!("Product of two complex numbers: {}", c*k);
    println!("Sub of two complex numbers: {}", c-k);
    println!("Div of two complex numbers: {}", c/k);
}
```
More complex functions
```rust
use ancomplex::*;

fn main() {
    let c = complex(4, 5);
    let k = complex(6, 7);

    println!("Conjugate of complex number: {}", c.conj());
}
```
Fancy printing
```rust
use ancomplex::*;

fn main() {
    let c = complex(4, 2);

    println!("{}", c); // Output: 4 + 2i
}
```
