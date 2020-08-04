use itertools::Itertools;
use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    let mut res = 0;
    let mut pq = BinaryHeap::new();

    for x in a.iter().sorted().rev() {
        if let Some(v) = pq.pop() {
            res += v;
            pq.push(x);
            pq.push(x);
        } else {
            pq.push(x);
        }
    }
    println!("{}", res);
}
