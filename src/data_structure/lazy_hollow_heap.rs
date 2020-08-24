use std::cmp::Ordering;
use std::mem;
use std::option::Option::Some;

pub struct Ref(usize);

struct Node<K, V> {
    key: K,
    value: Option<V>,
    rank: usize,
    second_parent: Option<usize>,
    first_child: Option<usize>,
    next_sibling: Option<usize>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value: Option::Some(value),
            rank: 0,
            second_parent: Option::None,
            first_child: Option::None,
            next_sibling: Option::None,
        }
    }
}

pub struct LazyHollowHeap<K, V> {
    nodes: Vec<Node<K, V>>,
    root: Option<usize>,
}

impl<K: Ord + Clone, V> LazyHollowHeap<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            root: Option::None,
        }
    }

    fn new_node(&mut self, key: K, value: V) -> usize {
        let res = self.nodes.len();
        self.nodes.push(Node::new(key, value));
        res
    }

    fn add_child(&mut self, child: usize, parent: usize) -> usize {
        self.nodes[child].next_sibling =
            mem::replace(&mut self.nodes[parent].first_child, Some(child));
        parent
    }

    fn link(&mut self, lhs: usize, rhs: usize) -> usize {
        match self.nodes[lhs].key.cmp(&self.nodes[rhs].key) {
            Ordering::Less => self.add_child(rhs, lhs),
            Ordering::Equal | Ordering::Greater => self.add_child(lhs, rhs),
        }
    }

    fn meld(&mut self, lhs: Option<usize>, rhs: Option<usize>) -> Option<usize> {
        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            Some(self.link(lhs, rhs))
        } else {
            lhs.or(rhs)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, key: K, value: V) -> Ref {
        let res = self.new_node(key, value);
        self.root = self.meld(Some(res), self.root);
        Ref(res)
    }

    pub fn find_min(&self) -> Option<(&K, &V)> {
        self.root
            .map(|i| (&self.nodes[i].key, self.nodes[i].value.as_ref().unwrap()))
    }

    pub fn update_key_value_better(&mut self, p: &mut Ref, key: K, value: V) {
        if self.nodes[p.0].key <= key {
            return;
        }
        if self.root == Some(p.0) {
            self.nodes[p.0].key = key;
            self.nodes[p.0].value.replace(value);
            return;
        }
        let _ = self.nodes[p.0]
            .value
            .take()
            .expect("Update on a reference to a deleted node");
        let res = self.new_node(key, value);
        if self.nodes[p.0].rank > 2 {
            self.nodes[res].rank = self.nodes[p.0].rank - 2;
        }
        self.nodes[p.0].second_parent = Some(res);
        self.nodes[res].first_child = Some(p.0);
        *p = Ref(res);
        let old_root = self.root.take().expect("Heap should be non-empty");
        self.root = Some(self.link(res, old_root));
    }

    pub fn update_key_better(&mut self, p: &mut Ref, key: K) {
        if self.nodes[p.0].key <= key {
            return;
        }
        if self.root == Some(p.0) {
            self.nodes[p.0].key = key;
            return;
        }
        let value = self.nodes[p.0]
            .value
            .take()
            .expect("Update on a reference to a deleted node");
        let res = self.new_node(key, value);
        if self.nodes[p.0].rank > 2 {
            self.nodes[res].rank = self.nodes[p.0].rank - 2;
        }
        self.nodes[p.0].second_parent = Some(res);
        self.nodes[res].first_child = Some(p.0);
        *p = Ref(res);
        let old_root = self.root.take().expect("Heap should be non-empty");
        self.root = Some(self.link(res, old_root));
    }

    pub fn delete(&mut self, p: Ref) -> (K, V) {
        let value = self.nodes[p.0]
            .value
            .take()
            .expect("Can't delete already deleted node");
        let res = (self.nodes[p.0].key.clone(), value);
        if self.nodes[self.root.unwrap()].value.is_some() {
            return res;
        }
        let mut full_roots: Vec<Option<usize>> = Vec::new();
        let mut next_scan_hollow_root = self.root.take();
        while let Some(t) = next_scan_hollow_root {
            let mut next_scan_child = self.nodes[t].first_child.take();
            next_scan_hollow_root = self.nodes[t].next_sibling;
            while let Some(c) = next_scan_child {
                let node = &mut self.nodes[c];
                next_scan_child = node.next_sibling.take();
                if node.value.is_none() {
                    // take the second parent since we do either remove the parent or move it to the first
                    if let Some(sp) = node.second_parent.take() {
                        if sp == t {
                            // t, which we remove, is the second parent, and c is the last child.
                            // We put next_scan_child back to next_sibling because it was actually for
                            // the first parent.
                            node.next_sibling = next_scan_child.take();
                        } else {
                            // move the second parent to the first
                            // nothing actually needed. second_parent as well as next_sibling are already taken out.
                        }
                    } else {
                        // This node has the first parent only.
                        node.next_sibling = next_scan_hollow_root;
                        next_scan_hollow_root = Some(c)
                    }
                } else {
                    let mut c = c;
                    // ranked link
                    loop {
                        let rank = self.nodes[c].rank;
                        if full_roots.len() <= rank {
                            full_roots.resize_with(rank + 1, Default::default)
                        }
                        if let Some(other) = full_roots[rank].take() {
                            c = self.link(c, other);
                            self.nodes[c].rank += 1;
                        } else {
                            full_roots[rank] = Some(c);
                            break;
                        }
                    }
                }
            }
        }
        // unranked link
        let mut full_roots = full_roots.into_iter().flat_map(|o| o.into_iter());
        if let Some(first) = full_roots.next() {
            // want fold_first stabilized...
            self.root = Some(full_roots.fold(first, |acc, x| self.link(acc, x)))
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut pq = LazyHollowHeap::new();
        let node0 = pq.insert(10, 0);
        let mut node1 = pq.insert(11, 1);
        pq.update_key_better(&mut node1, 9);
        assert_eq!(pq.find_min(), Some((&9, &1)));
        assert_eq!(pq.delete(node1), (9, 1));
        assert_eq!(pq.find_min(), Some((&10, &0)));
        assert_eq!(pq.delete(node0), (10, 0));
        assert!(pq.is_empty());
    }
}
