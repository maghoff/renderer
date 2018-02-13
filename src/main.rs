#![feature(allocator_api)]

extern crate cgmath;
extern crate ndarray;

mod core;
mod screen;

use std::heap::{Alloc, Heap, Layout};
use std::mem;
use std::slice;

use cgmath::Vector2;
use ndarray::ArrayViewMut2;

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
pub fn fill(
    screen_ptr: *mut u8, screen_width: usize, screen_height: usize,
    cx: f64, cy: f64,
    dx: f64, dy: f64
) {
    let screen_buf = unsafe {
        slice::from_raw_parts_mut(
            std::mem::transmute(screen_ptr),
            screen_width * screen_height
        )
    };
    let mut screen = ArrayViewMut2::from_shape(
        (screen_height, screen_width),
        screen_buf
    ).unwrap();

    let pos = Vector2::new(cx, cy);
    let dir = Vector2::new(dx, dy);

    core::render(&mut screen, pos, dir);
}

fn main() {
    // We need main() for everything to work out with wasm.
    // ... I think
}
