use std::{iter::repeat_with, option::Option::Some};

use proconio::input;
use ralgo::data_structures::lazy_hollow_heap::LazyHollowHeap;

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize, u64); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v, c) in edges.into_iter() {
        g[u].push((v, c));
    }
    let mut prev = vec![n; n];
    let mut refs: Vec<_> = repeat_with(|| None).take(n).collect();
    let mut pq = LazyHollowHeap::new();
    refs[s] = Some(pq.insert(0, (s, s)));
    while let Some((d, (u, p))) = pq.find_min().map(|(a, b)| (a.clone(), b.clone())) {
        pq.delete(refs[u].take().unwrap());
        prev[u] = p;
        if u == t {
            let mut path = vec![];
            let mut u = u;
            let mut v = prev[u];
            while u != s {
                path.push((v, u));
                u = v;
                v = prev[u];
            }
            path.reverse();
            println!("{} {}", d, path.len());
            for (u, v) in path {
                println!("{} {}", u, v);
            }
            return;
        }
        for (v, c) in g[u].iter().cloned() {
            if prev[v] != n {
                continue;
            }
            if let Some(r) = refs[v].as_mut() {
                pq.update_key_value_better(r, d + c, (v, u));
            } else {
                refs[v] = Some(pq.insert(d + c, (v, u)));
            }
        }
    }
    println!("-1")
}
