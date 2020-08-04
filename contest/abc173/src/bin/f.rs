use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n-1],
    };

    let mut g = vec![vec![]; n];
    for (s, t) in es.into_iter() {
        let a = min(s, t);
        let b = max(s, t);
        g[b].push(a);
    }

    let mut res = 0;
    let mut s = 0;
    for i in 0..n {
        s += i + 1;
        for j in g[i].iter().cloned() {
            s -= j + 1;
        }
        res += s;
    }
    println!("{}", res);
}
