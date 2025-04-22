mod wavetable;

use crate::wavetable::{WaveTable, WaveTableCollection};

use std::f64::consts::PI;

fn main() {
    let mut collection = WaveTableCollection::new("Rich-Nested-FM".into());
    // Wavetables
    collection.push(Box::new(RichNestedFM1));
    collection.push(Box::new(RichNestedFM2));
    collection.push(Box::new(RichNestedFM3));
    collection.push(Box::new(RichNestedNoDelay));
    collection.push(Box::new(PerfectFifth));
    collection.push(Box::new(PerfectFifthNoDelay));
    collection.push(Box::new(MinorSeventh));
    collection.push(Box::new(Ninth));
    collection.push(Box::new(Octave));
    // Generate
    collection.generate_f32_wt(2048, 256);
    collection.generate_f32_wt(4096, 512);
    collection.generate_serum(2048, 256);
    collection.generate_serum(2048, 64);
}

struct RichNestedFM3;
impl WaveTable for RichNestedFM3 {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = 2.;
        let a = 0.75;
        let t = cycle * 2. * PI;
        let x = phase * 2. * PI;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "rich-nested-fm-3".into()
    }
}

struct RichNestedFM1;
impl WaveTable for RichNestedFM1 {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = cycle * 10.;
        let t = 2.2;
        let a = 0.47;
        let x = phase * 2. * PI;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "rich-nested-fm-1".into()
    }
}

struct RichNestedNoDelay;
impl WaveTable for RichNestedNoDelay {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = cycle * 10.;
        let t = 0.0;
        let a = 0.47;
        let x = phase * 2. * PI;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "rich-nested-no-delay".into()
    }
}

struct RichNestedFM2;
impl WaveTable for RichNestedFM2 {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 1.;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "rich-nested-fm-2".into()
    }
}

struct PerfectFifth;
impl WaveTable for PerfectFifth {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = 0.75;
        let m = 1.5;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "perfect-fifth".into()
    }
}

struct PerfectFifthNoDelay;
impl WaveTable for PerfectFifthNoDelay {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = 0.;
        let m = 1.5;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "perfect-fifth-no-delay".into()
    }
}

struct MinorSeventh;
impl WaveTable for MinorSeventh {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 4. * cycle;
        let t = PI / 8.;
        let m = 9. / 8.;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "minor-seventh".into()
    }
}

struct Octave;
impl WaveTable for Octave {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 5. * cycle;
        let t = 11.;
        let m = 2.;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "octave".into()
    }
}

struct Ninth;
impl WaveTable for Ninth {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = PI / 8.;
        let m = 23. / 13.;

        fm_feedback_series(x, a, t, m)
    }

    fn name(&self) -> String {
        "ninth".into()
    }
}

fn fm_feedback_series(x: f64, a: f64, t: f64, m: f64) -> f64 {
    let iterations = 4;
    let mut y = m.powi(4) * (x - 4. * t);

    for i in 0..iterations {
        let idx = iterations - 1 - i;
        y = m.powi(idx) * (x - idx as f64 * t) + a * f64::sin(y);
    }

    f64::sin(y)
}
