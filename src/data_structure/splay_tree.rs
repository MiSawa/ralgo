use std::{
    cell::{Ref, RefCell},
    mem,
};

pub trait SeqOps {
    type Value;
    type Acc;

    fn value_to_acc(_val: &Self::Value) -> Self::Acc;

    fn identity() -> Self::Acc;

    fn binary_operation(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc;

    fn reverse(_val: &mut Self::Value, _acc: &mut Self::Acc) {}
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
}
impl Dir {
    fn flip(&mut self) {
        *self = match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

type NodeRef<'a, K, O> = &'a RefCell<Node<'a, K, O>>;
pub struct Node<'a, K, O: SeqOps> {
    key: K,
    l: Option<NodeRef<'a, K, O>>,
    r: Option<NodeRef<'a, K, O>>,
    p: Option<(Dir, NodeRef<'a, K, O>)>,
    len: usize,
    rev: bool,
    value: O::Value,
    acc: O::Acc,
}

impl<'a, K, O: SeqOps> Node<'a, K, O> {
    fn create(key: K, value: O::Value) -> Self {
        let acc = O::value_to_acc(&value);
        Self {
            key,
            l: None,
            r: None,
            p: None,
            len: 1,
            rev: false,
            value,
            acc,
        }
    }
    fn if_present<V>(
        node: &Option<NodeRef<'a, K, O>>,
        mut f: impl FnMut(&mut Self) -> V,
    ) -> Option<V> {
        node.map(RefCell::borrow_mut).map(|mut node| f(&mut node))
    }
    fn each_child(&self, mut f: impl FnMut(&mut Self)) {
        Self::if_present(&self.l, &mut f);
        Self::if_present(&self.r, &mut f);
    }
    fn push(&mut self) {
        if mem::take(&mut self.rev) {
            mem::swap(&mut self.l, &mut self.r);
            self.each_child(|ch| {
                ch.rev ^= true;
                ch.p.as_mut().unwrap().0.flip();
            });
            O::reverse(&mut self.value, &mut self.acc);
        }
    }

    fn pull(&mut self) {
        let mut len = 1;
        let mut acc = O::identity();
        Self::if_present(&self.l, |l| {
            l.push();
            len += l.len;
            acc = O::binary_operation(&acc, &l.acc);
        });
        acc = O::binary_operation(&acc, &O::value_to_acc(&self.value));
        Self::if_present(&self.r, |r| {
            r.push();
            len += r.len;
            acc = O::binary_operation(&acc, &r.acc);
        });
        self.len = len;
        self.acc = acc;
    }

    fn rotate_left(node_ref: NodeRef<'a, K, O>) {
        let mut node = node_ref.borrow_mut();
        let p_ref = node.p.unwrap().1;
        let mut p = p_ref.borrow_mut();

        let l = node.l.take();
        Self::if_present(&l, |l| {
            l.p = Some((Dir::Right, p_ref));
        });
        p.r = l;

        node.p = p.p.take();
        node.l = Some(p_ref);
        match node.p {
            Some((Dir::Left, pp)) => pp.borrow_mut().l = Some(node_ref),
            Some((Dir::Right, pp)) => pp.borrow_mut().r = Some(node_ref),
            None => {}
        }

        p.p = Some((Dir::Left, node_ref));
        p.pull();
        drop(p);
        node.pull();
    }

    fn rotate_right(node_ref: NodeRef<'a, K, O>) {
        let mut node = node_ref.borrow_mut();
        let p_ref = node.p.unwrap().1;
        let mut p = p_ref.borrow_mut();

        let r = node.r.take();
        Self::if_present(&r, |r| {
            r.p = Some((Dir::Left, p_ref));
        });
        p.l = r;

        node.p = p.p.take();
        node.r = Some(p_ref);
        match node.p {
            Some((Dir::Left, pp)) => pp.borrow_mut().l = Some(node_ref),
            Some((Dir::Right, pp)) => pp.borrow_mut().r = Some(node_ref),
            None => {}
        }

        p.p = Some((Dir::Right, node_ref));
        p.pull();
        drop(p);
        node.pull();
    }

    fn splay(node_ref: NodeRef<'a, K, O>) {
        loop {
            let Some((dir, p_ref)) = node_ref.borrow().p else {
                node_ref.borrow_mut().push();
                return;
            };

            let Some((p_dir, pp_ref)) = p_ref.borrow().p else {
                p_ref.borrow_mut().push();
                node_ref.borrow_mut().push();
                match dir {
                    Dir::Left => Self::rotate_right(node_ref),
                    Dir::Right => Self::rotate_left(node_ref),
                }
                return;
            };
            pp_ref.borrow_mut().push();
            p_ref.borrow_mut().push();
            node_ref.borrow_mut().push();
            match (p_dir, dir) {
                (Dir::Left, Dir::Left) => {
                    Self::rotate_right(p_ref);
                    Self::rotate_right(node_ref);
                }
                (Dir::Right, Dir::Right) => {
                    Self::rotate_left(p_ref);
                    Self::rotate_left(node_ref);
                }
                (Dir::Left, Dir::Right) => {
                    Self::rotate_left(node_ref);
                    Self::rotate_right(node_ref);
                }
                (Dir::Right, Dir::Left) => {
                    Self::rotate_right(node_ref);
                    Self::rotate_left(node_ref);
                }
            }
        }
    }

    fn leftmost(mut node_ref: NodeRef<'a, K, O>) -> NodeRef<'a, K, O> {
        while let Some(l) = node_ref.borrow().l {
            node_ref = l
        }
        node_ref
    }

    fn rightmost(mut node_ref: NodeRef<'a, K, O>) -> NodeRef<'a, K, O> {
        while let Some(r) = node_ref.borrow().r {
            node_ref = r
        }
        node_ref
    }

    fn join(lhs: NodeRef<'a, K, O>, rhs: NodeRef<'a, K, O>) -> NodeRef<'a, K, O> {
        Self::splay(lhs);
        let lhs = Self::rightmost(lhs);
        Self::splay(lhs);
        Self::splay(rhs);
        Self::link_right_child(lhs, rhs);
        lhs.borrow_mut().pull();
        lhs
    }

    /// Only sets up parent/child references.
    /// Caller must ensure
    /// - push on parent is called before this, and
    /// - pull on parent is called after this.
    fn link_right_child(p_ref: NodeRef<'a, K, O>, r_ref: NodeRef<'a, K, O>) {
        let mut p = p_ref.borrow_mut();
        p.r.replace(r_ref);
        r_ref.borrow_mut().p.replace((Dir::Right, p_ref));
    }

    /// Only unsets parent/child references.
    /// Caller must ensure that a push on parent is called before this, and
    fn cut_left_child(p_ref: NodeRef<'a, K, O>) -> Option<NodeRef<'a, K, O>> {
        let mut p = p_ref.borrow_mut();
        if let Some(l) = p.l.take() {
            l.borrow_mut().p = None;
            p.pull();
            Some(l)
        } else {
            None
        }
    }
    /// Only unsets parent/child references.
    /// Caller must ensure that a push on parent is called before this, and
    fn cut_right_child(p_ref: NodeRef<'a, K, O>) -> Option<NodeRef<'a, K, O>> {
        let mut p = p_ref.borrow_mut();
        if let Some(r) = p.r.take() {
            r.borrow_mut().p = None;
            p.pull();
            Some(r)
        } else {
            None
        }
    }
}

pub struct Trees<'arena, K, O: SeqOps> {
    arena: &'arena crate::util::arena::Arena<RefCell<Node<'arena, K, O>>>,
}
pub type Sequences<'arena, O> = Trees<'arena, (), O>;

impl<'arena, K, O: SeqOps> From<&'arena crate::util::arena::Arena<RefCell<Node<'arena, K, O>>>>
    for Trees<'arena, K, O>
{
    fn from(arena: &'arena crate::util::arena::Arena<RefCell<Node<'arena, K, O>>>) -> Self {
        Self { arena }
    }
}
impl<'arena, K, O: SeqOps> Trees<'arena, K, O> {
    pub fn new(arena: &'arena crate::util::arena::Arena<RefCell<Node<'arena, K, O>>>) -> Self {
        Self { arena }
    }

    pub fn empty_tree(&self) -> TreeHandle<'arena, K, O> {
        TreeHandle(None)
    }

    pub fn singleton_tree(&self, key: K, value: O::Value) -> TreeHandle<'arena, K, O> {
        TreeHandle(Some(self.arena.allocate(Node::create(key, value))))
    }
}
impl<'arena, O: SeqOps> Sequences<'arena, O> {
    pub fn empty_sequence(&'arena self) -> SeqHandle<'arena, O> {
        self.empty_tree()
    }
    pub fn singleton_sequence(&'arena self, value: O::Value) -> SeqHandle<'arena, O> {
        self.singleton_tree((), value)
    }
}
pub struct TreeHandle<'arena, K, O: SeqOps>(Option<NodeRef<'arena, K, O>>);
pub type SeqHandle<'arena, O> = TreeHandle<'arena, (), O>;

impl<'arena, O: SeqOps> SeqHandle<'arena, O> {
    pub fn concat(TreeHandle(lhs): &Self, TreeHandle(rhs): &Self) -> Self {
        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            Self(Some(Node::join(lhs, rhs)))
        } else {
            Self(lhs.or(*rhs))
        }
    }

    pub fn append(&self, other: &Self) {
        Self::concat(self, other);
    }

    pub fn reverse(&self) {
        if let Some(node) = self.0 {
            Node::splay(node);
            node.borrow_mut().rev ^= true;
        }
    }
}

impl<'arena, K, O: SeqOps> TreeHandle<'arena, K, O> {
    pub fn len(&self) -> usize {
        self.0.map_or(0, |node| {
            Node::splay(node);
            node.borrow().len
        })
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn split_by_size(&self, k: usize) -> (Self, Self) {
        let len = self.len();
        assert!(k <= len);
        let Some(mut node) = self.0 else {
            return (Self(None), Self(None));
        };
        if k == 0 {
            return (Self(None), Self(Some(self.0.unwrap())));
        }
        if k == len {
            return (Self(Some(self.0.unwrap())), Self(None));
        }
        Node::splay(node);
        let mut s = 0;
        let node = loop {
            let l = Node::if_present(&node.borrow().l, |l| l.len).unwrap_or(0);
            match (s + l + 1).cmp(&k) {
                std::cmp::Ordering::Less => {
                    s += l + 1;
                    node = node.borrow().r.unwrap();
                }
                std::cmp::Ordering::Equal => {
                    break node;
                }
                std::cmp::Ordering::Greater => {
                    node = node.borrow().l.unwrap();
                }
            }
        };
        Node::splay(node);
        let r = Node::cut_right_child(node);
        (Self(Some(node)), Self(r))
    }

    pub fn leftmost(&self) -> Option<(Ref<'arena, K>, Ref<'arena, <O as SeqOps>::Value>)> {
        self.0.map(|node| {
            Node::splay(node);
            let l = Node::leftmost(node);
            Node::splay(l);
            Ref::map_split(l.borrow(), |l| (&l.key, &l.value))
        })
    }

    pub fn rightmost(&self) -> Option<(Ref<'arena, K>, Ref<'arena, <O as SeqOps>::Value>)> {
        self.0.map(|node| {
            Node::splay(node);
            let r = Node::rightmost(node);
            Node::splay(r);
            Ref::map_split(r.borrow(), |r| (&r.key, &r.value))
        })
    }

    pub fn all_product(&self) -> Option<Ref<'arena, <O as SeqOps>::Acc>> {
        self.0.map(|node| {
            Node::splay(node);
            Ref::map(node.borrow(), |node| &node.acc)
        })
    }
}
fn visit<K, O: SeqOps>(root: &Node<'_, K, O>, f: &mut impl FnMut(&K, &O::Value, &O::Acc)) {
    Node::if_present(&root.l, |l| visit(l, f));
    f(&root.key, &root.value, &root.acc);
    Node::if_present(&root.r, |r| visit(r, f));
}

impl<'arena, K: Ord + Clone, O: SeqOps> std::fmt::Debug for TreeHandle<'arena, K, O>
where
    K: std::fmt::Debug,
    O::Value: std::fmt::Debug,
    O::Acc: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dm = f.debug_map();
        if let Some(mut node) = self.0 {
            while let Some(p) = node.borrow().p {
                node = p.1;
            }
            visit(&node.borrow(), &mut |k, v, acc| {
                dm.entry(k, &(v, acc));
            });
        }
        dm.finish()
    }
}

impl<'arena, K: Ord + Clone, O: SeqOps> TreeHandle<'arena, K, O>
where
    K: std::fmt::Debug,
    O::Value: std::fmt::Debug,
    O::Acc: std::fmt::Debug,
{
    /// (true, false)
    pub fn partition_with_key(&self, cmp: impl Fn(&K) -> bool) -> (Self, Self) {
        let Some(mut node) = self.0 else {
            return (Self(None), Self(None));
        };
        Node::splay(node);
        loop {
            let c = cmp(&node.borrow().key);
            if c {
                if let Some(r) = node.borrow().r {
                    node = r;
                    continue;
                }
                Node::splay(node);
                let r = Node::cut_right_child(node);
                return (Self(Some(node)), Self(r));
            } else {
                if let Some(l) = node.borrow().l {
                    node = l;
                    continue;
                }
                Node::splay(node);
                let l = Node::cut_left_child(node);
                return (Self(l), Self(Some(node)));
            }
        }
    }

    pub fn meld(lhs: &Self, rhs: &Self) -> Self {
        if lhs.is_empty() {
            return Self(rhs.0);
        }
        if rhs.is_empty() {
            return Self(lhs.0);
        }
        let (a, b, pivot) = match (lhs.leftmost(), rhs.leftmost()) {
            (Some((l, _)), Some((r, _))) => {
                if l.cmp(&r).is_le() {
                    (lhs, rhs, r.clone())
                } else {
                    (rhs, lhs, l.clone())
                }
            }
            (_, _) => return Self(lhs.0.or(rhs.0)),
        };
        let (smaller, larger) = a.partition_with_key(|k| *k <= pivot);
        let larger = Self::meld(b, &larger);
        if let (Self(Some(a)), Self(Some(b))) = (&smaller, &larger) {
            Self(Some(Node::join(a, b)))
        } else {
            Self(smaller.0.or(larger.0))
        }
    }
}

pub enum Reversible<O> {
    _Phantom(std::marker::PhantomData<O>, std::convert::Infallible),
}
impl<O: SeqOps> SeqOps for Reversible<O> {
    type Value = <O as SeqOps>::Value;

