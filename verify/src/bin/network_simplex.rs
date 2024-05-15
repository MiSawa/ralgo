use proconio::input;
use ralgo::flows::network_simplex::NetworkSimplex;
// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow

fn main() {
    input! {
        n: usize,
        m: usize,
        bs: [i64; n],
        es: [(usize, usize, i64, i64, i64); m]
    }
    let mut ns = NetworkSimplex::new();
    for (v, b) in bs.into_iter().enumerate() {
        ns.add_supply(v, b);
    }
    let mut eids = Vec::new();
    for (s, t, l, u, c) in es.into_iter() {
        eids.push(ns.add_edge(s, t, l, u, c));
    }
    if let Some(ret) = ns.run() {
        println!("{}", ret.get_value::<i128>());
        for v in 0..n {
            println!("{}", ret.get_potential(v));
        }
        for eid in eids {
            println!("{}", ret.get_flow(&eid));
        }
    } else {
        println!("infeasible")
    }
}
