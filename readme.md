# s2dm

Static 2-Dimensional Matrices

## About

This is just a silly project I slapped together in an afternoon to help me learn Rust.

This library exposes a struct called `Matrix<M, N>`, which can be created using various shortcuts like `Vector3` or `Matrix2x3`, or even common structures used in computer graphics like `Quaternion` or `Transform`.

## Usage

To create a 4-by-4 matrix, you can use the `Matrix4` struct:

```rust
let m4 = Matrix4::new(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
);
```

If you want to quickly create a vector, there are some macros for that:

```rust
let v3 = vec3!(1.0, 2.0, 3.0);
```

You can convert a higher-dimensional vector quickly too:

```rust
let v4 = vec4!(1.0, 2.0, 3.0, 4.0);
let v2 = v4.xy();
assert_eq!(v2, vec2!(1.0, 2.0));
```

Getting an individual value is easy. In fact, we can choose whether we want to get column-first or row-first:

```rust
let transform = Transform::new(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
);
let a = transform.row(2).col(1);
assert_eq!(a, 7.0);
let b = transform.col(1).row(2);
assert_eq!(b, 10.0);
```

The library allows you to be open-ended in your approach to performing operations on matrices. For example, you might choose to get the dot product of a Vector2 and Vector3 like this:

```rust
// Approach A: use the `dot` method
let result = v2.dot(v3.xy());
// Approach B: transpose the vector and use matrix multiplication
let result = v2 * v3.xy().transpose();
```
