use std::cmp::{max, min};
use cgmath::prelude::*;
use cgmath::Vector2;
use ndarray::{ArrayView2, ArrayViewMut2};

use screen::{Pixel, Screen};
use textures::Textures;

const TAU: f64 = 2. * ::std::f64::consts::PI;
const WALL_HEIGHT: f64 = 64.;

const CEIL: Pixel = Pixel { r: 255, g: 0, b: 0, a: 255 };
const FLOOR: Pixel = Pixel { r: 0, g: 0, b: 255, a: 255 };

pub fn render<F>(map: ArrayView2<u8>, screen: &mut ArrayViewMut2<Pixel>, textures: &Textures, pos: Vector2<f64>, dir: Vector2<f64>, cast_ray: F)
where
    F: Fn(ArrayView2<u8>, Vector2<f64>, Vector2<f64>) -> Option<(Vector2<f64>, f64, u8)>
{
    // Hard-coded input:
    let projection_plane_width = 320.;
    let fov = 60. * TAU / 360.;
    // --

    let projection_plane_half_width = projection_plane_width / 2.;
    let distance_to_projection_plane = projection_plane_half_width / (fov / 2.).tan();

    let side = Vector2::new(-dir.y, dir.x);

    let projection_plane_center = distance_to_projection_plane * dir;
    let projection_plane_left = projection_plane_center - projection_plane_half_width * side;

    let dside = side * projection_plane_width / (screen.width() as f64);

    for x in 0..screen.width() {
        // Add 0.5 dside to cast the ray in the center of the column
        let ray_dir = projection_plane_left + dside * 0.5 + dside * (x as f64);

        let (projected_height, u, tx) = match cast_ray(map, pos, ray_dir.normalize()) {
            Some((intersection_point, u, tx)) => {
                let z = (intersection_point - &pos).dot(dir);
                let w = 1./z;

                (w * WALL_HEIGHT * distance_to_projection_plane, u, tx)
            },
            None => (0., 0., 0)
        };

        let mid = screen.height() as f64 / 2.;
        let ceil = max((mid - projected_height/2.).floor() as isize, 0) as usize;
        let floor = min((mid + projected_height/2.).floor() as usize, screen.height());

        for y in 0..ceil {
            *screen.px(x, y) = CEIL;
        }

        let texture = textures.tx(tx);

        let dv = 64. / projected_height as f64;
        let mut v = ((mid - projected_height/2.).floor() - ceil as f64) * -dv;
        for y in ceil..floor {
            *screen.px(x, y) = texture[[v as usize, u as usize]];
            v += dv;
        }

        for y in floor..screen.height() {
            *screen.px(x, y) = FLOOR;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ndarray::prelude::*;

    use consts::*;
    use continuous;

    #[test]
    fn can_render() {
        let mut screen = Array2::default((200, 320));
        let map = ArrayView2::from_shape(
            (5, 5),
            b"\
            xxxxx\
            x   x\
            x   x\
            x   x\
            xxxxx"
        ).unwrap();
        let textures = Array5::default((19, 3, 2, 64, 64));
        let textures = Textures::new(textures.view());

        let pos = Vector2::new(map.dim().1 as f64 / 2. * SQUARE_SZ, map.dim().0 as f64 / 2. * SQUARE_SZ);
        for ang in 0..10 {
            let rad = ang as f64 * TAU / 10.;
            let dir = Vector2::new(rad.cos(), rad.sin());
            render(map, &mut screen.view_mut(), &textures, pos, dir, continuous::cast_ray);
        }
    }

    #[test]
    fn can_render_holes() {
        let mut screen = Array2::default((200, 320));
        let map = ArrayView2::from_shape(
            (5, 5),
            b"\
            xx xx\
            x   x\
            x    \
            x   x\
            xx xx"
        ).unwrap();
        let textures = Array5::default((19, 3, 2, 64, 64));
        let textures = Textures::new(textures.view());

        let pos = Vector2::new(map.dim().1 as f64 / 2. * SQUARE_SZ, map.dim().0 as f64 / 2. * SQUARE_SZ);
        for ang in 0..10 {
            let rad = ang as f64 * TAU / 10.;
            let dir = Vector2::new(rad.cos(), rad.sin());
            render(map, &mut screen.view_mut(), &textures, pos, dir, continuous::cast_ray);
        }
    }
}
