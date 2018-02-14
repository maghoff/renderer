#![feature(allocator_api)]

extern crate cgmath;
extern crate ndarray;

mod core;
mod screen;

use std::heap::{Alloc, Heap, Layout};
use std::mem;
use std::slice;

use cgmath::Vector2;
use ndarray::{ArrayView2, ArrayViewMut2};

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
    map_ptr: *mut u8, map_width: usize, map_height: usize,
    screen_ptr: *mut u8, screen_width: usize, screen_height: usize,
    cx: f64, cy: f64,
    dx: f64, dy: f64
) {
    let screen_slice = unsafe {
        slice::from_raw_parts_mut(
            std::mem::transmute(screen_ptr),
            screen_width * screen_height
        )
    };
    let mut screen = ArrayViewMut2::from_shape(
        (screen_height, screen_width),
        screen_slice
    ).unwrap();

    let map_slice = unsafe {
        slice::from_raw_parts(
            std::mem::transmute(map_ptr),
            map_width * map_height
        )
    };
    let map = ArrayView2::from_shape((map_height, map_width), map_slice).unwrap();

    let pos = Vector2::new(cx, cy);
    let dir = Vector2::new(dx, dy);

    core::render(map, &mut screen, pos, dir);
}

fn main() {
    // We need main() for everything to work out with wasm.
    // ... I think
}
