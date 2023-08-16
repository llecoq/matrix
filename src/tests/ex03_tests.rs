use crate::Vector;

#[test]
fn dot_product_tests() {
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(&v), 0.);

    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    assert_eq!(u.dot(&v), 2.);

    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    assert_eq!(u.dot(&v), 9.);

    let u = Vector::from(vec![-1., 6., 3., 9., 0., -9.]);
    let v = Vector::from(vec![3., 2., 6., 3., -6., -1.]);
    assert_eq!(u.dot(&v), 63.);

    let u = Vector::from(vec![3., 9., 3., -9.]);
    let v = Vector::from(vec![6., 3., -6., -1.]);
    assert_eq!(u.dot(&v), 36.);

    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![0., 0.]);
    assert_eq!(u.dot(&v), 0.);

    let u: Vector<f32> = Vector::from(vec![]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), 0.);

    // undefined
    let u: Vector<f32> = Vector::from(vec![1.]);
    let v = Vector::from(vec![]);
    assert_eq!(u.dot(&v), 0.);

    let u: Vector<f32> = Vector::from(vec![]);
    let v = Vector::from(vec![1.]);
    assert_eq!(u.dot(&v), 0.);
}