use std::{iter::Sum, ops::Mul};

/// Represents a mathematical matrix along with operations to manipulate
/// them and more specifically to transform points.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize>([[T; COLS]; ROWS]);

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Constructs a new Matrix, taking ownership of the given array.
    pub const fn new(array: [[T; COLS]; ROWS]) -> Self {
        Self(array)
    }
}

impl<
        T: Copy + Default + Mul + Sum<<T as Mul>::Output>,
        const ROWS_A: usize,
        const COLS_A: usize,
        const COLS_B: usize,
    > Mul<Matrix<T, COLS_A, COLS_B>> for Matrix<T, ROWS_A, COLS_A>
{
    type Output = Matrix<T, ROWS_A, COLS_B>;

    fn mul(self, rhs: Matrix<T, COLS_A, COLS_B>) -> Self::Output {
        &self * &rhs
    }
}

impl<
        T: Copy + Default + Mul + Sum<<T as Mul>::Output>,
        const ROWS_A: usize,
        const COLS_A: usize,
        const COLS_B: usize,
    > Mul<&Matrix<T, COLS_A, COLS_B>> for &Matrix<T, ROWS_A, COLS_A>
{
    type Output = Matrix<T, ROWS_A, COLS_B>;

    fn mul(self, rhs: &Matrix<T, COLS_A, COLS_B>) -> Self::Output {
        let mut result = Matrix::new([[T::default(); COLS_B]; ROWS_A]);
        (0..ROWS_A).for_each(|row| {
            (0..COLS_B).for_each(|col| {
                let row_iter = self
                    .0
                    .iter()
                    .skip(row)
                    .take(1)
                    .flat_map(|row| row.iter().copied());
                let col_iter = (0..COLS_A).map(|idx| rhs.0[idx][col]);
                result.0[row][col] = row_iter.zip(col_iter).map(|(row, col)| row * col).sum();
            });
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        assert_eq!(Matrix([[2.0; 3]; 3]), Matrix::new([[2.0; 3]; 3]));
    }

    #[test]
    fn test_matrix_multiplication() {
        assert_eq!(
            Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]) * Matrix::new([[0], [10], [0]]),
            Matrix::new([[0], [10], [0]])
        );
        assert_eq!(
            Matrix::new([[0, -1, 2], [4, 11, 2]]) * Matrix::new([[3, -1], [1, 2], [6, 1]]),
            Matrix::new([[11, 0], [35, 20]])
        );
        assert_eq!(
            Matrix::new([[8, 9], [5, -1]]) * Matrix::new([[-2, 3], [4, 0]]),
            Matrix::new([[20, 24], [-14, 15]])
        );
        assert_eq!(
            Matrix::new([[0, -1, 2], [4, 11, 2]]) * Matrix::new([[3, -1], [1, 2], [6, 1]]),
            Matrix::new([[11, 0], [35, 20]])
        );
        assert_eq!(
            Matrix::new([[3, -1], [1, 2], [6, 1]]) * Matrix::new([[0, -1, 2], [4, 11, 2]]),
            Matrix::new([[-4, -14, 4], [8, 21, 6], [4, 5, 14]])
        );
    }

    #[test]
    fn test_matrix_multiplication_references() {
        assert_eq!(
            &Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]) * &Matrix::new([[0], [10], [0]]),
            Matrix::new([[0], [10], [0]])
        );
        assert_eq!(
            &Matrix::new([[0, -1, 2], [4, 11, 2]]) * &Matrix::new([[3, -1], [1, 2], [6, 1]]),
            Matrix::new([[11, 0], [35, 20]])
        );
        assert_eq!(
            &Matrix::new([[8, 9], [5, -1]]) * &Matrix::new([[-2, 3], [4, 0]]),
            Matrix::new([[20, 24], [-14, 15]])
        );
        assert_eq!(
            &Matrix::new([[0, -1, 2], [4, 11, 2]]) * &Matrix::new([[3, -1], [1, 2], [6, 1]]),
            Matrix::new([[11, 0], [35, 20]])
        );
        assert_eq!(
            &Matrix::new([[3, -1], [1, 2], [6, 1]]) * &Matrix::new([[0, -1, 2], [4, 11, 2]]),
            Matrix::new([[-4, -14, 4], [8, 21, 6], [4, 5, 14]])
        );
    }
}
