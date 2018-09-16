use ndarray::prelude::*;

use std::cmp::min;

#[derive(Debug)]
pub struct Levenshtein<T: Eq + Clone> {
    matrix: Array2<u32>,
    a: Vec<T>,
    b: Vec<T>,
    dim_a: usize,
    dim_b: usize,
}

impl<T: Eq + Clone> Levenshtein<T> {
    pub fn new(a: &Vec<T>, b: &Vec<T>) -> Levenshtein<T> {
        let vec_a = a.clone();
        let vec_b = b.clone();

        let dim_a = vec_a.len() + 1;
        let dim_b = vec_b.len() + 1;

        let mut matrix = Array2::zeros((dim_a, dim_b));
        for i in 0..dim_a {
            matrix[[i, 0]] = i as u32;
        }
        for j in 0..dim_b {
            matrix[[0, j]] = j as u32;
        }

        Levenshtein {
            matrix: matrix,
            a: vec_a.to_vec(),
            b: vec_b.to_vec(),
            dim_a: dim_a,
            dim_b: dim_b,
        }
    }

    pub fn distance(&mut self) -> u32 {
        let dim_a = self.dim_a;
        let dim_b = self.dim_b;

        for i in 1..dim_a {
            for j in 1..dim_b {
                self.matrix[[i, j]] = self.value_of(i, j);
            }
        }
        self.matrix[[dim_a - 1, dim_b - 1]]
    }

    fn value_of(&self, i: usize, j: usize) -> u32 {
        let cost = if self.a[i - 1] == self.b[j -1] { 0 } else { 1 };
        let m = &self.matrix;
        min3(m[[i - 1, j]] + 1, m[[i, j - 1]] + 1, m[[i - 1, j - 1]] + cost)
    }
}

fn min3(a: u32, b: u32, c: u32) -> u32 {
    let s = min(a, b);
    min(s, c)
}
