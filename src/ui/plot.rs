use textplots::{Chart, Plot, Shape};

use crate::data::HashrateData;

const EXA: u128 = 1_000_000_000_000_000_000;

pub fn hashrate(hashrate_data: &HashrateData) {
    let mut points: Vec<(f32, f32)> = vec![];
    for i in 0..hashrate_data.hashrates.len() {
        points.push((
            i as f32,
            (hashrate_data.hashrates[i].avg_hashrate / EXA) as f32,
        ))
    }

    Chart::new(160, 50, 0.0, hashrate_data.hashrates.len() as f32)
        .lineplot(&Shape::Lines(&points))
        .nice();
}
