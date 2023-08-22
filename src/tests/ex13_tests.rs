use crate::Matrix;

#[test]
fn rank_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]).unwrap();
    println!("{}", u.rank());
    // 3
    let u = Matrix::from(vec![
        vec![ 1., 2., 0., 0.],
        vec![ 2., 4., 0., 0.],
        vec![-1., 2., 1., 1.],
    ]).unwrap();
    println!("{}", u.rank());
    // 2
    let u = Matrix::from(vec![
        vec![ 8., 5., -2.],
        vec![ 4., 7., 20.],
        vec![ 7., 6., 1.],
        vec![21., 18., 7.],
    ]).unwrap();
    println!("{}", u.rank());
    // 3

}