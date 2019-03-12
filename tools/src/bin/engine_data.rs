use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::{Mul, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// Engine output wrt the throttle input, t, and time decay, d.
fn engine_output(t: f64, d: f64) -> f64 {
    assert!(0.0 <= t && t <= 1.0);
    assert!(d >= 1.0);
    t.sin() + t - 0.5 * t.powi(2) - (0.67 - 0.47 / d) * t.powi(3)
}

fn main() {
    let mut writer = BufWriter::new(File::create("client/data/engine.txt").unwrap());

    // 100 days ago.
    let start_timestamp = SystemTime::now()
        .sub(Duration::from_secs(86400).mul(100))
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    for day in 1..20 {
        let measurements = (0..1000).map(|sec| {
            let throttle = sec as f64 / 1000.0;
            (sec, throttle, engine_output(throttle, day as f64))
        });
        let max_output = measurements.clone().map(|x| x.2).fold(0./0., f64::max);
        for (sec, throttle, output) in measurements {
            writer.write(
                format!(
                    "{}: {} -> {}\n",
                    start_timestamp + day * 86400 + sec,
                    throttle as f32,
                    (output / max_output) as f32,
                )
                .as_bytes(),
            ).unwrap();
        }
    }

    // TODO(ryo): Should I shuffle the records a bit to make it harder for
    // humans to parse?
}
