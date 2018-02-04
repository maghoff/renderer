use std::cmp::{max, min};
use cgmath::prelude::*;
use cgmath::Vector2;

#[repr(C)]
#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

const SQUARE_SZ: f64 = 64.;
const TAU: f64 = 2. * ::std::f64::consts::PI;
const WALL_HEIGHT: f64 = 64.;

const CEIL: Pixel = Pixel { r: 255, g: 0, b: 0, a: 255 };
const WALL: Pixel = Pixel { r: 0, g: 255, b: 0, a: 255 };
const FLOOR: Pixel = Pixel { r: 0, g: 0, b: 255, a: 255 };

const MAP: &[u8] = b"\
    xxxxxxxxxx\
    x   x    x\
    x      x x\
    x        x\
    x        x\
    x x      x\
    x        x\
    x        x\
    x        x\
    x        x\
    x        x\
    x        x\
    x        x\
    xxxxxxxxxx";

const MAP_W: i32 = 10;
const MAP_H: i32 = 14;

fn is_wall(cell: &Vector2<i32>) -> bool {
    assert!(0 <= cell.x);
    assert!(cell.x < MAP_W);
    assert!(0 <= cell.y);
    assert!(cell.y < MAP_H);

    MAP[(cell.y * MAP_W + cell.x) as usize] == b'x'
}

fn is_wall_f(coord: Vector2<f64>) -> bool {
    is_wall(&(coord / SQUARE_SZ).cast().unwrap())
}

fn cast_ray(o: Vector2<f64>, dir: Vector2<f64>) -> Vector2<f64> {
    let origin_cell: Vector2<i32> = (o / SQUARE_SZ).cast().unwrap();

    // Check horizontal intersections

    // Find first intersection point
    let mut dy = o.y - (origin_cell.y as f64) * SQUARE_SZ;
    if dir.y > 0. {
        dy = SQUARE_SZ - dy;
    }
    let dist = dy / dir.y; // Degenerate: Division by something that could be close to zero

    let first_horizontal_intersection_coord = o + dir * dist;

    // Scan map rows for intersections
    let row_delta = dir * (SQUARE_SZ / dir.y.abs()); // Degenerate: Division by something that could be close to zero

    let good_measure = Vector2::new(
        0.,
        if dir.y > 0. {
            SQUARE_SZ / 2.
        } else {
            -SQUARE_SZ / 2.
        }
    );

    let mut coord = first_horizontal_intersection_coord;
    let mut cell = origin_cell;
    loop {
        let x = (coord.x / SQUARE_SZ).floor() as i32;
        if (x != cell.x) && is_wall(&Vector2::new(x, cell.y)) {
            // It is apparent that row_delta.x is not near zero, since we
            // have come to a different column on the map

            let intersection_x =
                (if row_delta.x > 0. { cell.x+1 } else { cell.x }) as f64
                * SQUARE_SZ;

            let rows = (intersection_x - o.x) / row_delta.x;

            return o + row_delta * rows;
        }
        cell.x = x;
        cell.y -= 1;

        if is_wall_f(coord + good_measure) {
            return coord;
        }
        coord += row_delta;
    }
}

pub fn render(buf: &mut [Pixel], screen_width: usize, screen_height: usize, _time: f64) {
    // Hard-coded input:
    let projection_plane_width = 320.;
    let fov = 60. * TAU / 360.;

    let pos = SQUARE_SZ * Vector2::new(5 as f64, 12 as f64);
    let dir = Vector2::new(0., -1.).normalize();
    // --

    let projection_plane_half_width = projection_plane_width / 2.;
    let distance_to_projection_plane = projection_plane_half_width / (fov / 2.).tan();

    let side = Vector2::new(-dir.y, dir.x);

    let projection_plane_center = distance_to_projection_plane * dir;
    let projection_plane_left = projection_plane_center - projection_plane_half_width * side;

    let dside = side * projection_plane_width / (screen_width as f64);

    for x in 0..screen_width {
        // Add 0.5 dside to cast the ray in the center of the column
        let ray_dir = projection_plane_left + dside * 0.5 + dside * (x as f64);

        let intersection_point = cast_ray(pos, ray_dir.normalize());
        let z = (intersection_point - &pos).dot(dir);
        let w = 1./z;

        let projected_height = w * WALL_HEIGHT * distance_to_projection_plane;
        assert!(projected_height >= 0.);

        let mid = screen_height as f64 / 2.;
        let ceil = max((mid - projected_height/2.).floor() as usize, 0);
        let floor = min((mid + projected_height/2.).floor() as usize, screen_height);

        for y in 0..ceil {
            buf[y*screen_width + x] = CEIL;
        }

        for y in ceil..floor {
            buf[y*screen_width + x] = WALL;
        }

        for y in floor..screen_height {
            buf[y*screen_width + x] = FLOOR;
        }
    }
}
