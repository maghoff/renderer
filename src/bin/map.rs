#[macro_use] extern crate ndarray;
use ndarray::*;

const MAP_DATA: &[u8; 16384] = include_bytes!("map.bin");

fn wall(w: u16) -> char {
    match w {
        0 => ' ',
        wall if 0 < wall && wall <= 0x38 => {
            (match wall - 1 {
                x if x < 10 => b'0' + x as u8,
                x if x < 36 => b'a' + x as u8 - 10,
                x => b'A' + x as u8 - 36,
            }) as char
        },
        // Doors?
        floor if 0x38 < floor => ' ',
        _ => '?',
    }
}

fn main() {
    let map: &[u16; 8192] = unsafe { std::mem::transmute(MAP_DATA) };
    let map = ArrayView3::from_shape((2, 64, 64).strides((64*64, 1, 64)), map).unwrap();

    let walls = map.slice(s![0, .., ..]);
    let _objs = map.slice(s![1, .., ..]);

    for row in walls.outer_iter() {
        print!("    \"");
        for &col in row {
            print!("{}", wall(col));
        }
        println!("\" +");
    }
}
