#[macro_export]
macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::matrix::types::Vector4::new([[$x], [$y], [$z], [$w]])
    };
}
#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::matrix::types::Vector3::new([[$x], [$y], [$z]])
    };
}
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::matrix::types::Vector2::new([[$x], [$y]])
    };
}

#[macro_export]
macro_rules! rvec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::matrix::types::RowVector4::new([[$x, $y, $z, $w]])
    };
}
#[macro_export]
macro_rules! rvec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::matrix::types::RowVector3::new([[$x, $y, $z]])
    };
}
#[macro_export]
macro_rules! rvec2 {
    ($x:expr, $y:expr) => {
        $crate::matrix::types::RowVector2::new([[$x, $y]])
    };
}
