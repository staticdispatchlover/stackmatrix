# 📐 StackMatrix

A lightweight, const-generic matrix math library in Rust for learning purposes

---

## ✨ Features

- Fixed-size matrices using const generics
- Compile-time shape checking for safe operations
- Matrix addition and subtraction
- Matrix multiplication (naive implementation)
- Pretty printing for debugging
- Benchmark support via Criterion

## Future Improvements
- SIMD-accelerated matmul
- Operator overloading (+, *)
- Transpose and identity matrices

Example:

```rust
use stackgebra::{Matrix, MatrixOps};

fn main() {
    let a: Matrix<2, 3> = Matrix {
        data: [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ],
    };

    let b: Matrix<3, 2> = Matrix {
        data: [
            [7.0, 8.0],
            [9.0, 10.0],
            [11.0, 12.0],
        ],
    };

    let c = a.matmul(b);

    println!("Result:");
    println!("{c}");
}
```
