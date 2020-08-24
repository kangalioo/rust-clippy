#![warn(clippy::float_equality_without_abs)]

fn main() {
    let a = 0.05;
    let b = 0.0500001;

    let _ = (a - b) < f32::EPSILON;
    let _ = a - b < f32::EPSILON;
    let _ = a - b.abs() < f32::EPSILON;
    let _ = (a as f64 - b as f64) < f64::EPSILON;

    let _ = (a - b).abs() < f32::EPSILON;
    let _ = (a as f64 - b as f64).abs() < f64::EPSILON;
}
