#![feature(allocator_api)]

extern crate cgmath;

mod core;
mod screen;

use std::heap::{Alloc, Heap, Layout};
use std::mem;
use std::slice;

use cgmath::Vector2;

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
pub fn fill(pointer: *mut u8, width: usize, height: usize, cx: f64, cy: f64, dx: f64, dy: f64) {
    let pitch = width * 4;
    let buf_sz = pitch * height;
    let buf = unsafe { slice::from_raw_parts_mut(std::mem::transmute(pointer), buf_sz) };
    let screen = screen::Screen::new(buf, width, height);

    let pos = Vector2::new(cx, cy);
    let dir = Vector2::new(dx, dy);

    core::render(screen, pos, dir);
}

fn main() {
    // We need main() for everything to work out with wasm.
    // ... I think
}
