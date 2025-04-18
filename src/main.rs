use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::Write;
use std::u32;

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

trait WaveTable {
    fn sample(&self, cycle: f64, phase: f64) -> f64;
    fn name(&self) -> String;
    fn generate_f32_wt(&self, wave_size: u32, wave_count: u16) -> Vec<u8> {
        let mut data = vec![];
        data.extend(create_wt_header(wave_size, wave_count, 0));

        for cycle in (0..wave_count).map(|x| x as f64 / wave_count as f64) {
            for phase in (0..wave_size).map(|x| x as f64 / wave_size as f64) {
                let sample = self.sample(cycle, phase);
                let bytes = (sample as f32).to_le_bytes();
                data.extend(bytes);
            }
        }

        data
    }

    fn generate_i16_wt(&self, wave_size: u32, wave_count: u16) -> Vec<u8> {
        let mut data = vec![];
        data.extend(create_wt_header(wave_size, wave_count, 4 | 8));

        for cycle in (0..wave_count).map(|x| x as f64 / wave_count as f64) {
            for phase in (0..wave_size).map(|x| x as f64 / wave_size as f64) {
                let sample = self.sample(cycle, phase);
                let amplitude = i16::MAX as f64;
                let bytes = ((sample * amplitude) as i16).to_le_bytes();
                data.extend(bytes);
            }
        }

        data
    }

    fn generate_serum(&self, wave_size: u32, wave_count: u32) -> Vec<u8> {
        let mut data = vec![];
        data.extend(create_serum_header(wave_size, wave_count));

        for cycle in (0..wave_count).map(|x| x as f64 / wave_count as f64) {
            for phase in (0..wave_size).map(|x| x as f64 / wave_size as f64) {
                let sample = self.sample(cycle, phase);
                let bytes = (sample as f32).to_le_bytes();
                data.extend(bytes);
            }
        }

        data
    }
}

struct WaveTableCollection {
    name: String,
    wave_tables: Vec<Box<dyn WaveTable>>,
}

impl WaveTableCollection {
    fn new(name: String) -> Self {
        let wave_tables = vec![];
        Self { name, wave_tables }
    }

    fn push(&mut self, wave_table: Box<dyn WaveTable>) {
        self.wave_tables.push(wave_table);
    }

    fn generate_i16_wt(&self, wave_size: u32, wave_count: u16) {
        for wave_table in self.wave_tables.iter() {
            let path = format!("./{}/wt/i16/{}x{}", self.name, wave_size, wave_count);
            fs::create_dir_all(&path).unwrap();
            let file_path = format!("{path}/{}.wt", wave_table.name());

            println!("Generating {}", file_path);
            let mut file = File::create(file_path).unwrap();
            let data = wave_table.generate_i16_wt(wave_size, wave_count);
            file.write_all(&data).unwrap();
        }
    }

    fn generate_f32_wt(&self, wave_size: u32, wave_count: u16) {
        for wave_table in self.wave_tables.iter() {
            let path = format!("./{}/wt/f32/{}x{}", self.name, wave_size, wave_count);
            fs::create_dir_all(&path).unwrap();
            let file_path = format!("{path}/{}.wt", wave_table.name());

            println!("Generating {}", file_path);
            let mut file = File::create(file_path).unwrap();
            let data = wave_table.generate_f32_wt(wave_size, wave_count);
            file.write_all(&data).unwrap();
        }
    }

    fn generate_serum(&self, wave_size: u32, wave_count: u32) {
        for wave_table in self.wave_tables.iter() {
            let path = format!("./{}/serum/{}x{}", self.name, wave_size, wave_count);
            fs::create_dir_all(&path).unwrap();
            let file_path = format!("{path}/{}.wav", wave_table.name());

            println!("Generating {}", file_path);
            let mut file = File::create(file_path).unwrap();
            let data = wave_table.generate_serum(wave_size, wave_count);
            file.write_all(&data).unwrap();
        }
    }
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

// flags:
// 1  -> Is sample instead of wavetable
// 2  -> Is a looped sample
// 4  -> i16 if true, f32 if false
// 8  -> Use full range
// 10 -> Include XML block after data
fn create_wt_header(wave_size: u32, wave_count: u16, flags: u16) -> [u8; 12] {
    let mut data: Vec<u8> = vec![];

    data.extend("vawt".as_bytes());
    data.extend(wave_size.to_le_bytes().iter());
    data.extend(wave_count.to_le_bytes().iter());
    data.extend(flags.to_le_bytes().iter());

    data.try_into().unwrap()
}

fn create_serum_header(wave_size: u32, wave_count: u32) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];

    // RIFF chunk
    data.extend("RIFF".as_bytes());
    // Place holder, need to know header size first
    data.extend(0u32.to_le_bytes());
    data.extend("WAVE".as_bytes());

    // fmt chunk
    data.extend("fmt ".as_bytes());
    data.extend(16u32.to_le_bytes());
    data.extend(3u16.to_le_bytes()); // 1 -> i16, 3 -> f32
    data.extend(1u16.to_le_bytes()); // Num Channels
    data.extend(48_000u32.to_le_bytes()); // Sample rate
    data.extend(192_000u32.to_le_bytes()); // Bytes / second
    data.extend(4u16.to_le_bytes()); // Bytes per block
    data.extend(32u16.to_le_bytes()); // Bit depth

    // clm chunk
    // 1 -> Linear, 2,3,4 -> Spectral
    let clm_string = format!("<!>{wave_size:4} 10000000 MATH-MAGE");
    data.extend("clm ".as_bytes());
    data.extend((clm_string.len() as u32).to_le_bytes());
    data.extend(clm_string.as_bytes());

    // data chunk
    data.extend("data".as_bytes());
    let data_size = wave_size * wave_count * 4;
    data.extend(data_size.to_le_bytes());

    // data len is the header size now that we are done writing to it
    let riff_chunk_size = data.len() as u32 + data_size - 8;

    println!("{}", riff_chunk_size);
    data.splice(4..8, riff_chunk_size.to_le_bytes());

    data
}
