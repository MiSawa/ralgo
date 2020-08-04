use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::repeat_with;
use std::mem::take;

const E: usize = 5;
const B: usize = 1 << E;
const BUF_LEN: usize = B - 1;

const MASK: u64 = (B - 1) as u64;

struct Elem<K, V> {
    hash: u64,
    key: K,
    value: V,
}

enum Node<K, V> {
    Inner(Box<[Node<K, V>; B]>),
    Outer(Vec<Elem<K, V>>),
}

impl<K, V> Node<K, V> {
    fn new_outer() -> Self {
        Self::Outer(Vec::new())
    }
}
impl<K, V> Default for Node<K, V> {
    fn default() -> Self {
        Self::new_outer()
    }
}

pub struct HashRadixTree<K, V, B: BuildHasher = RandomState> {
    root: Node<K, V>,
    build_hasher: B,
}

impl<K: Hash + Eq, V> HashRadixTree<K, V, RandomState> {
    pub fn new() -> Self {
        HashRadixTree::with_hasher(Default::default())
    }
}

impl<K: Hash + Eq, V, B: BuildHasher> HashRadixTree<K, V, B> {
    pub fn with_hasher(build_hasher: B) -> Self {
        HashRadixTree {
            root: Node::new_outer(),
            build_hasher,
        }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn build_inner(buffer: Vec<Elem<K, V>>, pos: usize) -> Node<K, V> {
        if pos * E > 64 {
            // Ugh.... whatever
            return Node::Outer(buffer);
        }
        let mut buffers: Vec<_> = repeat_with(|| Vec::new()).take(B).collect();
        for elem in buffer {
            let id = ((elem.hash >> (pos * E)) & MASK) as usize;
            buffers[id].push(elem);
        }
        let mut arr: [Node<K, V>; B] = Default::default();
        for (i, b) in buffers.into_iter().enumerate() {
            if b.len() > BUF_LEN {
                arr[i] = Self::build_inner(b, pos + 1)
            } else {
                arr[i] = Node::Outer(b)
            }
        }
        return Node::Inner(Box::new(arr));
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = self.hash(&key);
        let mut node = &mut self.root;
        let mut shifted = hash;
        for i in 0.. {
            let id = (shifted & MASK) as usize;
            match node {
                Node::Inner(children) => {
                    node = &mut children[id];
                }
                Node::Outer(buf) => {
                    for elem in buf.iter_mut() {
                        if &elem.key == &key {
                            let ret = std::mem::replace(&mut elem.value, value);
                            return Some(ret);
                        }
                    }
                    buf.push(Elem { hash, key, value });
                    if buf.len() > BUF_LEN {
                        *node = Self::build_inner(take(buf), i);
                    }
                    return None;
                }
            };
            shifted >>= E;
        }
        unreachable!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(&key);
        let mut node = &self.root;
        let mut shifted = hash;
        for _ in 0.. {
            let id = (shifted & MASK) as usize;
            match node {
                Node::Inner(children) => {
                    node = &children[id];
                }
                Node::Outer(buf) => {
                    for elem in buf {
                        if &elem.key == key {
                            return Some(&elem.value);
                        }
                    }
                    return None;
                }
            };
            shifted >>= E;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut lsmt: HashRadixTree<usize, usize> = HashRadixTree::new();
        for i in 10..1000 {
            lsmt.insert(i, i);
        }
        lsmt.insert(0, 10);
        assert_eq!(lsmt.get(&0), Some(&10));
        assert_eq!(lsmt.get(&1), None);
        lsmt.insert(1, 11);
        assert_eq!(lsmt.get(&1), Some(&11));
    }
}
