use cgmath::Vector2;
use ndarray::ArrayView2;

use consts::*;

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

pub fn cast_ray(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    if dir.x.abs() > dir.y.abs() {
        cast_horizontal_ray(map, o, dir)
    } else {
        cast_vertical_ray(map, o, dir)
    }
}
