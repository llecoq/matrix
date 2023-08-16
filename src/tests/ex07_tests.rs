use crate::{Matrix, Vector, tests::test_utils::assert_output};

#[test]
fn mul_vec_tests() {
    let u = Matrix::from(vec![
        vec![1., 0.],
        vec![0., 1.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][2.0]");
    // [4.]
    // [2.]
    let u = Matrix::from(vec![
        vec![2., 0.],
        vec![0., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[8.0][4.0]");
    // [8.]
    // [4.]
    let u = Matrix::from(vec![
        vec![2., -2.],
        vec![-2., 2.],
    ]).unwrap();
    let v = Vector::from(vec![4., 2.]);
    assert_output(&u.mul_vec(&v), "[4.0][-4.0]");
    // [4.]
    // [-4.]
}