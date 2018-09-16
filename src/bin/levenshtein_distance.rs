extern crate edit_distance;

use edit_distance::levenshtein::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let a = &args[1];
    let b = &args[2];

    let mut l = Levenshtein::new(a, b);
    println!("Distance between {} and {}: {}", a, b, l.distance());
    println!("{:?}", l);
}
