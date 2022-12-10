use std::{fs, vec};

struct Matrix {
    pub matrix: Vec<Vec<u32>>,
}

impl Matrix {
    fn new(matrix: Vec<Vec<u32>>) -> Matrix {
        Matrix { matrix }
    }

    fn n_rows(&self) -> usize {
        self.matrix.len()
    }

    fn n_columns(&self) -> usize {
        self.matrix[0].len()
    }

    fn get_column(&self, col: usize) -> Vec<u32> {
        self.matrix
            .iter()
            .map(|row| *row.get(col).unwrap())
            .collect()
    }

    fn visible_to_left(&self, vector: &[u32], idx: usize) -> bool {
        vector[0..idx].iter().all(|n| n < &vector[idx])
    }

    fn score_to_left(&self, vector: &[u32], idx: usize) -> usize {
        match vector[0..idx].iter().rev().position(|n| n >= &vector[idx]) {
            Some(position) => position + 1,
            None => idx,
        }
    }

    fn visible_to_right(&self, vector: &[u32], idx: usize) -> bool {
        vector[idx + 1..vector.len()]
            .iter()
            .all(|n| n < &vector[idx])
    }

    fn score_to_right(&self, vector: &[u32], idx: usize) -> usize {
        match vector[idx + 1..vector.len()]
            .iter()
            .position(|n| n >= &vector[idx])
        {
            Some(position) => (position + idx + 1) - idx,
            None => vector.len() - idx - 1,
        }
    }

    pub fn score(&self, row: usize, col: usize) -> usize {
        // score from row
        let vector = &self.matrix[row];
        let row_score = self.score_to_left(&vector, col) * self.score_to_right(&vector, col);

        // score from column
        let vector = self.get_column(col);
        let col_score = self.score_to_left(&vector, row) * self.score_to_right(&vector, row);

        row_score * col_score
    }

    pub fn is_visible_in_row(&self, row: usize, col: usize) -> bool {
        let left_side = self.visible_to_left(&self.matrix[row], col);
        let right_side = self.visible_to_right(&self.matrix[row], col);
        left_side || right_side
    }

    pub fn is_visible_in_col(&self, row: usize, col: usize) -> bool {
        let column = self.get_column(col);
        let upper_side = self.visible_to_left(&column, row);
        let lower_side = self.visible_to_right(&column, row);
        upper_side || lower_side
    }
}

