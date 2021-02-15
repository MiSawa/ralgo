use proconio::input;

use std::cmp::max;

struct Edge<F> {
    src: usize,
    dst: usize,
    cap: F,
}
struct Graph<F> {
    pub edges: Vec<Edge<F>>,
}
impl<F> Graph<F> {
    fn new() -> Self {
        Graph { edges: vec![] }
    }
    fn add_edge(&mut self, src: usize, dst: usize, cap: F) {
        self.edges.push(Edge { src, dst, cap })
    }
    fn num_nodes(&self) -> usize {
        self.edges
            .iter()
            .map(|e| 1 + max(e.src, e.dst))
            .max()
            .unwrap_or(0)
    }
    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
    };
    a.iter_mut().for_each(|x| *x *= 2);
    b.iter_mut().for_each(|x| *x *= 2);
    let mut g = Graph::new();

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

    println!("p max {} {}", g.num_nodes(), g.num_edges());
    println!("n {} s", s + 1);
    println!("n {} t", t + 1);
    for e in g.edges.iter() {
        println!("a {} {} {}", e.src + 1, e.dst + 1, e.cap);
    }
}
