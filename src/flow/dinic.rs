use crate::flow::Flow;
use core::mem;
use std::cmp::{max, min};
use std::option::Option::Some;

struct Edge<F> {
    dst: usize,
    rev: usize,
    flow: F,
    upper: F,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct EdgeId(usize, usize);
impl EdgeId {}

struct TemporaryData {
    n: usize,
    s: usize,
    t: usize,
    label: Vec<usize>,
    current_edge: Vec<usize>,
    buffer: Vec<usize>,
}

pub struct Dinic<F: Flow> {
    edges: Vec<Vec<Edge<F>>>,
}
impl<F: Flow> Dinic<F> {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn reserve(&mut self, n: usize) {
        self.edges.resize_with(n, || Vec::with_capacity(8));
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, capacity: F) -> EdgeId {
        let n = max(max(src, dst) + 1, self.edges.len());
        self.edges.resize_with(n, || Vec::with_capacity(8));
        let e = self.edges[src].len();
        let re = self.edges[dst].len() + if src == dst { 1 } else { 0 };

        self.edges[src].push(Edge {
            dst,
            rev: re,
            flow: F::zero(),
            upper: capacity,
        });
        self.edges[dst].push(Edge {
            dst: src,
            rev: e,
            flow: capacity,
            upper: capacity,
        });
        EdgeId(src, e)
    }

    fn prepare_data(&mut self, s: usize, t: usize) -> TemporaryData {
        let n = max(max(s, t) + 1, self.edges.len());
        self.edges.resize_with(n, || Default::default());
        TemporaryData {
            n,
            s,
            t,
            label: vec![0; n],
            current_edge: vec![0; n],
            buffer: Vec::with_capacity(n),
        }
    }

    fn dual(&self, data: &mut TemporaryData) -> bool {
        let n = data.n;
        data.label.iter_mut().for_each(|v| *v = n);
        data.current_edge.iter_mut().for_each(|v| *v = 0);
        let mut queue = mem::take(&mut data.buffer);
        queue.clear();
        queue.push(data.s);
        data.label[data.s] = 0;
        let mut q_pos = 0;
        while q_pos < queue.len() {
            let u = queue[q_pos];
            q_pos += 1;
            let next_label = data.label[u] + 1;
            for e in &self.edges[u] {
                if e.flow < e.upper && data.label[e.dst] == data.n {
                    data.label[e.dst] = next_label;
                    queue.push(e.dst);
                }
            }
        }
        data.buffer = queue;
        data.label[data.t] < n
    }
    fn primal(&mut self, data: &mut TemporaryData, limit: F) -> F {
        let mut res = F::zero();
        let mut stack = mem::take(&mut data.buffer);
        stack.clear();
        stack.push(data.t);
        'new_node: while let Some(&top) = stack.last() {
            if top == data.s {
                stack.pop();
                let mut f = limit - res;
                for &u in &stack {
                    let e = &self.edges[u][data.current_edge[u]];
                    f = min(f, e.flow);
                }
                for i in (0..stack.len()).rev() {
                    let u = stack[i];
                    let e = data.current_edge[u];
                    let v = self.edges[u][e].dst;
                    let re = self.edges[u][e].rev;
                    self.edges[v][re].flow += f;
                    self.edges[u][e].flow -= f;
                    if self.edges[u][e].flow.is_zero() {
                        stack.truncate(i + 1);
                        data.current_edge[u] += 1;
                    }
                }
                res += f;
                if res == limit {
                    break 'new_node;
                }
                continue 'new_node;
            }
            let u = top;
            let i = &mut data.current_edge[u];
            while *i < self.edges[u].len() {
                let e: &mut Edge<F> = &mut self.edges[u][*i];
                if e.flow.is_positive() && data.label[e.dst] < data.label[u] {
                    stack.push(e.dst);
                    continue 'new_node;
                }
                *i += 1;
            }
            data.label[u] = data.n;
            stack.pop();
        }
        data.buffer = stack;
        res
    }

    pub fn augment(&mut self, s: usize, t: usize, limit: F) -> F {
        let mut data = self.prepare_data(s, t);
        let mut flow = F::zero();
        while self.dual(&mut data) {
            flow += self.primal(&mut data, limit - flow);
            if flow == limit {
                break;
            }
        }
        flow
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> (F, Vec<usize>) {
        let mut data = self.prepare_data(s, t);
        let inf = self.edges[s]
            .iter()
            .map(|e| e.upper - e.flow)
            .fold(F::zero(), |a, b| a + b);
        let mut flow = F::zero();
        while self.dual(&mut data) {
            flow += self.primal(&mut data, inf);
        }
        let label = mem::take(&mut data.label);
        let cut = label
            .into_iter()
            .enumerate()
            .filter(|(_, l)| l < &data.n)
            .map(|(i, _)| i)
            .collect();
        (flow, cut)
    }

    pub fn get_flow(&self, e: &EdgeId) -> F {
        self.edges[e.0][e.1].flow
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut dinic: Dinic<i32> = Dinic::new();
        let mut edges = Vec::new();
        edges.push(dinic.add_edge(0, 1, 3));
        edges.push(dinic.add_edge(0, 2, 3));
        edges.push(dinic.add_edge(1, 2, 2));
        edges.push(dinic.add_edge(1, 3, 3));
        edges.push(dinic.add_edge(2, 4, 2));
        edges.push(dinic.add_edge(3, 4, 4));
        edges.push(dinic.add_edge(3, 5, 2));
        edges.push(dinic.add_edge(4, 5, 3));
        let ret = dinic.max_flow(0, 5);
        assert_eq!(5, ret.0);
        assert_eq!(vec![0, 2], ret.1);
        assert_eq!(3, dinic.get_flow(&edges[0]));
        assert_eq!(2, dinic.get_flow(&edges[1]));
        assert_eq!(0, dinic.get_flow(&edges[2]));
        assert_eq!(3, dinic.get_flow(&edges[3]));
        assert_eq!(2, dinic.get_flow(&edges[4]));
        assert_eq!(1, dinic.get_flow(&edges[5]));
        assert_eq!(2, dinic.get_flow(&edges[6]));
        assert_eq!(3, dinic.get_flow(&edges[7]));
    }
}
