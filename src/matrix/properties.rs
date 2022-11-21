use crate::{vec2, vec3, vec4};

use super::{
    types::{Vector2, Vector3, Vector4},
    Matrix,
};

impl<const M: usize> Matrix<M, 1>
where
    [(); M - 2]:,
{
    pub fn x(&self) -> f64 {
        self.data[0][0]
    }
    pub fn y(&self) -> f64 {
        self.data[1][0]
    }
    pub fn xy(&self) -> Vector2 {
        vec2!(self.x(), self.y())
    }
}

impl<const M: usize> Matrix<M, 1>
where
    [(); M - 3]:,
    [(); M - 2]:,
{
    pub fn z(&self) -> f64 {
        self.data[2][0]
    }
    pub fn xyz(&self) -> Vector3 {
        vec3!(self.x(), self.y(), self.z())
    }
    pub fn yz(&self) -> Vector2 {
        vec2!(self.y(), self.z())
    }
}

impl<const M: usize> Matrix<M, 1>
where
    [(); M - 4]:,
    [(); M - 3]:,
    [(); M - 2]:,
{
    pub fn w(&self) -> f64 {
        self.data[3][0]
    }
    pub fn xyzw(&self) -> Vector4 {
        vec4!(self.x(), self.y(), self.z(), self.w())
    }
    pub fn yzw(&self) -> Vector3 {
        vec3!(self.y(), self.z(), self.w())
    }
    pub fn zw(&self) -> Vector2 {
        vec2!(self.z(), self.w())
    }
}
