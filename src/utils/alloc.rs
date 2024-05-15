use std::alloc::{GlobalAlloc, System};

/// Allocator that doesn't free.
///
/// # Examples
///
/// ```
/// use ralgo::utils::alloc::LeakyAlloc;
///
/// #[global_allocator]
/// static GLOBAL_ALLOC: LeakyAlloc = LeakyAlloc::system();
/// ```
///
/// ```
/// use ralgo::utils::alloc::LeakyAlloc;
/// # pub struct WhateverGlobalAlloc;
/// # unsafe impl std::alloc::GlobalAlloc for WhateverGlobalAlloc {
/// #     unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
/// #         std::alloc::System.alloc(layout)
/// #     }
/// #     unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
/// #         std::alloc::System.dealloc(ptr, layout)
/// #     }
/// # }
///
/// #[global_allocator]
/// static GLOBAL_ALLOC: LeakyAlloc<WhateverGlobalAlloc> = LeakyAlloc(WhateverGlobalAlloc);
/// ```

pub struct LeakyAlloc<A: GlobalAlloc = System>(pub A);
impl LeakyAlloc<System> {
    pub const fn system() -> Self {
        Self(System)
    }
}

unsafe impl<A: GlobalAlloc> GlobalAlloc for LeakyAlloc<A> {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {
        // leak!
    }

    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        self.0.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        self.0.realloc(ptr, layout, new_size)
    }
}
