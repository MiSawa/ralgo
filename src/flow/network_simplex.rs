use crate::flow::{Cost, Flow, Zero};
use core::mem;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Add, Mul};
use std::option::Option::{None, Some};

struct Edge<F, C> {
    src: usize,
    dst: usize,
    flow: F,
    capacity: F,
    cost: C,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct EdgeId(usize);
impl EdgeId {
    fn rev(&self) -> Self {
        EdgeId(self.0 ^ 1)
    }
}

struct VertexData<C> {
    potential: C,
    adjacent_edges: Vec<EdgeId>,
    parent: Option<usize>,
    parent_edge: Option<EdgeId>, // out-tree, i.e. this node == e.src
    depth: usize,
    tree_edges: HashSet<EdgeId>,
}
impl<C: Zero> Default for VertexData<C> {
    fn default() -> Self {
        Self {
            potential: C::zero(),
            adjacent_edges: Vec::new(),
            parent: None,
            parent_edge: None,
            depth: 0,
            tree_edges: HashSet::new(),
        }
    }
}
pub struct NetworkSimplex<F: Flow, C: Cost> {
    edges: Vec<Edge<F, C>>,
    balances: Vec<F>,
}
struct TemporaryData<C: Cost> {
    vertices: Vec<VertexData<C>>,
    n: usize,
    root: usize,
    block_size: usize,
    next_scan_start: usize,
}

pub struct Ret<F, C> {
    edges: Vec<(F, C)>,
    potential: Vec<C>,
}
impl<F: Flow, C: Cost> Ret<F, C> {
    pub fn get_value<T>(&self) -> T
    where
        T: From<F> + From<C> + Mul<Output = T> + Add<Output = T> + Zero,
    {
        self.edges
            .iter()
            .filter(|(f, _)| f.is_positive())
            .map(|(f, c)| T::from(*f) * T::from(*c))
            .fold(T::zero(), |a, b| a + b)
    }
    pub fn get_flow(&self, e: &EdgeId) -> F {
        self.edges[e.0].0
    }
    pub fn get_potential(&self, v: usize) -> C {
        self.potential[v]
    }
}

impl<F: Flow, C: Cost> NetworkSimplex<F, C> {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            balances: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, src: usize, dst: usize, lower: F, upper: F, cost: C) -> EdgeId {
        assert!(
            lower <= upper,
            "lower {} should be less or equal to upper {}",
            lower,
            upper
        );
        let id = self.edges.len();
        self.edges.push(Edge {
            src,
            dst,
            flow: lower,
            capacity: upper,
            cost,
        });
        self.edges.push(Edge {
            src: dst,
            dst: src,
            flow: -lower,
            capacity: -lower,
            cost: -cost,
        });
        if !lower.is_zero() {
            self.add_demand(src, lower);
            self.add_supply(dst, lower);
        }
        EdgeId(id)
    }

    pub fn add_supply(&mut self, v: usize, b: F) {
        let n = max(v + 1, self.balances.len());
        self.balances.resize_with(n, || F::zero());
        self.balances[v] += b;
    }

    pub fn add_demand(&mut self, v: usize, b: F) {
        self.add_supply(v, -b);
    }

    fn get_edge(&self, e: &EdgeId) -> &Edge<F, C> {
        &self.edges[e.0]
    }

    fn get_edge_mut(&mut self, e: &EdgeId) -> &mut Edge<F, C> {
        &mut self.edges[e.0]
    }

    /// return true iff this was a saturating push
    fn add_flow(&mut self, e: &EdgeId, f: F) -> bool {
        self.get_edge_mut(&e.rev()).flow -= f;
        let e = self.get_edge_mut(e);
        e.flow += f;
        e.flow == e.capacity
    }

    fn residual_capacity(e: &Edge<F, C>) -> F {
        e.capacity - e.flow
    }

    fn reduced_cost(data: &TemporaryData<C>, e: &Edge<F, C>) -> C {
        e.cost + data.vertices[e.src].potential - data.vertices[e.dst].potential
    }

