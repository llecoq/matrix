use crate::Matrix;

#[test]
fn determinant_tests() {
    let u = Matrix::from(vec![
        vec![ 1., -1.],
        vec![-1., 1.],
    ]).unwrap();
    println!("{}", u.determinant());
    // 0.0
    let u = Matrix::from(vec![
        vec![2., 0., 0.],
        vec![0., 2., 0.],
        vec![0., 0., 2.],
    ]).unwrap();
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
    ]).unwrap();
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from(vec![
        vec![ 8., 5., -2., 4.],
        vec![ 4., 2.5, 20., 4.],
        vec![ 8., 5., 1., 4.],
        vec![28., -4., 17., 1.],
    ]).unwrap();
    println!("{}", u.determinant());
    // 1032
}