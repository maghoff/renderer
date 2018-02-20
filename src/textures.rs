use ndarray::prelude::*;

use screen::Pixel;

#[derive(Copy, Clone)]
pub enum Side {
    NorthSouth,
    WestEast
}

impl Side {
    pub fn flipped(self) -> Side {
        match self {
            Side::NorthSouth => Side::WestEast,
            Side::WestEast => Side::NorthSouth,
        }
    }

    fn index(self) -> usize {
        match self {
            Side::NorthSouth => 0,
            Side::WestEast => 1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextureSpec {
    pub tx: u8,
    pub side: Side,
}

impl TextureSpec {
    pub fn flipped(self) -> TextureSpec {
        TextureSpec {
            tx: self.tx,
            side: self.side.flipped(),
        }
    }
}

pub struct Textures<'a> {
    data: ArrayView5<'a, Pixel>
}

impl<'a> Textures<'a> {
    pub fn new(data: ArrayView5<'a, Pixel>) -> Textures<'a> {
        Textures {
            data
        }
    }

    pub fn tx(&self, tx: TextureSpec) -> ArrayView2<Pixel> {
        let i = tx.tx as usize;
        self.data.slice(s![i / 3, i % 3, tx.side.index(), .., ..])
    }
}
