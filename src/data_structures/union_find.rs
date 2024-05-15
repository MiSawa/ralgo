struct Node {
    parent: usize,
    size: usize,
}
pub struct UnionFind {
    nodes: Vec<Node>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (0..n).map(|i| Node { parent: i, size: 1 }).collect(),
        }
    }

    fn join_roots(&mut self, parent: usize, child: usize) {
        self.nodes[parent].size += self.nodes[child].size;
        self.nodes[child].parent = parent;
    }

    pub fn unite(&mut self, u: usize, v: usize) -> (bool, usize) {
        let u = self.find_mut(u);
        let v = self.find_mut(v);
        if u == v {
            (false, u)
        } else {
            let new_root = if self.nodes[u].size >= self.nodes[v].size {
                self.join_roots(u, v);
                u
            } else {
                self.join_roots(v, u);
                v
            };
            (true, new_root)
        }
    }

    pub fn find_mut(&mut self, mut u: usize) -> usize {
        while self.nodes[u].parent != u {
            let grand_parent = self.nodes[self.nodes[u].parent].parent;
            self.nodes[u].parent = grand_parent;
            u = grand_parent;
        }
        u
    }

    pub fn find(&self, mut u: usize) -> usize {
        loop {
            let p = self.nodes[u].parent;
            if u == p {
                return u;
            } else {
                u = p
            }
        }
    }

    pub fn same_mut(&mut self, u: usize, v: usize) -> bool {
        self.find_mut(u) == self.find_mut(v)
    }

    pub fn same(&self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn component_len(&self, u: usize) -> usize {
        let u = self.find(u);
        self.nodes[u].size
    }

    pub fn component_len_mut(&mut self, u: usize) -> usize {
        let u = self.find_mut(u);
        self.nodes[u].size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.find(3), 3);
        assert!(uf.unite(2, 3).0);
        assert!(uf.same(2, 3));
        assert!(!uf.same(1, 2));
        assert!(uf.unite(1, 3).0);
        assert!(uf.same_mut(1, 2));
        assert!(!uf.unite(1, 2).0);
    }
}
