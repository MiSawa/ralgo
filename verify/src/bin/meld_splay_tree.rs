use std::collections::BTreeMap;

use ac_library::{ModInt998244353 as K, Monoid, Segtree};
use proconio::input;
use ralgo::{data_structure::splay_tree::*, util::arena::Arena};

// verify-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_sort_range_composite

enum Query {
    Change(usize, usize, K, K),
    Prod(usize, usize, K),
    Sort(usize, usize, bool),
}
impl proconio::source::Readable for Query {
    type Output = Query;

    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
        source: &mut S,
    ) -> Self::Output {
        let t = u8::read(source);
        match t {
            0 => {
                input! {
                    from source,
                    i: usize,
                    p: usize,
                    a: K,
                    b: K,
                };
                Query::Change(i, p, a, b)
            }
            1 => {
                input! {
                    from source,
                    l: usize,
                    r: usize,
                    x: K,
                };
                Query::Prod(l, r, x)
            }
            2 | 3 => {
                input! {
                    from source,
                    l: usize,
                    r: usize,
                };
                Query::Sort(l, r, t == 3)
            }
            _ => unreachable!(),
        }
    }
}

struct RangeSortRangeProd<'a, K, M: Monoid> {
    factory: Trees<'a, K, Reversible<M>>,
    seg: Segtree<M>,
    trees: BTreeMap<usize, (TreeHandle<'a, K, Reversible<M>>, bool)>,
}
impl<'a, K: Ord + Clone, M: Monoid> RangeSortRangeProd<'a, K, M> where K: std::fmt::Debug, M::S: std::fmt::Debug {
    fn new(
        factory: impl Into<Trees<'a, K, Reversible<M>>>,
        it: impl IntoIterator<Item = (K, M::S)>,
    ) -> Self {
        let factory = factory.into();
        let mut trees = BTreeMap::new();
        let mut values = vec![];
        for (i, (k, v)) in it.into_iter().enumerate() {
            let tree = factory.singleton_tree(k, v.clone());
            trees.insert(i, (tree, false));
            values.push(v);
        }
        values.push(M::identity());
        Self {
            factory,
            seg: Segtree::from(values),
            trees,
        }
    }

    fn split(&mut self, i: usize) {
        let offset = *self.trees.range(..=i).last().unwrap().0;
        if offset == i {
            return;
        }
        let (tree, reversed) = self.trees.remove(&offset).unwrap();
        let len = tree.len();
        if reversed {
            let r_len = offset + len - i;
            let (r, l) = tree.split_by_size(r_len);
            self.seg.set(
                offset,
                l.all_product()
                    .map(|p| p.1.clone())
                    .unwrap_or(M::identity()),
            );
            self.trees.insert(offset, (l, true));
            self.seg.set(
                i,
                r.all_product()
                    .map(|p| p.1.clone())
                    .unwrap_or(M::identity()),
            );
            self.trees.insert(i, (r, true));
        } else {
            let (l, r) = tree.split_by_size(i - offset);
            self.seg.set(
                offset,
                l.all_product()
                    .map(|p| p.0.clone())
                    .unwrap_or(M::identity()),
            );
            self.trees.insert(offset, (l, false));
            self.seg.set(
                i,
                r.all_product()
                    .map(|p| p.0.clone())
                    .unwrap_or(M::identity()),
            );
            self.trees.insert(i, (r, false));
        }
    }

    fn set(&mut self, i: usize, key: K, value: M::S) {
        self.split(i);
        self.split(i + 1);

        let (_tree, reversed) = &self.trees[&i];
        let tree = self.factory.singleton_tree(key, value);
        let value = if *reversed {
            tree.all_product()
                .map(|p| p.1.clone())
                .unwrap_or(M::identity())
        } else {
            tree.all_product()
                .map(|p| p.0.clone())
                .unwrap_or(M::identity())
        };
        self.seg.set(i, value);
        self.trees.insert(i, (tree, *reversed));
    }

    fn prod(&mut self, l: usize, r: usize) -> M::S {
        self.split(l);
        self.split(r);
        self.seg.prod(l..r)
    }
    fn sort(&mut self, l: usize, r: usize, rev: bool) {
        self.split(l);
        self.split(r);
        let mut tree = self.factory.empty_tree();
        let mut keys_to_remove = vec![];
        for (k, (v, _)) in self.trees.range(l..r) {
            keys_to_remove.push(*k);
            tree = TreeHandle::meld(&tree, v);
        }
        for k in keys_to_remove {
            self.trees.remove(&k);
            self.seg.set(k, M::identity());
        }
        let value = if rev {
            tree.all_product()
                .map(|p| p.1.clone())
                .unwrap_or(M::identity())
        } else {
            tree.all_product()
                .map(|p| p.0.clone())
                .unwrap_or(M::identity())
        };
        self.seg.set(l, value);
        self.trees.insert(l, (tree, rev));
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        p_a_b: [(usize, (K, K)); n],
        queries: [Query; q],
    };
    enum O {}
    impl Monoid for O {
        type S = (K, K);

        fn identity() -> Self::S {
            (1.into(), 0.into())
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 * b.0, a.1 * b.0 + b.1)
        }
    }

    let arena = Arena::new();
    let mut seg = RangeSortRangeProd::<usize, O>::new(&arena, p_a_b);
    for query in queries {
        match query {
            Query::Change(i, p, a, b) => {
                seg.set(i, p, (a, b));
            }
            Query::Prod(l, r, x) => {
                let (a, b) = seg.prod(l, r);
                println!("{}", a * x + b);
            }
            Query::Sort(l, r, rev) => {
                seg.sort(l, r, rev);
            }
        }
    }
}
