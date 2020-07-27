use std::io::{stdin, BufRead};

use ralgo::a_plus_b::a_plus_b;

// verify-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb
// dependency: src/a_plus_b.rs

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut s = String::new();
    stdin.read_line(&mut s).expect("Read line");
    let ab: Vec<u32> = s.split_whitespace().map(|f| f.parse().unwrap()).collect();
    println!("{}", a_plus_b(ab[0], ab[1]));
}
