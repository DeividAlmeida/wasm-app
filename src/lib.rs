extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::f64::consts::PI;

#[wasm_bindgen]
pub fn wasm_func(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0;
    let d_lat = deg2rad(lat2 - lat1);
    let d_lon = deg2rad(lon2 - lon1);
    let a = f64::sin(d_lat / 2.0) * f64::sin(d_lat / 2.0) + f64::cos(deg2rad(lat1)) * f64::cos(deg2rad(lat2)) * f64::sin(d_lon / 2.0) * f64::sin(d_lon / 2.0);
    let c = 2.0 * f64::atan2(f64::sqrt(a), f64::sqrt(1.0 - a));
    let distance = R * c;

    return distance * 1000.0;
}

fn deg2rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}
