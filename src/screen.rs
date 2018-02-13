use ndarray::ArrayViewMut2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Screen<'a>(ArrayViewMut2<'a, Pixel>);

impl<'a> Screen<'a> {
    pub fn new(buf: &mut [Pixel], width: usize, height: usize) -> Screen {
        assert!(buf.len() == width * height);
        let buf = ArrayViewMut2::from_shape((height, width), buf).unwrap();

        Screen(buf)
    }

    pub fn width(&self) -> usize { self.0.dim().1 }
    pub fn height(&self) -> usize { self.0.dim().0 }

    pub fn px(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.0[[y, x]]
    }
}
