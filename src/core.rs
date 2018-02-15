use std::cmp::{max, min};
use cgmath::prelude::*;
use cgmath::Vector2;
use screen::{Pixel, Screen};
use ndarray::{ArrayView2, ArrayViewMut2};

const SQUARE_SZ: f64 = 64.;
const TAU: f64 = 2. * ::std::f64::consts::PI;
const WALL_HEIGHT: f64 = 64.;

const CEIL: Pixel = Pixel { r: 255, g: 0, b: 0, a: 255 };
const WALL: Pixel = Pixel { r: 0, g: 255, b: 0, a: 255 };
const FLOOR: Pixel = Pixel { r: 0, g: 0, b: 255, a: 255 };

fn is_wall(map: ArrayView2<u8>, cell: &Vector2<i32>) -> bool {
    let cell = cell.cast().unwrap();
    map[[cell.y, cell.x]] == b'x'
}

fn is_wall_f(map: ArrayView2<u8>, coord: Vector2<f64>) -> bool {
    is_wall(map, &(coord / SQUARE_SZ).cast().unwrap())
}

fn cast_vertical_ray(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    assert!(dir.y.abs() > 0.7); // We can divide by dir.y

    let origin_cell: Vector2<i32> = (o / SQUARE_SZ).cast().unwrap();

    // Check horizontal intersections

    // Find first intersection point
    // BUG! First intersection point could be vertical!
    let mut dy = o.y - (origin_cell.y as f64) * SQUARE_SZ;
    if dir.y > 0. {
        dy = SQUARE_SZ - dy;
    }
    let dist = dy / dir.y.abs();

    let first_horizontal_intersection_coord = o + dir * dist;

    // Scan map rows for intersections
    let row_delta = dir * (SQUARE_SZ / dir.y.abs());

    let good_measure = Vector2::new(
        0.,
        if dir.y > 0. {
            SQUARE_SZ / 2.
        } else {
            -SQUARE_SZ / 2.
        }
    );

    let mut coord = first_horizontal_intersection_coord;
    let mut cell: Vector2<i32> = ((coord + good_measure) / SQUARE_SZ).cast().unwrap();
    loop {
        let x = (coord.x / SQUARE_SZ).floor() as i32;
        if x < 0 || x >= map.dim().1 as i32 { return None; }

        if (x != cell.x) && is_wall(map, &Vector2::new(x, cell.y)) {
            let intersection_x =
                (if row_delta.x > 0. { cell.x+1 } else { cell.x }) as f64
                * SQUARE_SZ;

            // It is apparent that row_delta.x is not near zero, since we
            // have come to a different column on the map

            let rows = (intersection_x - o.x) / row_delta.x;

            return Some(o + row_delta * rows);
        }
        cell.x = x;
        cell.y = ((coord + good_measure).y / SQUARE_SZ).floor() as i32;

        if cell.y < 0 || cell.y >= map.dim().0 as i32 { return None; }

        if is_wall_f(map, coord + good_measure) {
            return Some(coord);
        }
        coord += row_delta;
    }
}

fn cast_horizontal_ray(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    assert!(dir.x.abs() > 0.7); // We can divide by dir.x

    let origin_cell: Vector2<i32> = (o / SQUARE_SZ).cast().unwrap();

    // Check vertical intersections

    // Find first intersection point
    // BUG! First intersection point could be horizontal!
    let mut dx = o.x - (origin_cell.x as f64) * SQUARE_SZ;
    if dir.x > 0. {
        dx = SQUARE_SZ - dx;
    }
    let dist = dx / dir.x.abs();

    let first_vertical_intersection_coord = o + dir * dist;

    // Scan map columns for intersections
    let col_delta = dir * (SQUARE_SZ / dir.x.abs());

    let good_measure = Vector2::new(
        if dir.x > 0. {
            SQUARE_SZ / 2.
        } else {
            -SQUARE_SZ / 2.
        },
        0.
    );

    let mut coord = first_vertical_intersection_coord;
    let mut cell: Vector2<i32> = ((coord + good_measure) / SQUARE_SZ).cast().unwrap();
    loop {
        let y = (coord.y / SQUARE_SZ).floor() as i32;
        if y < 0 || y >= map.dim().0 as i32 { return None; }

        if (y != cell.y) && is_wall(map, &Vector2::new(cell.x, y)) {
            let intersection_y =
                (if col_delta.y > 0. { cell.y+1 } else { cell.y }) as f64
                * SQUARE_SZ;

            // It is apparent that col_delta.y is not near zero, since we
            // have come to a different column on the map

            let cols = (intersection_y - o.y) / col_delta.y;

            return Some(o + col_delta * cols);
        }
        cell.x = ((coord + good_measure).x / SQUARE_SZ).floor() as i32;
        cell.y = y;

        if cell.x < 0 || cell.x >= map.dim().1 as i32 { return None; }

        if is_wall_f(map, coord + good_measure) {
            return Some(coord);
        }
        coord += col_delta;
    }
}

fn cast_ray(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    if dir.x.abs() > dir.y.abs() {
        cast_horizontal_ray(map, o, dir)
    } else {
        cast_vertical_ray(map, o, dir)
    }
}

pub fn render(map: ArrayView2<u8>, screen: &mut ArrayViewMut2<Pixel>, pos: Vector2<f64>, dir: Vector2<f64>) {
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

        let projected_height = match cast_ray(map, pos, ray_dir.normalize()) {
            Some(intersection_point) => {
                let z = (intersection_point - &pos).dot(dir);
                let w = 1./z;

                w * WALL_HEIGHT * distance_to_projection_plane
            },
            None => 0.
        };

        let mid = screen.height() as f64 / 2.;
        let ceil = max((mid - projected_height/2.).floor() as isize, 0) as usize;
        let floor = min((mid + projected_height/2.).floor() as usize, screen.height());

        for y in 0..ceil {
            *screen.px(x, y) = CEIL;
        }

        for y in ceil..floor {
            *screen.px(x, y) = WALL;
        }

        for y in floor..screen.height() {
            *screen.px(x, y) = FLOOR;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::Array2;

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

        let pos = Vector2::new(map.dim().1 as f64 / 2. * SQUARE_SZ, map.dim().0 as f64 / 2. * SQUARE_SZ);
        for ang in 0..10 {
            let rad = ang as f64 * TAU / 10.;
            let dir = Vector2::new(rad.cos(), rad.sin());
            render(map, &mut screen.view_mut(), pos, dir);
        }
    }
}
