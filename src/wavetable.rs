use std::fs::{self, File};
use std::io::Write;

pub trait WaveTable {
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

pub struct WaveTableCollection {
    name: String,
    wave_tables: Vec<Box<dyn WaveTable>>,
}

impl WaveTableCollection {
    pub fn new(name: String) -> Self {
        let wave_tables = vec![];
        Self { name, wave_tables }
    }

    pub fn push(&mut self, wave_table: Box<dyn WaveTable>) {
        self.wave_tables.push(wave_table);
    }

    pub fn generate_i16_wt(&self, wave_size: u32, wave_count: u16) {
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

    pub fn generate_f32_wt(&self, wave_size: u32, wave_count: u16) {
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

    pub fn generate_serum(&self, wave_size: u32, wave_count: u32) {
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
