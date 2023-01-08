fn matrix_addition(matrix1: Vec<Vec<i32>>, vector2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new(); 

    for (row1, row2) in matrix1.iter().zip(vector2.iter()) {
        result.push(vector_addition(row1.to_vec(), row2.to_vec()));
    }
    result


}
fn vector_addition(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new(); 

    for (x, y) in vector1.iter().zip(vector2.iter()) {
        result.push(x + y);
    };
    result
}

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector2 = vec![4, 5, 6];

    let result = vector_addition(vector1.clone(), vector2.clone());

    let matrix1 = vec![vector1.clone(), vector2.clone()];
    let matrix2 = vec![vector2, vector1];

    let matrix_result = matrix_addition(matrix1, matrix2);


    println!("{:?}", result);
    println!("{:?}", matrix_result);
}
