use crate::Matrix;

#[test]
fn row_echelon_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
        ]).unwrap();
        println!("{}", u.row_echelon());
        // [1.0, 0.0, 0.0]
        // [0.0, 1.0, 0.0]
        // [0.0, 0.0, 1.0]
        let u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
        ]).unwrap();
        println!("{}", u.row_echelon());
        // [1.0, 0.0]
        // [0.0, 1.0]
        let u = Matrix::from(vec![
        vec![1., 2.],
        vec![2., 4.],
        ]).unwrap();
        println!("{}", u.row_echelon());
        // [1.0, 2.0]
        // [0.0, 0.0]
        let u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
        ]).unwrap();
        println!("{}", u.row_echelon());
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}