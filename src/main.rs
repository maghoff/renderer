#![feature(allocator_api)]

#[macro_use] extern crate ndarray;
extern crate cgmath;

mod consts;
mod core;
mod ray;
mod screen;
mod textures;

use std::heap::{Alloc, Heap, Layout};
use std::mem;
use std::slice;

use cgmath::Vector2;
use ndarray::prelude::*;

use textures::Textures;

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
    textures_ptr: *mut u8, textures_width: usize, textures_height: usize,
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

    let textures_slice: &[screen::Pixel] = unsafe {
        slice::from_raw_parts(
            std::mem::transmute(textures_ptr),
            textures_width * textures_height
        )
    };
    let textures = ArrayView5::from_shape(
        (19, 3, 2, 64, 64).strides((64*6*64, 64*2, 64, 64*6, 1)),
        textures_slice
    ).unwrap();
    let textures = Textures::new(textures);

    let pos = Vector2::new(cx, cy);
    let dir = Vector2::new(dx, dy);

    core::render(map, &mut screen, &textures, pos, dir, ray::cast_ray);
}

fn main() {
    // We need main() for everything to work out with wasm.
    // ... I think
}
