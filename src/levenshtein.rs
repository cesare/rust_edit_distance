use ndarray::prelude::*;

use std::cmp::min;

#[derive(Debug)]
pub struct Levenshtein {
    matrix: Array2<u32>,
    a: Vec<char>,
    b: Vec<char>,
    dim_a: usize,
    dim_b: usize,
}

impl Levenshtein {
    pub fn new(a: &String, b: &String) -> Levenshtein {
        let vec_a: Vec<char> = a.chars().collect();
        let vec_b: Vec<char> = b.chars().collect();

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
            a: vec_a,
            b: vec_b,
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
