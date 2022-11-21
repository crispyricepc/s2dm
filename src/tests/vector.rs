use crate::{vec2, vec3, vec4};

#[test]
fn xy() {
    let v2 = vec2!(1.0, 2.0);
    assert_eq!(v2.xy(), v2);
    let v3 = vec3!(1.0, 2.0, 3.0);
    assert_eq!(v3.xy(), v2);
    let v4 = vec4!(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v4.xy(), v2);
}

#[test]
fn xyz() {
    let v3 = vec3!(1.0, 2.0, 3.0);
    assert_eq!(v3.xyz(), v3);
    let v4 = vec4!(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v4.xyz(), v3);
}

#[test]
fn xyzw() {
    let v4 = vec4!(1.0, 2.0, 3.0, 4.0);
    assert_eq!(v4.xyzw(), v4);
}

#[test]
fn dot() {
    let a = vec2!(1.0, 2.0);
    let b = vec2!(3.0, 4.0);

    assert_eq!(a.dot(b), 11.0);

    let a = vec3!(1.0, 2.0, 3.0);
    let b = vec3!(4.0, 5.0, 6.0);
    assert_eq!(a.dot(b), 32.0);
}
