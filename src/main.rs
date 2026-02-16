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

    /// Multiplier (1.0 to 35.0) - controls brightness/cutoff
    #[arg(short, long, default_value_t = 1.0)]
    multiplier: f32,
}

fn main() {
    let args = Args::parse();
    
    // Generate and save brown noise
    generate_brown_noise(
        &args.output,
        args.duration,
        args.sample_rate,
        args.amplitude,
        args.multiplier,
    );
    
    println!("Brown noise generated and saved to {:?}", args.output);
}

/// Generates brown noise with LPF and saves it to a WAV file
fn generate_brown_noise(output_path: &PathBuf, duration: f32, sample_rate: u32, amplitude: f32, multiplier: f32) {
    // Calculate number of samples
    let num_samples = (duration * sample_rate as f32) as u32;

    // Calculate cutoff frequency based on script.js logic
    // const min=100, max=20000;
    // let base = noiseType==='brown'? 80 : 150;
    // const minLog=Math.log(base);
    // const maxLog=Math.log(max*(noiseType==='brown'?0.85:1));
    // const scale=(maxLog-minLog)/(35-1);
    // return Math.exp(minLog+scale*(mult-1));
    let base = 80.0f32;
    let max = 20000.0f32;
    let min_log = base.ln();
    let max_log = (max * 0.85).ln();
    let scale = (max_log - min_log) / (35.0 - 1.0);
    let cutoff_hz = (min_log + scale * (multiplier - 1.0)).exp();

    // Setup Low Pass Filter (Simple RC filter approximation for Biquad Lowpass Q=1)
    // For a more accurate Biquad implementation we would need 5 coefficients,
    // but a simple 1-pole LPF is often sufficient for noise shaping or we can implement a basic Biquad.
    // Let's implement a basic Biquad Lowpass filter to match the web audio API closer.
    // Reference: https://www.w3.org/TR/audio-eq-cookbook/
    let w0 = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate as f32;
    let cos_w0 = w0.cos();
    let alpha_bq = w0.sin() / (2.0 * 1.0); // Q = 1.0

    let b0 = (1.0 - cos_w0) / 2.0;
    let b1 = 1.0 - cos_w0;
    let b2 = (1.0 - cos_w0) / 2.0;
    let a0 = 1.0 + alpha_bq;
    let a1 = -2.0 * cos_w0;
    let a2 = 1.0 - alpha_bq;

    // Normalize coefficients
    let b0 = b0 / a0;
    let b1 = b1 / a0;
    let b2 = b2 / a0;
    let a1 = a1 / a0;
    let a2 = a2 / a0;

    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;
    
    // Set up WAV file
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(output_path, spec).unwrap();
    
    // Brown noise generation based on script.js
    // last = (last + 0.02 * white) / 1.02
    // output = last * 3.5
    
    // Create uniform distribution for white noise (-1.0 to 1.0)
    // script.js uses Math.random() * 2 - 1
    let uniform = rand::distributions::Uniform::new_inclusive(-1.0, 1.0);
    let mut rng = rand::thread_rng();
    
    // Initial value
    let mut last = 0.0;
    
    for _ in 0..num_samples {
        // Generate white noise sample
        let white: f32 = uniform.sample(&mut rng);
        
        // Apply leaky integrator algorithm from script.js
        last = (last + 0.02 * white) / 1.02;
        
        // Raw brown noise sample
        let raw_brown = last * 3.5;

        // Apply Biquad Low Pass Filter
        let filtered = b0 * raw_brown + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;

        // Shift delay lines
        x2 = x1;
        x1 = raw_brown;
        y2 = y1;
        y1 = filtered;

        // Apply user amplitude
        let output = filtered * amplitude;
        
        // Clamp and convert to i16
        let sample = (output.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        
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
