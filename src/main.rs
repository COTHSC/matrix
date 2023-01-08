mod matrix_addition;
use matrix_addition::matrix_addition;

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector2 = vec![4, 5, 6];

    let matrix1 = vec![vector1.clone(), vector2.clone()];
    let matrix2 = vec![vector2, vector1];

    let matrix_result = matrix_addition(matrix1, matrix2);

    println!("{:?}", matrix_result);
}
