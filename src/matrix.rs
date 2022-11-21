use std::fmt::Debug;

pub mod macros;
pub mod ops;
pub mod properties;
pub mod types;

pub use types::*;

pub struct Matrix<const M: usize, const N: usize> {
    data: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn zeroes() -> Self {
        Self {
            data: [[0.0; N]; M],
        }
    }

    pub fn identity() -> Self {
        Self {
            data: [[1.0; N]; M],
        }
    }

    pub fn with(value: f64) -> Self {
        Self {
            data: [[value; N]; M],
        }
    }

    pub fn new(data: [[f64; N]; M]) -> Self {
        Self { data }
    }
}

impl Into<f64> for Matrix<1, 1> {
    fn into(self) -> f64 {
        self.data[0][0]
    }
}
impl From<f64> for Matrix<1, 1> {
    fn from(value: f64) -> Self {
        Self { data: [[value]] }
    }
}

impl<const M: usize, const N: usize> Debug for Matrix<M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("data", &self.data).finish()
    }
}
