extern crate edit_distance;

use edit_distance::levenshtein::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let a = &args[1];
    let b = &args[2];

    let vec_a: Vec<char> = a.chars().collect();
    let vec_b: Vec<char> = b.chars().collect();

    let mut l = Levenshtein::new(&vec_a, &vec_b);
    println!("Distance between {} and {}: {}", a, b, l.distance());
    println!("{:?}", l);
}
