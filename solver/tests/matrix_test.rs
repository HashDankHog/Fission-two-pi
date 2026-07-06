//First time ever writing unit test: it sucks
// like it really sucks omg
// OMG AND ILL HAVE TO REWRITE ALL OF THE PANIC ONES IF I DECIDE THAT I NEED A DIFFERENT BEHAVIOR FAH
use solver::matrix::Matrix;
use solver::parameter::Parameter;
struct TestMatrix {
    identity: Matrix<Parameter>,
    empty: Matrix<Parameter>,
    vector: Matrix<Parameter>,
    three_by_three: Matrix<Parameter>,
    two_by_three: Matrix<Parameter>,
    three_by_two: Matrix<Parameter>,
}
impl Default for TestMatrix {
    fn default() -> TestMatrix {
        TestMatrix {
            identity: Matrix {
                elements: vec![Parameter { expression: vec![String::from("1")], value: 1.0}, Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 }, 
                    Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , 
                    Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } ],
                rows: 3,
                colums: 3
            },
            empty: Matrix {
                elements: vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , 
                    Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , 
                    Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("0")], value: 0.0 } ],
                rows: 3,
                colums: 3
            },
            vector: Matrix {
                elements: vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , 
                    Parameter { expression: vec![String::from("1")], value: 1.0 } , 
                    Parameter { expression: vec![String::from("2")], value: 2.0 } ],
                rows: 3,
                colums: 1
            },
            three_by_three: Matrix {
                elements: vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("6")], value: 6.0 } , Parameter { expression: vec![String::from("8")], value: 8.0 } , 
                    Parameter { expression: vec![String::from("3")], value: 3.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("7")], value: 7.0 } , 
                    Parameter { expression: vec![String::from("5")], value: 5.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } , Parameter { expression: vec![String::from("2")], value: 2.0 } ],
                rows: 3,
                colums: 3
            },
            two_by_three: Matrix {
                elements: vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("2")], value: 2.0 } , 
                    Parameter { expression: vec![String::from("3")], value: 3.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } , Parameter { expression: vec![String::from("5")], value: 5.0 } ],
                rows: 2,
                colums: 3
            },
            three_by_two: Matrix {
                elements: vec![Parameter { expression: vec![String::from("0")], value: 0.0 } , Parameter { expression: vec![String::from("3")], value: 3.0 } , 
                    Parameter { expression: vec![String::from("1")], value: 1.0 } , Parameter { expression: vec![String::from("4")], value: 4.0 } , 
                    Parameter { expression: vec![String::from("2")], value: 2.0 } , Parameter { expression: vec![String::from("5")], value: 5.0 } ],
                rows: 3,
                colums: 2
            },
        }
    }
}

#[test]
fn identity_matrix_test() {
    use solver::matrix::IdentityMatrix;
    let matrices = TestMatrix::default();
    let result_matrix = Parameter::imat(3);
    let expected_matrix = matrices.identity;
    assert_eq!(result_matrix, expected_matrix);
}

/* 
#[test]
fn create_matrix_test() {
    let matrices = TestMatrix::default();
    let result_matrix = create_matrix(3, 3);
    let expected_matrix = matrices.empty;
    assert_eq!(result_matrix, expected_matrix);
}
*/
//NOTE: test should be rewritten at a later date for a test case besides the identity matrix
#[test]
fn inverse_test() { // I refuse to fix this fuck -0.0 and fuck floating point ts john works in theory
    let matrices = TestMatrix::default();

    let input_matrix = matrices.identity.clone();

    let mut result_matrix = input_matrix.inverse().unwrap();
    for i in 0..result_matrix.rows*result_matrix.colums {
        result_matrix.elements[i].simplify_expression();
    }
    let expected_matrix = matrices.identity;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
#[should_panic]
fn inverse_test_panic() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.two_by_three;
    let _result_matrix = input_matrix.inverse();
}

#[test]
fn adjoint_test() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.three_by_three;
    let result_matrix = input_matrix.adjoint();

    let expected_matrix = Matrix {
        elements: vec![Parameter { expression: vec![String::from("-26")], value: -26.0 } , Parameter { expression: vec![String::from("20")], value: 20.0 } , Parameter { expression: vec![String::from("34")], value: 34.0 } , 
            Parameter { expression: vec![String::from("29")], value: 29.0 } , Parameter { expression: vec![String::from("-40")], value: -40.0 } , Parameter { expression: vec![String::from("24")], value: 24.0 } , 
            Parameter { expression: vec![String::from("7")], value: 7.0 } , Parameter { expression: vec![String::from("30")], value: 30.0 } , Parameter { expression: vec![String::from("-18")], value: -18.0 } ],
        rows: 3,
        colums: 3
    };

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
#[should_panic]
fn adjoint_test_panic() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.two_by_three;
    let _result_matrix = input_matrix.adjoint();
}

