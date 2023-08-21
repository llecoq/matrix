use crate::{Matrix, tests::test_utils::{assert_output, matrix_to_string}};

#[test]
fn row_echelon_tests() {
    let u = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
        ]).unwrap();
        let result: Vec<Vec<f32>> = vec![
            vec![1., 0., 0.],
            vec![0., 1., 0.],
            vec![0., 0., 1.],
        ];
        assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
            // [1.0, 0.0, 0.0]
            // [0.0, 1.0, 0.0]
            // [0.0, 0.0, 1.0]

        let u = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
        ]).unwrap();
        let result: Vec<Vec<f32>> = vec![
            vec![1., 0.],
            vec![0., 1.],
        ];
        assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
        // [1.0, 0.0]
        // [0.0, 1.0]
      
        let u = Matrix::from(vec![
        vec![1., 2.],
        vec![2., 4.],
        ]).unwrap();
        let result: Vec<Vec<f32>> = vec![
            vec![1., 2.],
            vec![0., 0.],
        ];
        assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
        // [1.0, 2.0]
        // [0.0, 0.0]

        let u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
        ]).unwrap();
        let result: Vec<Vec<f32>> = vec![
            vec![1., 0.6, 0.0, 0.0, -12.2],
            vec![0.0, 0.0, 1.0, 0.0, -3.7],
            vec![0.0, 0.0, 0.0, 1.0, 29.5],
        ];
        assert_output(&u.row_echelon(), matrix_to_string(&result).as_str());
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}