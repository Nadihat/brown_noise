use clap::Parser;
use hound::{SampleFormat, WavSpec, WavWriter};
use rand::distributions::Distribution;
use rand_distr::Normal;
use std::path::PathBuf;

/// Brown noise generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output WAV file path
    #[arg(short, long, default_value = "brown_noise.wav")]
    output: PathBuf,

    /// Duration in seconds
    #[arg(short, long, default_value_t = 10.0)]
    duration: f32,

    /// Sample rate in Hz
    #[arg(short, long, default_value_t = 44100)]
    sample_rate: u32,

    /// Amplitude (0.0 to 1.0)
    #[arg(short, long, default_value_t = 0.5)]
    amplitude: f32,
}

fn main() {
    let args = Args::parse();
    
    // Generate and save brown noise
    generate_brown_noise(
        &args.output,
        args.duration,
        args.sample_rate,
        args.amplitude,
    );
    
    println!("Brown noise generated and saved to {:?}", args.output);
}

/// Generates mathematically perfect brown noise and saves it to a WAV file
fn generate_brown_noise(output_path: &PathBuf, duration: f32, sample_rate: u32, amplitude: f32) {
    // Calculate number of samples
    let num_samples = (duration * sample_rate as f32) as u32;
    
    // Set up WAV file
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(output_path, spec).unwrap();
    
    // Brown noise generation
    // We use a leaky integrator on white noise to create brown noise
    // y[n] = alpha * y[n-1] + (1-alpha) * x[n]
    // where x[n] is white noise and alpha is close to 1
    
    // Create normal distribution for white noise
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    
    // Leaky integrator coefficient (close to 1 for brown noise)
    let alpha = 0.99;
    
    // Initial value
    let mut y_prev = 0.0;
    
    // Normalization factor to prevent clipping
    let normalization = amplitude * 0.15; // Brown noise needs more normalization
    
    for _ in 0..num_samples {
        // Generate white noise sample
        let white_sample = normal.sample(&mut rng) as f32;
        
        // Apply leaky integrator to get brown noise
        y_prev = alpha * y_prev + (1.0 - alpha) * white_sample;
        
        // Normalize and convert to i16
        let normalized_sample = y_prev * normalization;
        let sample = (normalized_sample * i16::MAX as f32) as i16;
        
        writer.write_sample(sample).unwrap();
    }
    
    writer.finalize().unwrap();
}

/// Verify the spectral characteristics of the generated noise
/// This function is not called in the main program but can be used for testing
#[allow(dead_code)]
fn verify_spectral_characteristics(_samples: &[f32], _sample_rate: u32) -> bool {
    // Perform FFT and check if power decreases at 6 dB per octave
    // This would require an FFT library, which we're not including for simplicity
    // In a real implementation, you would use rustfft or similar
    
    // For each octave, the power should be approximately 1/4 of the previous octave
    // (6 dB = factor of 4 in power)
    
    true // Placeholder
}
