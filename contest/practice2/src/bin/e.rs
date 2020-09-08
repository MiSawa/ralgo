use proconio::{fastout, input};
use ralgo::flow::network_simplex::NetworkSimplex;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[i64; n]; n]
    }
    let mut ns = NetworkSimplex::new();
    let s = 2 * n;
    let mut edges = vec![];
    for u in 0..n {
        ns.add_edge(s, u, 0, k as i64, 0);
        ns.add_edge(u + n, s, 0, k as i64, 0);
        edges.push(vec![]);
        for v in 0..n {
            edges[u].push(ns.add_edge(u, v + n, 0, 1, -a[u][v]));
        }
    }
    let res = ns.run().expect("Infeasible!");
    println!("{}", -res.get_value::<i64>());
    for u in 0..n {
        for v in 0..n {
            print!(
                "{}",
                if res.get_flow(&edges[u][v]) == 0 {
                    '.'
                } else {
                    'X'
                }
            );
        }
        println!()
    }
}
