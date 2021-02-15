use proconio::input;

use itertools::Itertools;
use ralgo::flow::dinic::Dinic;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
    };
    a.iter_mut().for_each(|x| *x *= 2);
    b.iter_mut().for_each(|x| *x *= 2);
    let mut g = Dinic::new();

    let ss: Vec<_> = (0..).take(n).collect();
    let tt: Vec<_> = (ss.last().unwrap() + 1..).take(n).collect();
    let inc: Vec<_> = (tt.last().unwrap() + 1..).take(n + 1).collect();
    let dec: Vec<_> = (inc.last().unwrap() + 1..).take(n + 1).collect();
    let s = dec.last().unwrap() + 1;
    let t = s + 1;

    let mut to_inc_edges = Vec::new();
    let mut from_inc_edges = Vec::new();
    for i in 0..n {
        g.add_edge(s, ss[i], 1);
        g.add_edge(tt[i], t, 1);
    }
    for i in 0..n {
        g.add_edge(inc[i], inc[i + 1], n as i32);
        from_inc_edges.push(g.add_edge(inc[i], tt[i], 1 as i32));

        g.add_edge(dec[i + 1], dec[i], n as i32);
        g.add_edge(dec[i + 1], tt[i], 1 as i32);
    }
    for (i, ai) in a.iter().enumerate() {
        let j = b.binary_search(&(ai - 1)).unwrap_or_else(|x| x);
        g.add_edge(ss[i], dec[j], 1);
        let j = b.binary_search(&(ai + 1)).unwrap_or_else(|x| x);
        to_inc_edges.push(g.add_edge(ss[i], inc[j], 1));
    }
    let mf = g.max_flow(s, t).0;
    if (mf as usize) == n {
        println!("Yes");
        let mut q = VecDeque::new();
        for i in 0..n {
            if g.get_flow(&to_inc_edges[i]) > 0 {
                q.push_back(i);
            }
        }
        let mut res = vec![0; n];
        let mut used = vec![false; n];
        for j in (0..n).filter(|&j| g.get_flow(&from_inc_edges[j]) == 1) {
            if let Some(&i) = q.front() {
                if b[j] > a[i] {
                    used[j] = true;
                    res[i] = b[j];
                    q.pop_front();
                }
            } else {
                panic!("???");
            }
        }
        q.clear();
        for i in (0..n).rev() {
            if g.get_flow(&to_inc_edges[i]) == 0 {
                q.push_back(i);
            }
        }
        for j in (0..n).rev().filter(|&j| !used[j]) {
            if let Some(&i) = q.front() {
                if b[j] < a[i] {
                    res[i] = b[j];
                    q.pop_front();
                }
            }
        }
        println!("{}", res.iter().map(|x| x / 2).join(" "));
    } else {
        println!("No");
    }
}