fn main() {
    // get content
    let contents = fs::read_to_string("../input.txt").expect("Missing input file");

    // Parse the contents of the file as a matrix of numbers
    let matrix: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let matrix = Matrix::new(matrix);

    let mut visible_count = matrix.n_columns() * 2 + matrix.n_rows() * 2 - 4;
    for col in 1..matrix.n_columns() - 1 {
        for row in 1..matrix.n_rows() - 1 {
            if matrix.is_visible_in_col(row, col) || matrix.is_visible_in_row(row, col) {
                visible_count += 1
            }
        }
    }
    println!("Visible Count: {}", visible_count);

    let mut max = 0;
    for col in 1..matrix.n_columns() - 1 {
        for row in 1..matrix.n_rows() - 1 {
            let current_score = matrix.score(row, col);
            if current_score > max {
                max = current_score;
            }
        }
    }
    println!("The max score is {}", max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_row_visibility() {
        let matrix: Vec<Vec<u32>> = [
            [3, 0, 3, 7, 3].to_vec(),
            [2, 5, 5, 1, 2].to_vec(),
            [6, 5, 3, 3, 2].to_vec(),
            [3, 3, 5, 4, 9].to_vec(),
            [3, 5, 3, 9, 0].to_vec(),
        ]
        .to_vec();

        let matrix = Matrix::new(matrix);

        // 25512
        //  ^
        assert!(matrix.is_visible_in_row(1, 1));
        // 25512
        //   ^
        assert!(matrix.is_visible_in_row(1, 2));
        // 33549
        //  ^
        assert!(!matrix.is_visible_in_row(3, 1));
        // 33549
        //   ^
        assert!(matrix.is_visible_in_row(3, 2));
        // 33549
        //    ^
        assert!(!matrix.is_visible_in_row(3, 1));
    }

    #[test]
    fn test_matrix_row_score() {
        let matrix: Vec<Vec<u32>> = [
            [3, 0, 3, 7, 3].to_vec(),
            [2, 5, 5, 1, 2].to_vec(),
            [6, 5, 3, 3, 2].to_vec(),
            [3, 3, 5, 4, 9].to_vec(),
            [3, 5, 3, 9, 0].to_vec(),
        ]
        .to_vec();

        let matrix = Matrix::new(matrix);

        let row = &matrix.matrix[1];
        assert_eq!(1, matrix.score_to_left(&row, 2));

        let row = &matrix.matrix[3];
        assert_eq!(2, matrix.score_to_left(&row, 2));

        let row = &matrix.matrix[1];
        assert_eq!(2, matrix.score_to_right(&row, 2));

        let row = &matrix.matrix[3];
        assert_eq!(2, matrix.score_to_right(&row, 2));

        let row = &matrix.matrix[4];
        assert_eq!(1, matrix.score_to_left(&row, 2));

        let row = &matrix.matrix[4];
        assert_eq!(1, matrix.score_to_right(&row, 2));

        let row = &matrix.matrix[2];
        assert_eq!(1, matrix.score_to_left(&row, 1));

        let row = &matrix.matrix[2];
        assert_eq!(3, matrix.score_to_right(&row, 1));

        let row = &matrix.matrix[4];
        assert_eq!(1, matrix.score_to_left(&row, 1));

        let row = &matrix.matrix[4];
        assert_eq!(2, matrix.score_to_right(&row, 1));
    }

    #[test]
    fn test_matrix_score() {
        let matrix: Vec<Vec<u32>> = [
            [3, 0, 3, 7, 3].to_vec(),
            [2, 5, 5, 1, 2].to_vec(),
            [6, 5, 3, 3, 2].to_vec(),
            [3, 3, 5, 4, 9].to_vec(),
            [3, 5, 3, 9, 0].to_vec(),
        ]
        .to_vec();

        let matrix = Matrix::new(matrix);

        assert_eq!(4, matrix.score(1, 2));

        assert_eq!(8, matrix.score(3, 2));
    }

    #[test]
    fn test_matrix_get_column() {
        let matrix: Vec<Vec<u32>> = [
            [3, 0, 3, 7, 3].to_vec(),
            [2, 5, 5, 1, 2].to_vec(),
            [6, 5, 3, 3, 2].to_vec(),
            [3, 3, 5, 4, 9].to_vec(),
            [3, 5, 3, 9, 0].to_vec(),
        ]
        .to_vec();

        let matrix = Matrix::new(matrix);

        assert_eq!([3, 2, 6, 3, 3].to_vec(), matrix.get_column(0))
    }

    #[test]
    fn test_matrix_column_visibility() {
        let matrix: Vec<Vec<u32>> = [
            [3, 0, 3, 7, 3].to_vec(),
            [2, 5, 5, 1, 2].to_vec(),
            [6, 5, 3, 3, 2].to_vec(),
            [3, 3, 5, 4, 9].to_vec(),
            [3, 5, 3, 9, 0].to_vec(),
        ]
        .to_vec();

        let matrix = Matrix::new(matrix);

        //   3
        //   2
        // > 6
        //   3
        //   3
        assert!(matrix.is_visible_in_col(2, 0));

        //   3
        // > 2
        //   6
        //   3
        //   3
        assert!(!matrix.is_visible_in_col(1, 0));

        //   3
        //   5
        // > 3
        //   5
        //   3
        assert!(!matrix.is_visible_in_col(3, 3));
    }
}
