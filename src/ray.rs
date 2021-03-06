use cgmath::{vec2, Vector2};
use ndarray::{ArrayView2, Axis};

use consts::*;
use textures::*;

fn is_wall(map: ArrayView2<u8>, cell: &Vector2<i32>) -> Option<Option<u8>> {
    if cell.x < 0 || cell.x as usize >= map.dim().1 { return None; }
    if cell.y < 0 || cell.y as usize >= map.dim().0 { return None; }

    let cell = cell.cast().unwrap();
    Some(match map[[cell.y, cell.x]] {
        0 => None,
        x => Some(x-1)
    })
}

fn cast_ray_south_east_east(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<(Vector2<f64>, f64, TextureSpec)> {
    assert!(dir.y >= 0.); // We're going south
    assert!(dir.x >= 0.); // We're going east
    assert!(dir.x >= dir.y); // Major direction is east
    assert!(dir.x >= 0.7); // We can divide by x

    let first_vertical_intersection_coord = {
        let dx = (o.x / SQUARE_SZ + 1.).floor() * SQUARE_SZ - o.x;
        let dist = dx / dir.x;

        o + dir * dist
    };

    let start_x = (first_vertical_intersection_coord.x / SQUARE_SZ).ceil() as i32;
    let step_y = dir.y * (SQUARE_SZ / dir.x);
    let mut y = first_vertical_intersection_coord.y;

    for x in start_x..(map.dim().1 as i32) {
        if let Some(tx) = is_wall(map, &vec2(x - 1, (y / SQUARE_SZ) as i32))? {
            // Intersection with horizontal line

            let intersection_y = (y / SQUARE_SZ).floor() * SQUARE_SZ;

            // It is apparent that dir.y is not near zero, since we
            // have come to a different column on the map
            let dist = (intersection_y - o.y) / dir.y;

            let p = o + dir * dist;
            let u = (1. + p.x / SQUARE_SZ).floor() * SQUARE_SZ - p.x;

            if dist > 0. {
                return Some((p, u, TextureSpec { tx, side: Side::NorthSouth }));
            }
        }

        if let Some(tx) = is_wall(map, &vec2(x, (y / SQUARE_SZ) as i32))? {
            // Intersection with vertical line
            let p = vec2(x as f64 * SQUARE_SZ, y);
            let u = y - (y / SQUARE_SZ).floor() * SQUARE_SZ;
            return Some((p, u, TextureSpec { tx, side: Side::WestEast }));
        }

        y += step_y;
    }

    return None;
}

fn cast_ray_south_east(map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<(Vector2<f64>, f64, TextureSpec)> {
    assert!(dir.x >= 0.);
    assert!(dir.y >= 0.);

    if dir.y > dir.x {
        return cast_ray_south_east_east(
            map.t(),
            vec2(o.y, o.x),
            vec2(dir.y, dir.x)
        ).map(|(p, u, tx)| (vec2(p.y, p.x), SQUARE_SZ-u, tx.flipped()));
    } else {
        cast_ray_south_east_east(map, o, dir)
    }
}

fn cast_ray_east(mut map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<(Vector2<f64>, f64, TextureSpec)> {
    assert!(dir.x >= 0.);

    if dir.y < 0. {
        let map_height = map.dim().0 as f64 * SQUARE_SZ;
        map.invert_axis(Axis(0));
        cast_ray_south_east(
            map,
            vec2(o.x, map_height - o.y),
            vec2(dir.x, -dir.y)
        ).map(|(p, u, tx)| (vec2(p.x, map_height - p.y), SQUARE_SZ-u, tx))
    } else {
        cast_ray_south_east(map, o, dir)
    }
}

pub fn cast_ray(mut map: ArrayView2<u8>, o: Vector2<f64>, dir: Vector2<f64>) -> Option<(Vector2<f64>, f64, TextureSpec)> {
    if dir.x < 0. {
        let map_width = map.dim().1 as f64 * SQUARE_SZ;
        map.invert_axis(Axis(1));
        cast_ray_east(
            map,
            vec2(map_width - o.x, o.y),
            vec2(-dir.x, dir.y)
        ).map(|(p, u, tx)| (vec2(map_width - p.x, p.y), SQUARE_SZ-u, tx))
    } else {
        cast_ray_east(map, o, dir)
    }
}
