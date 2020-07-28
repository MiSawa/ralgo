use proconio::input;
use ralgo::data_structure::union_find::UnionFind;

// verify-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind
// dependency: src/data_structure/union_find.rs

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize, usize); q]
    }
    let mut uf = UnionFind::new(n);
    for (t, u, v) in queries.into_iter() {
        if t == 0 {
            uf.unite(u, v);
        } else {
            println!("{}", if uf.same_mut(u, v) { 1 } else { 0 });
        }
    }
}