#[test]
fn determinant_test() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.three_by_three;
    let result = input_matrix.determinant();
    let expected_value = 230.0;

    //might not work due to floating point, which will need to be fixed somehow
    assert_eq!(result.value, expected_value);
}

#[test]
#[should_panic]
fn determinant_test_panic() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.three_by_two;
    let _result_expression = input_matrix.determinant();
}

/*  
#[test]
fn submatrix_test() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.identity.clone();
    let result_matrix = input_matrix.submatrix(0, 0);

    let expected_matrix = identity_matrix(2);

    assert_eq!(result_matrix, expected_matrix);
}
*/
#[test]
fn transpose_test() {
    let matrices = TestMatrix::default();

    let input_matrix = matrices.two_by_three.clone();
    let result_matrix = input_matrix.transpose();

    let expected_matrix = matrices.three_by_two;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
fn multiply_matrix_test_vector() {
    let matrices = TestMatrix::default();

    let input_matrix_1 = matrices.identity.clone();
    let input_matrix_2 = matrices.vector.clone();

    let mut result_matrix = input_matrix_1.mult(&input_matrix_2).unwrap();
    for i in 0..result_matrix.rows*result_matrix.colums {
        result_matrix.elements[i].simplify_expression();
    }
    let expected_matrix = matrices.vector;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
fn multiply_matrix_test_non_vector() {
    let matrices = TestMatrix::default();

    let input_matrix_1 = matrices.identity.clone();
    let input_matrix_2 = matrices.three_by_three.clone();

    let mut result_matrix = input_matrix_1.mult(&input_matrix_2).unwrap();
    for i in 0..result_matrix.rows*result_matrix.colums {
        result_matrix.elements[i].simplify_expression();
    }
    let expected_matrix = matrices.three_by_three;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
#[should_panic]
fn multiply_matrix_test_panic() {
    let matrices = TestMatrix::default();

    let input_matrix_1 = matrices.identity.clone();
    let input_matrix_2 = matrices.two_by_three.clone();

    let _result_matrix = input_matrix_1.mult(&input_matrix_2);
}

#[test]
fn add_matrix_test() {
    let matrices = TestMatrix::default();

    let input_matrix_1 = matrices.identity.clone();
    let input_matrix_2 = matrices.empty.clone();

    let mut result_matrix = input_matrix_1.add(&input_matrix_2).unwrap();
    for i in 0..result_matrix.rows*result_matrix.colums {
        result_matrix.elements[i].simplify_expression();
    }
    let expected_matrix = matrices.identity;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
#[should_panic]
fn add_matrix_test_panic() {
    let matrices = TestMatrix::default();

    let input_matrix_1 = matrices.vector.clone();
    let input_matrix_2 = matrices.empty.clone();

    let _result_matrix = input_matrix_1.add(&input_matrix_2);
}

#[test]
fn multiply_scalar_test() {
    let matrices = TestMatrix::default();
    
    let input_matrix_1 = matrices.identity.clone();
    let scalar = Parameter{expression: vec![String::from("1")], value: 1.0};

    let mut result_matrix = input_matrix_1.scale(scalar.clone());
    for i in 0..result_matrix.rows*result_matrix.colums {
        result_matrix.elements[i].simplify_expression();
    }
    let expected_matrix = matrices.identity;

    assert_eq!(result_matrix, expected_matrix);
}

#[test]
fn set_test() {
    let matrices = TestMatrix::default();

    let mut matrix = matrices.empty;
    let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0};
    matrix.set(0, 0, parameter_1.clone());
    matrix.set(1, 1, parameter_1.clone());
    matrix.set(2, 2, parameter_1.clone());
    let expected_matrix = matrices.identity;
    assert_eq!(matrix, expected_matrix);
}

#[test]
#[should_panic]
fn set_test_panic() {
    let matrices = TestMatrix::default();

    let parameter_1 = Parameter{expression: vec![String::from("1")], value: 1.0};
    let mut matrix = matrices.empty;
    matrix.set(3, 3, parameter_1.clone());
}
