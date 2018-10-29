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

pub fn distance(a: &String, b: &String) -> u32 {
    let vec_a: Vec<char> = a.chars().collect();
    let vec_b: Vec<char> = b.chars().collect();

    let mut l = Levenshtein::new(&vec_a, &vec_b);
    l.distance()
}

//////////////////////////////////////////////////////////////////////////////////////////

//
// tests
//

#[test]
fn test_distance_between_identical_strings() {
    let string1 = "test for echo".to_string();
    let string2 = "test for echo".to_string();

    let d = distance(&string1, &string2);
    assert_eq!(d, 0);
}

#[test]
fn test_distance_against_one_char_longer() {
    let string1 = "test for echo".to_string();
    let string2 = "test for echo1".to_string();

    let d = distance(&string1, &string2);
    assert_eq!(d, 1);
}

#[test]
fn test_distance_against_one_char_shorter() {
    let string1 = "test for echo".to_string();
    let string2 = "test for ech".to_string();

    let d = distance(&string1, &string2);
    assert_eq!(d, 1);
}

#[test]
fn test_distance_between_empty_one_and_non_empty_one() {
    let string1 = "".to_string();
    let string2 = "test for echo".to_string();

    let d = distance(&string1, &string2);
    assert_eq!(d, string2.len() as u32);
}

#[test]
fn test_distance_between_non_empty_one_and_empty_one() {
    let string1 = "test for echo".to_string();
    let string2 = "".to_string();

    let d = distance(&string1, &string2);
    assert_eq!(d, string1.len() as u32);
}

#[test]
fn test_distance_against_one_with_additional_chars_in_the_middle() {
    let string1 = "test for echo".to_string();
    let string2 = "test for an echo".to_string();  // add 3 chars: "an "

    let d = distance(&string1, &string2);
    assert_eq!(d, 3);
}

#[test]
fn test_distance_against_one_with_removing_some_chars() {
    let string1 = "test for echo".to_string();
    let string2 = "testforecho".to_string();  // remove 2 whitespaces

    let d = distance(&string1, &string2);
    assert_eq!(d, 2);
}

#[test]
fn test_distance_against_one_with_changing_some_chars() {
    let string1 = "test for echo".to_string();
    let string2 = "Test For Echo".to_string();  // change 3 charactors into upper-case

    let d = distance(&string1, &string2);
    assert_eq!(d, 3);
}

#[test]
fn test_distance_against_different_one() {
    let string1 = "test for echo".to_string();
    let string2 = "test 123".to_string();  // remove 5 chars " echo", and replace 3 chars "for" into "123"

    let d = distance(&string1, &string2);
    assert_eq!(d, 8);
}

#[test]
fn test_distance_against_one_without_common_prefix() {
    let string1 = "test for echo".to_string();
    let string2 = "for echo".to_string();  // remove prefix "test "

    let d = distance(&string1, &string2);
    assert_eq!(d, 5);
}

#[test]
fn test_distance_between_non_ascii_strings() {
    let string1 = "テストです".to_string();
    let string2 = "テストだよ".to_string();  // replace 2 chars "です" into "だよ"

    let d = distance(&string1, &string2);
    assert_eq!(d, 2);
}
