use wavetable::wavetable::{WaveTable, WaveTableCollection};

use std::f64::consts::PI;

fn main() {
    let mut collection = WaveTableCollection::new("Fractal-FM".into());
    // Wavetables
    collection.push(Box::new(OneToTenMultiplier));
    collection.push(Box::new(OneMultiplier));
    collection.push(Box::new(RichNestedFM3));
    collection.push(Box::new(OneToTenMultiplierNoDelay));
    collection.push(Box::new(ThreeOverTwo));
    collection.push(Box::new(ThreeOverTwoNoDelay));
    collection.push(Box::new(NineOverEight));
    collection.push(Box::new(TwentyThreeOverThirteen));
    collection.push(Box::new(TwoMultiplier));
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

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "rich-nested-fm-3".into()
    }
}

struct OneToTenMultiplier;
impl WaveTable for OneToTenMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = cycle * 10.;
        let t = 2.2;
        let a = 0.47;
        let x = phase * 2. * PI;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "one-to-ten-multiplier".into()
    }
}

struct OneToTenMultiplierNoDelay;
impl WaveTable for OneToTenMultiplierNoDelay {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = cycle * 10.;
        let t = 0.0;
        let a = 0.47;
        let x = phase * 2. * PI;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "one-to-ten-multiplier-no-delay".into()
    }
}

struct OneMultiplier;
impl WaveTable for OneMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 1.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "one-multiplier".into()
    }
}

struct ThreeOverTwo;
impl WaveTable for ThreeOverTwo {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = 0.75;
        let m = 1.5;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "three-over-two".into()
    }
}

struct ThreeOverTwoNoDelay;
impl WaveTable for ThreeOverTwoNoDelay {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = 0.;
        let m = 1.5;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "three-over-two-no-delay".into()
    }
}

struct NineOverEight;
impl WaveTable for NineOverEight {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 4. * cycle;
        let t = PI / 8.;
        let m = 9. / 8.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "nine-over-eight".into()
    }
}

struct TwoMultiplier;
impl WaveTable for TwoMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 5. * cycle;
        let t = 11.;
        let m = 2.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "two-multiplier".into()
    }
}

struct TwentyThreeOverThirteen;
impl WaveTable for TwentyThreeOverThirteen {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = PI / 8.;
        let m = 23. / 13.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "twenty-three-over-thirteen".into()
    }
}

fn fractal_fm(x: f64, a: f64, t: f64, m: f64) -> f64 {
    let iterations = 4;
    let mut y = m.powi(4) * (x - 4. * t);

    for i in 0..iterations {
        let idx = iterations - 1 - i;
        y = m.powi(idx) * (x - idx as f64 * t) + a * f64::sin(y);
    }

    f64::sin(y)
}
