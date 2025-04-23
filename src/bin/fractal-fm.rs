use wavetable::wavetable::{WaveTable, WaveTableCollection};

use std::f64::consts::PI;

fn main() {
    let mut collection = WaveTableCollection::new("Fractal-FM".into());
    // Wavetables
    collection.push(Box::new(OneToTenMultiplier));
    collection.push(Box::new(OneMultiplier));
    collection.push(Box::new(TwoMultiplier));
    collection.push(Box::new(ThreeMultiplier));
    collection.push(Box::new(FourMultiplier));
    collection.push(Box::new(FiveMultiplier));
    collection.push(Box::new(SixMultiplier));
    collection.push(Box::new(SevenMultiplier));
    collection.push(Box::new(EightMultiplier));
    collection.push(Box::new(NineMultiplier));
    collection.push(Box::new(TenMultiplier));
    collection.push(Box::new(ModulateDelay));
    collection.push(Box::new(OneToTenMultiplierNoDelay));
    collection.push(Box::new(ThreeOverTwo));
    collection.push(Box::new(ThreeOverTwoNoDelay));
    collection.push(Box::new(NineOverEight));
    collection.push(Box::new(TwentyThreeOverThirteen));
    collection.push(Box::new(SevenOverThree));
    collection.push(Box::new(FiveOverFour));
    // Generate
    collection.generate_f32_wt(2048, 256);
    collection.generate_f32_wt(4096, 512);
    collection.generate_serum(2048, 256);
    collection.generate_serum(2048, 64);
}

struct ModulateDelay;
impl WaveTable for ModulateDelay {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let m = 2.;
        let a = 0.75;
        let t = cycle * 2. * PI;
        let x = phase * 2. * PI;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "modulate-delay".into()
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
struct ThreeMultiplier;
impl WaveTable for ThreeMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 3.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "three-multiplier".into()
    }
}
struct FourMultiplier;
impl WaveTable for FourMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 4.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "four-multiplier".into()
    }
}
struct FiveMultiplier;
impl WaveTable for FiveMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 5.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "five-multiplier".into()
    }
}
struct SixMultiplier;
impl WaveTable for SixMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 6.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "six-multiplier".into()
    }
}
struct SevenMultiplier;
impl WaveTable for SevenMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 7.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "seven-multiplier".into()
    }
}
struct EightMultiplier;
impl WaveTable for EightMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 8.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "eight-multiplier".into()
    }
}
struct NineMultiplier;
impl WaveTable for NineMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 9.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "nine-multiplier".into()
    }
}
struct TenMultiplier;
impl WaveTable for TenMultiplier {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = cycle * 3.;
        let t = 2.2;
        let m = 10.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "ten-multiplier".into()
    }
}

struct ThreeOverTwo;
impl WaveTable for ThreeOverTwo {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 4. * PI;
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
struct SevenOverThree;
impl WaveTable for SevenOverThree {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = PI / 8.;
        let m = 7. / 3.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "seven-over-three".into()
    }
}
struct FiveOverFour;
impl WaveTable for FiveOverFour {
    fn sample(&self, cycle: f64, phase: f64) -> f64 {
        let x = phase * 2. * PI;
        let a = 3. * cycle;
        let t = 0.;
        let m = 5. / 4.;

        fractal_fm(x, a, t, m)
    }

    fn name(&self) -> String {
        "five-over-four".into()
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
