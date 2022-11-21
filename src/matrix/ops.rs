use std::ops::{Add, Mul, Sub};

use super::{
    types::{ColVector, RowVector, Vector},
    Matrix,
};

impl<const M: usize, const N: usize> Add for Matrix<M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            data: [[0.0; N]; M],
        };
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl<const M: usize, const N: usize> Sub for Matrix<M, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            data: [[0.0; N]; M],
        };
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl<const M: usize, const N: usize> Mul<f64> for Matrix<M, N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = Self {
            data: [[0.0; N]; M],
        };
        for i in 0..M {
            for j in 0..N {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }
}

impl<const M: usize, const N: usize> Mul<Matrix<N, M>> for Matrix<M, N> {
    type Output = Matrix<M, M>;

    fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut result = Matrix::<M, M>::zeroes();

        for i in 0..M {
            for j in 0..M {
                for k in 0..N {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        result
    }
}

impl<const M: usize, const N: usize> Eq for Matrix<M, N> {}
impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn row(&self, index: usize) -> RowVector<N> {
        Matrix {
            data: [self.data[index]],
        }
    }
    pub fn col(&self, index: usize) -> ColVector<M> {
        self.transpose().row(index).transpose()
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut result = Matrix::zeroes();
        for i in 0..M {
            for j in 0..N {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

impl<const M: usize> Vector<M> {
    pub fn scalar(&self) -> RowVector<M> {
        self.transpose()
    }
    pub fn dot(&self, other: Self) -> f64 {
        (self.scalar() * other).into()
    }
}
