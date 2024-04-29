use proconio::input;
use ralgo::flow::network_simplex::NetworkSimplex;
// verification-helper: PROBLEM https://judge.yosupo.jp/problem/assignment
// verification-helper: IGNORE

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    };
    let mut ns = NetworkSimplex::new();
    let mut perm = vec![];
    for i in 0..n {
        for j in 0..n {
            perm.push((i, j));
        }
    }
    perm.sort_by_key(|(x, y)| x ^ y);

    let mut edges = vec![vec![]; n];
    for i in 0..n {
        ns.add_supply(i, 1);
        ns.add_demand(i + n, 1);
    }
    for (i, j) in perm {
        edges[i].push(ns.add_edge(i, j + n, 0, 1, a[i][j]));
    }
    let result = ns.run().unwrap();

    println!("{}", result.get_value::<i64>());
    for i in 0..n {
        for j in 0..n {
            if result.get_flow(&edges[i][j]) == 1 {
                print!("{}{}", j, if i + 1 == n { '\n' } else { ' ' });
            }
        }
    }
}
