use crate::Vector;

#[test]
fn dot_product_tests() {
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(&v));
    // 0.0
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(&v));
    // 2.0
    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("{}", u.dot(&v));
    // 9.0
}