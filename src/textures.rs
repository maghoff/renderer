use ndarray::prelude::*;

use screen::Pixel;

pub struct Textures<'a> {
    data: ArrayView5<'a, Pixel>
}

impl<'a> Textures<'a> {
    pub fn new(data: ArrayView5<'a, Pixel>) -> Textures<'a> {
        Textures {
            data
        }
    }

    pub fn tx(&self, i: u8) -> ArrayView2<Pixel> {
        let i = i as usize;
        self.data.slice(s![i / 3, i % 3, 0, .., ..])
    }
}
