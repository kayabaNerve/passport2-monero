// SPDX-FileCopyrightText: 2023 Foundation Devices, Inc. <hello@foundationdevices.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types)]

extern crate alloc;

pub mod monero;
pub mod ur;

/// cbindgen:ignore
#[cfg(not(feature = "std"))]
mod stdout;

/// cbindgen:ignore
#[cfg(cortex_m)]
#[global_allocator]
static HEAP: embedded_alloc::Heap = embedded_alloc::Heap::empty();

// If we're not building for a cortex_m, it's the Unix port
/// cbindgen:ignore
#[cfg(not(cortex_m))]
mod allocator {
    use core::{
        alloc::{GlobalAlloc, Layout},
        ptr,
    };

    extern "C" {
        pub fn free(p: *mut u8);
        pub fn posix_memalign(
            memptr: *mut *mut u8,
            align: usize,
            size: usize,
        ) -> i32;
        pub fn realloc(p: *mut u8, size: usize) -> *mut u8;
    }

    struct Allocator;
    unsafe impl GlobalAlloc for Allocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let mut ptr = ptr::null_mut();
            let ret = posix_memalign(
                &mut ptr,
                layout.align().max(core::mem::size_of::<usize>()),
                layout.size(),
            );
            if ret == 0 {
                ptr as *mut u8
            } else {
                ptr::null_mut()
            }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let ptr = self.alloc(layout);
            if !ptr.is_null() {
                ptr::write_bytes(ptr, 0, layout.size());
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
            free(ptr);
        }

        unsafe fn realloc(
            &self,
            ptr: *mut u8,
            _: Layout,
            new_size: usize,
        ) -> *mut u8 {
            realloc(ptr, new_size)
        }
    }

    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;
}

#[cfg(not(cortex_m))]
#[no_mangle]
pub extern "C" fn setup_alloc() {}

#[cfg(cortex_m)]
#[no_mangle]
pub extern "C" fn setup_alloc() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 200_000;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] =
        [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

#[panic_handler]
#[cfg(all(not(feature = "std"), not(test)))]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let mut stdout = stdout::Stdout;
    writeln!(stdout, "{}", info).ok();

    loop {}
}
