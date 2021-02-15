#!/usr/bin/env run-cargo-script
// cargo-deps: rand, itertools

extern crate rand;
extern crate itertools;

use rand::distributions::{Distribution, Uniform};
use itertools::Itertools;

fn main() {
    let rng = rand::thread_rng();
    let n = 100_000;
    println!("{}", n);

    let dist = Uniform::from(1..=n);
    let mut a: Vec<_> = dist.sample_iter(rng).take(n).collect();
    let mut b: Vec<_> = dist.sample_iter(rng).take(n).collect();
    a.sort();
    b.sort();
    println!("{}", a.iter().join(" "));
    println!("{}", b.iter().join(" "));
}
