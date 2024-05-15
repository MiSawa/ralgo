use proconio::{input, marker::Usize1};
use ralgo::flows::dinic::Dinic;

// verification-helper: PROBLEM https://yukicoder.me/problems/177

fn main() {
    input! {
        w: i32,
        n: usize,
        js: [i32; n],
        m: usize,
        cs: [i32; m],
        qs: [[Usize1]; m],
    }
    let mut dinic = Dinic::new();
    let jv: Vec<_> = (0..n).collect();
    let cv: Vec<_> = (n..(n + m)).collect();
    let s = n + m;
    let t = s + 1;

    for i in 0..n {
        dinic.add_edge(s, jv[i], js[i]);
    }
    for i in 0..m {
        dinic.add_edge(cv[i], t, cs[i]);
    }
    let mut matchs = vec![vec![true; n]; m];
    for i in 0..m {
        for &j in &qs[i] {
            matchs[i][j] = false;
        }
    }
    for i in 0..m {
        for j in 0..n {
            if matchs[i][j] {
                dinic.add_edge(jv[j], cv[i], w);
            }
        }
    }
    if dinic.augment(s, t, w) == w {
        println!("SHIROBAKO")
    } else {
        println!("BANSAKUTSUKITA")
    }
}
