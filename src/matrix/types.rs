use super::Matrix;

pub type Matrix4x4 = Matrix<4, 4>;
pub type Matrix4x3 = Matrix<4, 3>;
pub type Matrix4x2 = Matrix<4, 2>;
pub type Matrix4 = Matrix4x4;
pub type Matrix3x4 = Matrix<3, 4>;
pub type Matrix3x3 = Matrix<3, 3>;
pub type Matrix3x2 = Matrix<3, 2>;
pub type Matrix3 = Matrix3x3;
pub type Matrix2x4 = Matrix<2, 4>;
pub type Matrix2x3 = Matrix<2, 3>;
pub type Matrix2x2 = Matrix<2, 2>;
pub type Matrix2 = Matrix2x2;

pub type Vector<const M: usize> = Matrix<M, 1>;

pub type Vector4 = Vector<4>;
pub type Vector3 = Vector<3>;
pub type Vector2 = Vector<2>;

pub type RowVector<const N: usize> = Matrix<1, N>;
pub type RowVector4 = RowVector<4>;
pub type RowVector3 = RowVector<3>;
pub type RowVector2 = RowVector<2>;

pub type ColVector<const M: usize> = Vector<M>;
pub type ColVector4 = ColVector<4>;
pub type ColVector3 = ColVector<3>;
pub type ColVector2 = ColVector<2>;

pub type Transform = Matrix4;
pub type Point = Vector4;
pub type Scalar<const N: usize> = RowVector<N>;
pub type Scalar4 = Scalar<4>;
pub type Scalar3 = Scalar<3>;
pub type Scalar2 = Scalar<2>;
pub type Quaternion = Vector4;
