#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

fn cross3(p0: Point, p1: Point, p2: Point) -> isize {
    (p2.x - p0.x) * (p1.y - p0.y) - (p1.x - p0.x) * (p2.y - p0.y)
}

fn graham_scan(points: &[Point]) -> Vec<usize> {
    let mut indices = (0..points.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| (points[i].x, points[i].y));
    indices.dedup_by_key(|i| points[*i]);

    if indices.len() <= 2 {
        return indices;
    }

    let mut upper = vec![];
    for &k in &indices {
        while upper.len() >= 2
            && cross3(
                points[upper[upper.len() - 2]],
                points[upper[upper.len() - 1]],
                points[k],
            ) <= 0
        {
            upper.pop();
        }
        upper.push(k);
    }

    let mut lower = vec![];
    for &k in &indices {
        while lower.len() >= 2
            && cross3(
                points[lower[lower.len() - 2]],
                points[lower[lower.len() - 1]],
                points[k],
            ) >= 0
        {
            lower.pop();
        }
        lower.push(k);
    }

    lower.reverse();
    upper.pop();
    lower.pop();
    for &k in &lower {
        upper.push(k);
    }
    upper
}

#[wasm_bindgen(js_name = convexHull)]
pub fn convex_hull(data: &JsValue) -> Result<JsValue, JsValue> {
    let points = data.into_serde::<Vec<Point>>().map_err(|e| e.to_string())?;
    let indices = graham_scan(&points[..]);
    JsValue::from_serde(&indices.into_iter().map(|k| points[k]).collect::<Vec<_>>())
        .map_err(|e| e.to_string().into())
}
