#![feature(allocator_api)]

extern crate cgmath;

mod core;

use std::heap::{Alloc, Heap, Layout};
use std::mem;

use std::slice;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        Heap.alloc(layout).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    unsafe  {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        Heap.dealloc(ptr, layout);
    }
}

#[no_mangle]
pub fn fill(pointer: *mut u8, width: usize, height: usize, time: f64) {
    let pitch = width * 4;
    let buf_sz = pitch * height;

    let buf = unsafe { slice::from_raw_parts_mut(std::mem::transmute(pointer), buf_sz) };

    core::render(buf, width, height, time);
}

fn main() {
    // We need main() for everything to work out with wasm.
    // ... I think
}
