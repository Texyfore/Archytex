
use crate::{matrix, utilities::math::{Matrix, Vec2, Vec3}, vector};




#[test]
fn vector_creation() {
    let v = vector!(1.0, 2.0, 3.0);
    assert_eq!(v.inner, [1.0, 2.0, 3.0]);
}
#[test]
fn vector_equality() {
    let v1 = vector!(1.0, 2.0, 3.0);
    let v2 = vector!(1.0, 2.0, 3.0);
    let v3 = vector!(4.0, 5.0, 6.0);
    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
}
#[test]
fn vector_addition() {
    let v1 = vector!(1.0, 2.0, 3.0);
    let v2 = vector!(4.0, 5.0, 6.0);
    assert_eq!(v1 + v2, vector!(5.0, 7.0, 9.0));
    let mut v1 = v1;
    v1 += v2;
    assert_eq!(v1, vector!(5.0, 7.0, 9.0));
}
#[test]
fn vector_mul() {
    let v1 = vector!(1.0, 2.0, 3.0);
    let v2 = vector!(4.0, 5.0, 6.0);
    assert_eq!(v1 * v2, vector!(4.0, 10.0, 18.0));
    let mut v1 = v1;
    v1 *= v2;
    assert_eq!(v1, vector!(4.0, 10.0, 18.0));
}
#[test]
fn vector_mul_scalar() {
    let v1 = vector!(1.0, 2.0, 3.0);
    assert_eq!(v1 * 2.0, vector!(2.0, 4.0, 6.0));
    assert_eq!(2.0 * v1, vector!(2.0, 4.0, 6.0));
    let mut v1 = v1;
    v1 *= 2.0;
    assert_eq!(v1, vector!(2.0, 4.0, 6.0));
}
#[test]
fn vector_dot_product() {
    let v1 = vector!(1.0, 2.0, 3.0);
    let v2 = vector!(4.0, 5.0, 6.0);
    assert_eq!(v1.dot(v2), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
}
#[test]
fn vec3() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = vector!(1.0, 2.0, 3.0);

    assert_eq!(v1, v2);
    assert_eq!(v1.x(), 1.0);
    assert_eq!(v1.y(), 2.0);
    assert_eq!(v1.z(), 3.0);
}
#[test]
fn vec2() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = vector!(1.0, 2.0);

    assert_eq!(v1, v2);
    assert_eq!(v1.x(), 1.0);
    assert_eq!(v1.y(), 2.0);
}
#[test]
fn matrix_equal() {
    let m1 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
    let m2 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
    let m3 = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(10.0, 10.0));
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}
#[test]
fn matrix_indexing() {
    let m = matrix!(vector!(1.0, 2.0), vector!(3.0, 4.0), vector!(5.0, 6.0));
    assert_eq!(m[0], vector!(1.0, 2.0));
    assert_eq!(m[2], vector!(5.0, 6.0));
    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[2][1], 6.0);
}
#[test]
fn matrix_identity() {
    let size_2 = Matrix::<2, 2>::identity();
    assert_eq!(size_2, matrix!(vector!(1.0, 0.0), vector!(0.0, 1.0)));
    let size_3 = Matrix::<3, 3>::identity();
    assert_eq!(
        size_3,
        matrix!(
            vector!(1.0, 0.0, 0.0),
            vector!(0.0, 1.0, 0.0),
            vector!(0.0, 0.0, 1.0)
        )
    );
}
#[test]
fn matrix_transpose() {
    let m1 = matrix!(vector!(0.0, 1.0), vector!(2.0, 3.0), vector!(4.0, 5.0));
    assert_eq!(
        m1.transpose(),
        matrix!(vector!(0.0, 2.0, 4.0), vector!(1.0, 3.0, 5.0))
    );
    let m2: Matrix<5, 5> = Matrix::identity();
    assert_eq!(m2, m2.transpose());
}
#[test]
fn matrix_vector_mul() {
    let v = vector!(1.0, 2.0, 3.0);
    let m: Matrix<3, 3> = Matrix::identity();
    assert_eq!(m * v, v);
    let m: Matrix<4, 3> = matrix!(
        vector!(1.0, 2.0, 3.0, 4.0),
        vector!(5.0, 6.0, 7.0, 8.0),
        vector!(9.0, 10.0, 11.0, 12.0)
    );
    assert_eq!(m * v, vector!(38.0, 44.0, 50.0, 56.0));
}
#[test]
fn matrix_matrix_mul() {
    let m1 = matrix!(
        vector!(1.0, 2.0, 3.0),
        vector!(4.0, 5.0, 6.0),
        vector!(7.0, 8.0, 9.0)
    );
    let m2: Matrix<3, 3> = Matrix::identity();
    assert_eq!(m2 * m1, m1);

    let m1 = matrix!(vector!(1.0, 2.0), vector!(5.0, 6.0), vector!(9.0, 10.0));
    let m2 = matrix!(vector!(1.0, 2.0, 3.0), vector!(6.0, 7.0, 8.0));
    let product = m1 * m2;
    assert_eq!(product[0], vector!(13.0, 16.0, 19.0));
    assert_eq!(product[1], vector!(41.0, 52.0, 63.0));
}