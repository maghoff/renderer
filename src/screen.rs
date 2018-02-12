#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Screen<'a> {
    buf: &'a mut [Pixel],
    width: usize,
    height: usize,
}

impl<'a> Screen<'a> {
    pub fn new(buf: &mut [Pixel], width: usize, height: usize) -> Screen {
        assert!(buf.len() == width * height);

        Screen {
            buf,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn px(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.buf[y * self.width + x]
    }
}
