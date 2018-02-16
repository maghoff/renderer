use cgmath::{vec2, Vector2};
use ndarray::{ArrayView2, Axis};

use consts::*;

fn is_wall(map: ArrayView2<u8>, cell: &Vector2<i32>) -> Option<bool> {
    if cell.x < 0 || cell.x as usize >= map.dim().1 { return None; }
    if cell.y < 0 || cell.y as usize >= map.dim().0 { return None; }

    let cell = cell.cast().unwrap();
    Some(map[[cell.y, cell.x]] == b'x')
}

fn cast_ray_south_east_east(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    assert!(dir.y >= 0.); // We're going south
    assert!(dir.x >= 0.); // We're going east
    assert!(dir.x >= dir.y); // Major direction is east
    assert!(dir.x >= 0.7); // We can divide by x

    let origin_cell: Vector2<i32> = (o / SQUARE_SZ).cast().unwrap();
    if origin_cell.x < 0 || origin_cell.x as usize >= map.dim().0 { return None; }
    if origin_cell.y < 0 || origin_cell.y as usize >= map.dim().1 { return None; }

    // Check vertical intersections

    // Find first intersection point
    // BUG! First intersection point could be horizontal!
    let dx = ((origin_cell.x + 1) as f64) * SQUARE_SZ - o.x;
    let dist = dx / dir.x;

    let first_vertical_intersection_coord = o + dir * dist;

    // Scan map columns for intersections
    let col_delta = dir * (SQUARE_SZ / dir.x);

    let good_measure = Vector2::new(SQUARE_SZ / 2., 0.);

    let coord = first_vertical_intersection_coord;
    let cell: Vector2<i32> = ((coord + good_measure) / SQUARE_SZ).cast().unwrap();

    let mut y = coord.y;
    for x in cell.x..(map.dim().1 as i32) {
        if is_wall(map, &vec2(x, (y / SQUARE_SZ) as i32))? {
            // Intersection with vertical line
            return Some(vec2(x as f64 * SQUARE_SZ, y));
        }
        y += col_delta.y;

        // Could check if y-cell has changed, but this gives the same result:
        if is_wall(map, &vec2(x, (y / SQUARE_SZ) as i32))? {
            // Intersection with horizontal line

            let intersection_y = (y / SQUARE_SZ).floor() * SQUARE_SZ;

            // It is apparent that col_delta.y is not near zero, since we
            // have come to a different column on the map
            let cols = (intersection_y - o.y) / col_delta.y;

            return Some(o + col_delta * cols);
        }
    }

    return None;
}

fn cast_ray_south_east(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    assert!(dir.x >= 0.);
    assert!(dir.y >= 0.);

    if dir.y > dir.x {
        return cast_ray_south_east_east(
            map.t(),
            vec2(o.y, o.x),
            vec2(dir.y, dir.x)
        ).map(|v| vec2(v.y, v.x));
    } else {
        cast_ray_south_east_east(map, o, dir)
    }
}

fn cast_ray_east(mut map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    assert!(dir.x >= 0.);

    if dir.y < 0. {
        let map_height = map.dim().0 as f64 * SQUARE_SZ;
        map.invert_axis(Axis(0));
        cast_ray_south_east(
            map,
            vec2(o.x, map_height - o.y),
            vec2(dir.x, -dir.y)
        ).map(|v| vec2(v.x, map_height - v.y))
    } else {
        cast_ray_south_east(map, o, dir)
    }
}

pub fn cast_ray(mut map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<Vector2<f64>> {
    if dir.x < 0. {
        let map_width = map.dim().1 as f64 * SQUARE_SZ;
        map.invert_axis(Axis(1));
        cast_ray_east(
            map,
            vec2(map_width - o.x, o.y),
            vec2(-dir.x, dir.y)
        ).map(|v| vec2(map_width - v.x, v.y))
    } else {
        cast_ray_east(map, o, dir)
    }
}
