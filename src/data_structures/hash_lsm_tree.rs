use std::{
    cmp::{min, Ordering},
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
    iter::Peekable,
    mem,
};

const BUFFER_SIZE: usize = 16;

struct Elem<K, V> {
    hash: u64,
    key: K,
    value: V,
}

impl<K: Ord, V> Elem<K, V> {
    fn compare_hash_and_key(lhs: &Self, rhs: &Self) -> Ordering {
        (&lhs.hash, &lhs.key).cmp(&(&rhs.hash, &rhs.key))
    }
}

type Level<K, V> = Vec<Elem<K, V>>;

pub struct HashLSMTree<K, V, B: BuildHasher = std::collections::hash_map::RandomState> {
    buffer: Vec<Elem<K, V>>,
    levels: Vec<Level<K, V>>,
    build_hasher: B,
}

struct MergingIter<K, V, I: Iterator<Item = Elem<K, V>>> {
    older: Peekable<I>,
    newer: Peekable<I>,
}
impl<K, V, I: Iterator<Item = Elem<K, V>>> MergingIter<K, V, I> {
    fn from(older: I, newer: I) -> Self {
        Self {
            older: older.peekable(),
            newer: newer.peekable(),
        }
    }
}

impl<K: Ord, V, I: Iterator<Item = Elem<K, V>>> Iterator for MergingIter<K, V, I> {
    type Item = Elem<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.older.peek(), self.newer.peek()) {
            (Some(lhs), Some(rhs)) => {
                return match Elem::compare_hash_and_key(lhs, rhs) {
                    Ordering::Equal => {
                        self.older.next();
                        self.newer.next()
                    }
                    Ordering::Less => self.older.next(),
                    Ordering::Greater => self.newer.next(),
                }
            }
            (_, Some(_)) => self.newer.next(),
            (Some(_), _) => self.older.next(),
            _ => None,
        }
    }
}

impl<K: Ord + Hash + Eq, V> HashLSMTree<K, V, RandomState> {
    pub fn new() -> Self {
        HashLSMTree::with_hasher(Default::default())
    }
}

impl<K: Ord + Hash + Eq, V, B: BuildHasher> HashLSMTree<K, V, B> {
    pub fn with_hasher(build_hasher: B) -> Self {
        HashLSMTree {
            buffer: Vec::new(),
            levels: Vec::new(),
            build_hasher,
        }
    }

    fn merge_levels(older: Level<K, V>, newer: Level<K, V>) -> Level<K, V> {
        let merging_iter = MergingIter::from(older.into_iter(), newer.into_iter());
        return merging_iter.collect();
    }

    fn fix(&mut self) {
        if self.buffer.len() < BUFFER_SIZE {
            return;
        }
        let mut buf = mem::replace(&mut self.buffer, Vec::new());
        buf.sort_unstable_by(|lhs, rhs| Elem::compare_hash_and_key(lhs, rhs));
        while let Some(l) = self.levels.pop() {
            if buf.len() < l.len() {
                self.levels.push(l);
                break;
            }
            buf = Self::merge_levels(l, buf);
        }
        self.levels.push(buf);
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, key: K, value: V) {
        for elem in self.buffer.iter_mut() {
            if elem.key == key {
                elem.value = value;
                return;
            }
        }
        self.buffer.push(Elem {
            hash: self.hash(&key),
            key,
            value,
        });
        self.fix()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for elem in self.buffer.iter() {
            if &elem.key == key {
                return Some(&elem.value);
            }
        }
        let hash = self.hash(&key);
        let ratio: f64 = (hash as f64) / 2.0f64.powi(64);
        for level in self.levels.iter().rev() {
            if level.is_empty() {
                continue;
            }
            let i = min(
                ((level.len() as f64) * ratio).floor() as usize,
                level.len() - 1,
            );
            if level[i].hash <= hash {
                for j in i..level.len() {
                    if hash < level[j].hash {
                        break;
                    }
                    if &level[j].key == key {
                        return Some(&level[j].value);
                    }
                }
            }
            if level[i].hash >= hash {
                for j in (0..=i).rev() {
                    if hash > level[j].hash {
                        break;
                    }
                    if &level[j].key == key {
                        return Some(&level[j].value);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut lsmt: HashLSMTree<usize, usize> = HashLSMTree::new();
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
