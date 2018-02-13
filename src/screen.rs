use ndarray::ArrayViewMut2;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait Screen {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn px(&mut self, x: usize, y: usize) -> &mut Pixel;
}

impl<'a> Screen for ArrayViewMut2<'a, Pixel> {
    fn width(&self) -> usize {
        self.dim().1
    }

    fn height(&self) -> usize {
        self.dim().0
    }

    fn px(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self[[y, x]]
    }
}
