use std::error::Error;
use std::path::Path;
use derive_more::Display;
use pitch_shift::PitchShifter;
use rodio::{OutputStream, Sink, Source};
use rodio::buffer::SamplesBuffer;
use rubato::Resampler;
use rustfft::num_traits::real::Real;
use rustfft::num_traits::Zero;
use stringcase::snake_case;


#[derive(Display)]
pub enum Instrument {
    SalamanderGrandPiano,
}

impl Instrument {
    pub fn sample_folder_path(&self) -> Option<String> {
        match self {
            Instrument::SalamanderGrandPiano => {
                let folder_name = snake_case(&self.to_string());
                let folder_path = format!("./resources/samples/{}", folder_name);
                Some(folder_path)
            }
        }
    }
}

// pub fn wave_generate(pitch: Pitch, instrument: Instrument) -> Vec<f32> {
    // let sample_folder_path = instrument.sample_folder_path().unwrap();
    // let pitch_file_path = format!("{}/{}.flac", sample_folder_path, pitch.to_string());
    //
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let file = BufReader::new(File::open(pitch_file_path).unwrap());
    // let source = rodio::Decoder::new(file).unwrap();
    //
    // let resampler = Ftf
    // source.
    // let wave = SamplesBuffer::new(source.channels(), source.sample_rate(), [
    //     2.0; 44100*2
    // ]);
    // let source = wave.convert_samples::<f32>();
    // let sink = Sink::try_new(&stream_handle).unwrap();
    // sink.append(source);
    // sink.sleep_until_end();

//     vec![]
// }

/// Generate pitch samples for the given instrument and pitch.
///
/// This function reads the sample file for the given instrument and pitch, and generates samples for the pitch.
/// If the instrument is not found, it will panic.
/// If the pitch is not found, it will search the nearest pitch and do pitch shifting .
///
/// # Arguments
/// * `instrument` - The instrument to generate samples for
/// * `pitch` - The pitch to generate samples for
///
/// # Returns
/// * A vector of samples for the given instrument and pitch
// pub fn generate_pitch_samples(instrument: Instrument, pitch: Pitch) -> Result<Vec<f32>, Box<dyn Error>> {
//     let sample_folder_path_str = instrument.sample_folder_path().ok_or("Sample folder not found")?;
//     let sample_folder_path = Path::new(&sample_folder_path_str);
//     if !sample_folder_path.exists() {
//         return Err("Sample folder not found".into());
//     }
//     let pitch_file_name = format!("{}.flac", pitch.to_string());
//     let pitch_file_path = sample_folder_path.join(pitch_file_name);
// 
//     let shift_pitch: i16 = 0;
//     if !pitch_file_path.exists() {
//     }
// 
//     // read the sample
//     // read the sample
//     let mut reader = claxon::FlacReader::open(pitch_file_path).unwrap();
//     let meta_info = reader.streaminfo();
//     let bit = 2f32.powf(meta_info.bits_per_sample as f32) / 2.0 - 1.0; // calculate the bit
//     let samples: Vec<f32> = reader.samples().map(|s| s.unwrap() as f32 / bit).collect(); // read the samples and normalize
//     let samples: Vec<f32> = samples.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, s)| *s).collect(); // get only one channel
// 
//     // pitch shift
//     let mut out_samples = samples.clone();
//     let mut ps = PitchShifter::new(50, meta_info.sample_rate as usize);
//     ps.shift_pitch(
//         5,
//         3.0, // 3 semitones
//         &samples,
//         &mut out_samples,
//     );
//     let samples = out_samples;
// 
//     // Play
//     // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     // let source = SamplesBuffer::new(1, meta_info.sample_rate, samples).convert_samples::<f32>();
//     // let sink = Sink::try_new(&stream_handle).unwrap();
//     // sink.append(source);
//     // sink.sleep_until_end();
// 
//     Ok(samples)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_generate() {
    }
}