    type Acc = (<O as SeqOps>::Acc, <O as SeqOps>::Acc);

    fn value_to_acc(val: &Self::Value) -> Self::Acc {
        (O::value_to_acc(val), O::value_to_acc(val))
    }

    fn identity() -> Self::Acc {
        (O::identity(), O::identity())
    }

    fn binary_operation(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc {
        (
            O::binary_operation(&lhs.0, &rhs.0),
            O::binary_operation(&rhs.1, &lhs.1),
        )
    }

    fn reverse(_val: &mut Self::Value, acc: &mut Self::Acc) {
        std::mem::swap(&mut acc.0, &mut acc.1);
    }
}

impl<M: ac_library::Monoid> SeqOps for M {
    type Value = <Self as ac_library::Monoid>::S;

    type Acc = <Self as ac_library::Monoid>::S;

    fn value_to_acc(val: &Self::Value) -> Self::Acc {
        val.clone()
    }

    fn identity() -> Self::Acc {
        <Self as ac_library::Monoid>::identity()
    }

    fn binary_operation(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc {
        <Self as ac_library::Monoid>::binary_operation(lhs, rhs)
    }
}

#[test]
fn test() {
    enum Nop {}
    impl SeqOps for Nop {
        type Value = usize;

        type Acc = ();

        fn value_to_acc(_val: &Self::Value) -> Self::Acc {}

        fn identity() -> Self::Acc {}

        fn binary_operation(_lhs: &Self::Acc, _rhs: &Self::Acc) -> Self::Acc {}
    }
    let arena = crate::util::arena::Arena::new();
    let seq = Trees::<(), Nop>::new(&arena);
    let a = seq.singleton_tree((), 0);
    let seq2 = Trees::<(), Nop>::new(&arena);
    let b = seq2.singleton_tree((), 1);
    a.append(&b);
    assert_eq!(a.len(), 2);
    assert_eq!(b.len(), 2);
}