    fn update_tree(&self, data: &mut TemporaryData<C>, v: usize) {
        let mut stack = vec![v];
        while let Some(v) = stack.pop() {
            let adj = mem::replace(&mut data.vertices[v].tree_edges, Default::default());
            for eid in adj.iter() {
                let e = self.get_edge(&eid);
                if data.vertices[v].parent == Some(e.dst) {
                    continue;
                }
                data.vertices[e.dst].parent = Some(v);
                data.vertices[e.dst].parent_edge = Some(eid.rev());
                data.vertices[e.dst].depth = data.vertices[e.src].depth + 1;
                data.vertices[e.dst].potential = data.vertices[e.src].potential + e.cost;
                stack.push(e.dst);
            }
            data.vertices[v].tree_edges = adj;
        }
    }

    fn prepare_data(&mut self) -> TemporaryData<C> {
        // allocate root vertex
        let mut infinity = C::one();
        let mut data = TemporaryData {
            vertices: Default::default(),
            n: self.balances.len(),
            root: 0,
            block_size: 1,
            next_scan_start: 0,
        };

        data.vertices.clear();
        for (i, e) in self.edges.iter().enumerate() {
            data.n = max(data.n, 1 + e.src);
            data.vertices.resize_with(data.n, || Default::default());
            data.vertices[e.src].adjacent_edges.push(EdgeId(i));
            if e.cost.is_positive() {
                infinity += e.cost;
            }
        }
        data.root = data.n;
        data.n += 1;
        let root = data.root;
        data.vertices.resize_with(data.n, || Default::default());
        self.balances.resize_with(data.n - 1, || F::zero());
        for v in 0..root {
            let b = mem::replace(&mut self.balances[v], F::zero());
            let (x, y, cap) = if b.is_negative() {
                (root, v, -b)
            } else {
                (v, root, b + F::one())
            };
            let eid = self.add_edge(x, y, F::zero(), cap, infinity);
            self.add_flow(&eid, b.abs());
            data.vertices[x].adjacent_edges.push(eid);
            data.vertices[y].adjacent_edges.push(eid.rev());
            data.vertices[x].tree_edges.insert(eid);
            data.vertices[y].tree_edges.insert(eid.rev());
        }
        data.block_size = min(
            (self.edges.len() as f64).sqrt() as usize + 10,
            self.edges.len(),
        );
        self.update_tree(&mut data, root);
        data
    }

    fn select_edge(&mut self, data: &mut TemporaryData<C>) -> Option<EdgeId> {
        let mut edges = (data.next_scan_start..self.edges.len())
            .chain(0..data.next_scan_start)
            .map(EdgeId)
            .peekable();
        while edges.peek().is_some() {
            let mut selection = Option::None;
            for _ in 0..data.block_size {
                match edges.next() {
                    None => {
                        break;
                    }
                    Some(id) => {
                        let e = self.get_edge_mut(&id);
                        if e.flow == e.capacity {
                            continue;
                        }
                        let rc = Self::reduced_cost(data, e);
                        if rc.is_negative() {
                            let candidate = (rc, id);
                            if let Some(current) = selection.take() {
                                selection = Some(min(current, candidate))
                            } else {
                                selection = Some(candidate)
                            }
                        }
                    }
                }
            }
            if let Some((_, eid)) = selection {
                if let Some(nid) = edges.peek() {
                    data.next_scan_start = nid.0;
                }
                return Some(eid);
            }
        }
        None
    }

