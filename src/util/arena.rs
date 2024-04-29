use std::cell::RefCell;

struct Inner<T> {
    buffer: Vec<T>,
    old_buffers: Vec<Vec<T>>,
}

pub struct Arena<T> {
    inner: RefCell<Inner<T>>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            inner: RefCell::new(Inner {
                buffer: Default::default(),
                old_buffers: Default::default(),
            }),
        }
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            inner: RefCell::new(Inner {
                buffer: Vec::with_capacity(n),
                old_buffers: Default::default(),
            }),
        }
    }

    pub fn reserve(&self, n: usize) {
        let mut inner = self.inner.borrow_mut();
        if inner.buffer.len() + n > inner.buffer.capacity() {
            let n = n.max(inner.buffer.capacity() * 2);
            let buf = std::mem::replace(&mut inner.buffer, Vec::with_capacity(n));
            inner.old_buffers.push(buf);
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn allocate(&self, value: impl Into<T>) -> &mut T {
        let mut inner = self.inner.borrow_mut();
        if inner.buffer.len() == inner.buffer.capacity() {
            let n = inner.buffer.capacity() * 2;
            let buf = std::mem::replace(&mut inner.buffer, Vec::with_capacity(n));
            inner.old_buffers.push(buf);
        }
        let pos = inner.buffer.len();
        inner.buffer.push(value.into());
        unsafe { &mut *inner.buffer.as_mut_ptr().add(pos) }
    }
}

impl<T: Default> Arena<T> {
    pub fn allocate_default(&self) -> &mut T {
        self.allocate(T::default())
    }
}
