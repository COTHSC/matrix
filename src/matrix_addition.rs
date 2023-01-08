pub fn matrix_addition(matrix1: Vec<Vec<i32>>, vector2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for (row1, row2) in matrix1.iter().zip(vector2.iter()) {
        result.push(vector_addition(row1.to_vec(), row2.to_vec()));
    }
    result
}

fn vector_addition(vector1: Vec<i32>, vector2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut iter1 = vector1.iter().peekable();
    let mut iter2 = vector2.iter().peekable();

    while let (Some(x), Some(y)) = (iter1.peek(), iter2.peek()) {
        result.push(*x + *y);
        iter1.next();
        iter2.next();
    }

    for x in iter1 {
        result.push(*x);
    }

    for y in iter2 {
        result.push(*y);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let matrix1 = vec![1, 2];
        let matrix2 = vec![5, 6, 7, 8];
        let expected = vec![6, 8, 7, 8];
        assert_eq!(vector_addition(matrix1, matrix2), expected);

        let matrix1 = vec![vec![1, 2], vec![3, 4]];
        let matrix2 = vec![vec![5, 6], vec![7, 8]];
        let expected = vec![vec![6, 8], vec![10, 12]];

        assert_eq!(matrix_addition(matrix1, matrix2), expected);

        let matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let matrix2 = vec![vec![6, 5, 4], vec![3, 2, 1]];
        let expected = vec![vec![7, 7, 7], vec![7, 7, 7]];
        assert_eq!(matrix_addition(matrix1, matrix2), expected);

        let matrix1 = vec![vec![1, 2], vec![3, 4]];
        let matrix2 = vec![vec![5, 6, 7], vec![8, 9, 0]];
        let expected = vec![vec![6, 8, 7], vec![11, 13, 0]];
        assert_eq!(matrix_addition(matrix1, matrix2), expected);
    }
}
