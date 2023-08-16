use crate::{Vector, cross_product, tests::test_utils::assert_output};

#[test]
fn cross_product_tests() {
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    assert_output(&cross_product(&u, &v), "[0.0][1.0][0.0]");
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_output(&cross_product(&u, &v), "[-3.0][6.0][-3.0]");
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    assert_output(&cross_product(&u, &v), "[17.0][-58.0][-16.0]");
    // [17.]
    // [-58.]
    // [-16.]
}