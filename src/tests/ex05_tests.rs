use crate::{Vector, angle_cos, tests::test_utils::floats_are_close};

#[test]
fn cosine_tests() {
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    assert_eq!(floats_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    assert_eq!(floats_are_close(angle_cos(&u, &v), 0.0), true);
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![ 1., -1.]);
    assert_eq!(floats_are_close(angle_cos(&u, &v), -1.0), true);
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    assert_eq!(floats_are_close(angle_cos(&u, &v), 1.0), true);
    // 1.0
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    assert_eq!(floats_are_close(angle_cos(&u, &v), 0.974631846), true);
    // 0.974631846

    // undefined
    let u = Vector::from(vec![1., 2.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));

    // NaN
    let u = Vector::from(vec![]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));

    let u = Vector::from(vec![1.]);
    let v = Vector::from(vec![]);
    println!("{}", angle_cos(&u, &v));

}