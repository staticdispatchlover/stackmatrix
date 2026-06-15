use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub data: [[f32; C]; R],
}

pub trait MatrixOps<const R: usize, const C: usize> {
    fn zeros() -> Matrix<R, C>;

    fn get(&self, row: usize, col: usize) -> f32;

    fn set(&mut self, row: usize, col: usize, value: f32);

    fn matmul<const K: usize, const N: usize>(
        &self,
        b: Matrix<K, N>,
    ) -> Matrix<R, N>;

    fn add(&self, b: Matrix<R, C>) -> Matrix<R, C>;

    fn subtract(&self, b: Matrix<R, C>) -> Matrix<R, C>;
}

impl<const R: usize, const C: usize> MatrixOps<R, C> for Matrix<R, C> {
    fn zeros() -> Matrix<R, C> {
        Matrix {
            data: [[0.0; C]; R],
        }
    }

    fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row][col] = value;
    }

    fn matmul<const K: usize, const N: usize>(
        &self,
        b: Matrix<K, N>,
    ) -> Matrix<R, N> {
        assert_eq!(C, K, "Matrix dimensions are incompatible for multiplication");

        let mut result = Matrix::<R, N>::zeros();

        for row in 0..R {
            for col in 0..N {
                let mut sum = 0.0;

                for k in 0..C {
                    sum += self.get(row, k) * b.get(k, col);
                }

                result.set(row, col, sum);
            }
        }

        result
    }

    fn add(&self, b: Matrix<R, C>) -> Matrix<R, C> {
        let mut result = Matrix::<R, C>::zeros();

        for row in 0..R {
            for col in 0..C {
                result.set(
                    row,
                    col,
                    self.get(row, col) + b.get(row, col),
                );
            }
        }

        result
    }

    fn subtract(&self, b: Matrix<R, C>) -> Matrix<R, C> {
        let mut result = Matrix::<R, C>::zeros();

        for row in 0..R {
            for col in 0..C {
                result.set(
                    row,
                    col,
                    self.get(row, col) - b.get(row, col),
                );
            }
        }

        result
    }
}

impl<const R: usize, const C: usize> std::fmt::Display for Matrix<R, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Matrix {}x{}", R, C)?;

        for row in self.data {
            write!(f, "[")?;
            for value in row {
                write!(f, "{:8.3}", value)?;
            }
            write!(f, "  ]\n")?;
        }

        Ok(())
    }
}
