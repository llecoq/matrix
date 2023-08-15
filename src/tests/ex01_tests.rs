use crate::{Vector, operations::linear_combination::linear_combination, tests::test_utils::assert_output};

#[test]
fn linear_combination_tests() {
    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);

    assert_output(&linear_combination(&[e1, e2, e3], &[10., -2., 0.5]), "[10.0][-2.0][0.0]");
    assert_output(&linear_combination(&[v1, v2], &[10., -2.]), "[10.0][0.0][230.0]");
}
