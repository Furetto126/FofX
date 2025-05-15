use std::ops::{Index, IndexMut};

use crate::constraints::{constraint::Constraint, Point};

#[derive(Debug, Clone)]
struct LinearMatrix {
    h: usize,
    w: usize,
    data: Vec<Vec<f64>>
}

impl LinearMatrix {
    fn new(n: usize) -> Self {
        let data = vec![vec![0.0; n+1]; n];
        LinearMatrix { h: n, w: n+1, data }
    }

    fn print(&self) {
        for row in &self.data {
            print!("[ ");
            for i in 0..self.w-1 {
                print!("{} ", row[i]);
            }
            print!(",\t\t {}", row[self.w-1]);
            println!("]");
        }
        println!();
    }
}
impl Index<usize> for LinearMatrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for LinearMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

fn gaussian_elimination(matrix: &mut LinearMatrix, i: usize) {
    // Swap rows.
    let mut max_abs_val_row = i;
    let mut max_abs_val     = 0.0;
    for (j, row) in (matrix.data[i..matrix.h]).iter().enumerate() {
        let current_val = row[i].abs();
        if current_val > max_abs_val {
            max_abs_val = current_val;
            max_abs_val_row = j+i;
        }
    }
    matrix.data.swap(i, max_abs_val_row);

    let pivot = matrix[i][i];
    for r in i+1..matrix.h {
        let elimination_coefficient = -matrix[r][i] / pivot;
        for j in i..matrix.w {
            matrix[r][j] = matrix[r][j] + elimination_coefficient*matrix[i][j];
        }
    }
}

pub fn function_through(points: &Vec<Box<dyn Constraint>>) -> Vec<f64> {
    // Construct matrix for Gaussian elimination.
    // ------------------------------------------
    let n = points.len();
    let mut matrix = LinearMatrix::new(n);

    for i in 0..n {
        matrix[i] = points[i].to_linear_equation(n).clone();
    }

    // Iterate gaussian elimination.
    for i in 0..n-1 {
        gaussian_elimination(&mut matrix, i);
    }

    let mut coefficients = vec![0.0; n];
    for i in (0..n).rev() {
        let mut tot = matrix[i][matrix.w-1];
        for j in (i+1..matrix.w-1).rev() {
            tot -= matrix[i][j]*coefficients[j];
        }
        coefficients[i] = tot / matrix[i][i];
    }

    coefficients
}