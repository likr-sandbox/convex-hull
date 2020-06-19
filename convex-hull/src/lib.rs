#[macro_use]
extern crate serde_derive;

use ordered_float::OrderedFloat;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn anticlockwise(p0: Point, p1: Point, p2: Point) -> bool {
    let c = (p2.x - p0.x) * (p1.y - p0.y) - (p1.x - p0.x) * (p2.y - p0.y);
    c < 0.
}

fn graham_scan(points: &[Point]) -> Result<Vec<Point>, String> {
    let n = points.len();
    if n < 3 {
        return Ok(vec![]);
    }
    let p0 = *points
        .iter()
        .min_by_key(|&p| (OrderedFloat::from(p.y), OrderedFloat::from(p.x)))
        .unwrap();
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| OrderedFloat::from((points[i].y - p0.y).atan2(points[i].x - p0.x)));
    let mut stack = vec![];
    for &i in &indices[0..3] {
        stack.push(points[i]);
    }
    for &i in &indices[3..n] {
        loop {
            let m = stack.len();
            if m < 2 || anticlockwise(stack[m - 2], stack[m - 1], points[i]) {
                break;
            }
            stack.pop();
        }
        stack.push(points[i]);
    }
    Ok(stack)
}

#[wasm_bindgen(js_name = convexHull)]
pub fn convex_hull(data: &JsValue) -> Result<JsValue, JsValue> {
    let points = data.into_serde::<Vec<Point>>().map_err(|e| e.to_string())?;
    let ch = graham_scan(&points[..])?;
    JsValue::from_serde(&ch).map_err(|e| e.to_string().into())
}
