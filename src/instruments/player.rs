use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use pitch_shift::PitchShifter;
use rodio::{OutputStream, Sink, Source};
use rodio::buffer::SamplesBuffer;
use stringcase::snake_case;
use crate::theory::interval::Interval;
use crate::theory::pitch::Pitch;


#[derive(Debug, Clone)]
pub enum Instrument {
    SalamanderGrandPiano,
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instrument::SalamanderGrandPiano => write!(f, "SalamanderGrandPiano"),
        }
    }
}

impl Instrument {
    pub fn sample_folder_path(&self) -> PathBuf {
        match self {
            Instrument::SalamanderGrandPiano => {
                let folder_name = snake_case(&self.to_string());
                let folder_path = PathBuf::from(&format!("./resources/samples/{}", folder_name));
                folder_path
            }
        }
    }
    pub fn play(&self, pitch: Pitch) -> Result<(), Box<dyn Error>> {
        let (sample_rate, samples) = generate_pitch_samples(self.clone(), pitch)?;
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let source = SamplesBuffer::new(1, sample_rate, samples).convert_samples::<f32>();
        let sink = Sink::try_new(&stream_handle)?;
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }
}

/// Generate pitch samples for the given instrument and pitch.
///
/// This function reads the sample file for the given instrument and pitch, and generates samples for the pitch.
/// If the sample file for the given pitch is not found, it
/// 1. finds the nearest available pitch,
/// 2. shift this pitch to get the input pitch,
/// 3. generates samples for the input pitch.
///
/// # Arguments
/// * `instrument` - The instrument to generate samples for
/// * `pitch` - The pitch to generate samples for
///
/// # Returns
/// * A tuple of
/// * 1. u32: The sample rate of the generated samples
/// * 2. Vec<f32>: A vector of samples for the given instrument and pitch
pub fn generate_pitch_samples(instrument: Instrument, pitch: Pitch) -> Result<(u32, Vec<f32>), Box<dyn Error>> {
    // get the sample folder path
    let sample_folder_path = instrument.sample_folder_path();
    if !sample_folder_path.exists() {
        return Err("Sample folder not found".into());
    }
    // get the pitch file path
    let mut shift_steps: f32 = 0.0;  // the resample pitch shift
    let pitch_file_name = format!("{}", pitch);
    let mut pitch_file_path = sample_folder_path.join(pitch_file_name).with_extension("flac");
    if !pitch_file_path.exists() {
        // get all the audio files in the sample folder
        let audio_file_names: Vec<String> = fs::read_dir(&sample_folder_path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("flac"))
            .filter_map(|path| path.file_stem().map(|file_name| file_name.to_string_lossy().to_string()))
            .collect();
        // find the nearest pitch
        let pitches: Vec<Pitch> = audio_file_names.iter().map(|file_name| Pitch::try_from(file_name.to_string()).unwrap()).collect();
        let new_pitch = pitch.get_the_nearest_pitch(pitches);
        // set the pitch file path
        pitch_file_path = sample_folder_path.join(format!("{}", new_pitch)).with_extension("flac");
        // set the shift steps
        shift_steps = Interval::new(pitch.clone(), new_pitch.clone()).get_number_of_semitones(false) as f32;
        if new_pitch < pitch {
            shift_steps = -shift_steps;
        }
    }
    // read the sample
    let mut reader = claxon::FlacReader::open(pitch_file_path).unwrap();
    let meta_info = reader.streaminfo();
    if meta_info.channels > 2 {
        return Err("Only mono and stereo files are supported".into());
    }
    let bit = 2f32.powf(meta_info.bits_per_sample as f32) / 2.0 - 1.0; // calculate the bit for normalization
    let mut samples: Vec<f32> = reader.samples().map(|s| s.unwrap() as f32 / bit).collect(); // read the samples and normalize
    if meta_info.channels == 2 {
        samples = samples.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, s)| *s).collect(); // get only one channel
    }
    // pitch shift
    let mut out_samples = samples.clone();
    let mut ps = PitchShifter::new(50, meta_info.sample_rate as usize);
    ps.shift_pitch(
        5,
        -shift_steps, // 3 semitones
        &samples,
        &mut out_samples,
    );
    let samples = out_samples;

    Ok((meta_info.sample_rate, samples))
}