#[repr(C)]
#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn render(buf: &mut [Pixel], width: usize, height: usize, time: f64) {
    for y in 0..height {
        for x in 0..width {
            let p = &mut buf[y*width + x];

            *p = Pixel {
                r: {
                    let len = ((y*y + x*x) as f64).sqrt();
                    let nb = time  + len / 12.0;
                    let a = 128.0 + nb.cos() * 128.0;
                    a as u8
                },
                g: 0,
                b: {
                    let x = width - x;
                    let len = ((y*y + x*x) as f64).sqrt();
                    let nb = time  + len / 12.0;
                    let a = 128.0 + nb.cos() * 128.0;
                    a as u8
                },
                a: 255
            };
        }
    }
}