    fn pivot(&mut self, data: &mut TemporaryData<C>, eid: EdgeId) {
        let entering_edge = self.get_edge(&eid);
        let Edge { src, dst, .. } = *entering_edge;
        let mut f = Self::residual_capacity(entering_edge);
        let mut a = src;
        let mut b = dst;
        while a != b {
            if data.vertices[a].depth > data.vertices[b].depth {
                let down_edge = data.vertices[a].parent_edge.unwrap().rev();
                let e = self.get_edge(&down_edge);
                f = min(f, Self::residual_capacity(e));
                a = e.src;
            } else {
                let up_edge = data.vertices[b].parent_edge.unwrap();
                let e = self.get_edge(&up_edge);
                f = min(f, Self::residual_capacity(e));
                b = e.dst;
            }
        }
        enum LeavingSide {
            SRC,
            DST,
            ENTER,
        }
        let mut leaving_side = LeavingSide::ENTER;
        let top = a;
        let mut leaving_edge_id = None;
        a = src;
        while a != top {
            let v_data = &data.vertices[a];
            let down_edge = v_data.parent_edge.unwrap().rev();
            if self.add_flow(&down_edge, f) {
                if leaving_edge_id.is_none() {
                    leaving_edge_id = Some(down_edge);
                    leaving_side = LeavingSide::SRC;
                }
            }
            a = v_data.parent.unwrap();
        }
        if self.add_flow(&eid, f) {
            leaving_edge_id = Some(eid);
            leaving_side = LeavingSide::ENTER;
        }
        b = dst;
        while b != top {
            let v_data = &data.vertices[b];
            let up_edge = v_data.parent_edge.unwrap();
            if self.add_flow(&up_edge, f) {
                leaving_edge_id = Some(up_edge);
                leaving_side = LeavingSide::DST;
            }
            b = v_data.parent.unwrap();
        }
        let leaving_edge_id = leaving_edge_id.unwrap();
        let leaving_e = self.get_edge(&leaving_edge_id);
        if leaving_edge_id == eid {
            return;
        }
        assert!(data.vertices[src].tree_edges.insert(eid));
        assert!(data.vertices[dst].tree_edges.insert(eid.rev()));
        assert!(data.vertices[leaving_e.src]
            .tree_edges
            .remove(&leaving_edge_id));
        assert!(data.vertices[leaving_e.dst]
            .tree_edges
            .remove(&leaving_edge_id.rev()));
        match leaving_side {
            LeavingSide::SRC => self.update_tree(data, dst),
            LeavingSide::DST => self.update_tree(data, src),
            LeavingSide::ENTER => return,
        }
    }

    pub fn run(&mut self) -> Option<Ret<F, C>> {
        let mut data = self.prepare_data();
        while let Some(eid) = self.select_edge(&mut data) {
            self.pivot(&mut data, eid);
        }
        for e in self.edges.split_off(self.edges.len() - 2 * (data.n - 1)) {
            if !e.flow.is_zero() {
                return None;
            }
        }
        Some(Ret {
            edges: self.edges.iter().map(|e| (e.flow, e.cost)).collect(),
            potential: data
                .vertices
                .iter()
                .take(data.n - 1)
                .map(|v| v.potential)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut ns: NetworkSimplex<i32, i32> = NetworkSimplex::new();
        let mut edges = Vec::new();
        ns.add_supply(0, 1);
        ns.add_demand(1, 1);
        edges.push(ns.add_edge(0, 1, 1, 2, 1));
        edges.push(ns.add_edge(1, 2, 0, 2, 2));
        edges.push(ns.add_edge(2, 0, -3, 5, 1));
        edges.push(ns.add_edge(0, 2, 0, 3, -2));
        edges.push(ns.add_edge(2, 1, 0, 1, 0));
        let ret = ns.run();
        assert!(ret.is_some());
        let ret = ret.unwrap();
        assert_eq!(ret.get_value::<i32>(), -2);
        let flow: Vec<_> = edges.iter().map(|e| ret.get_flow(e)).collect();
        assert_eq!(flow, vec![1, 0, 3, 3, 0]);
        let mut potential: Vec<_> = (0..3).map(|v| ret.get_potential(v)).collect();
        let offset = potential[0];
        potential.iter_mut().for_each(|p| *p -= offset);
        assert_eq!(potential, vec![0, -1, -1]);
    }
}
