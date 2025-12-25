# Brown Noise Generator

A mathematically perfect brown noise generator written in Rust. This command-line tool generates high-quality brown noise and saves it as a WAV file.

## What is Brown Noise?

Brown noise (also known as Brownian noise or red noise) is a type of noise with a power spectral density that decreases at 6 dB per octave with increasing frequency. Unlike white noise, which has equal power across all frequencies, brown noise has more energy in the lower frequencies, giving it a deeper, richer sound.

Brown noise is named after Robert Brown, who discovered Brownian motion, as the noise's mathematical properties are similar to the random movement of particles in a fluid.

## Features

- Generates mathematically perfect brown noise
- Adjustable duration, sample rate, and amplitude
- Outputs to standard WAV format
- Simple command-line interface

## Installation

### Prerequisites

- Rust and Cargo (install from [rust-lang.org](https://www.rust-lang.org/tools/install))

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/brown_noise_generator.git
cd brown_noise_generator

# Build the project
cargo build --release
```

The compiled binary will be available at `./target/release/brown_noise_generator`.

## Usage

```bash
# Basic usage with default settings (10 seconds, 44.1kHz, amplitude 0.5)
./target/release/brown_noise_generator

# Generate a 30-second brown noise file
./target/release/brown_noise_generator -d 30

# Specify output file, duration, sample rate, and amplitude
./target/release/brown_noise_generator -o my_noise.wav -d 60 -s 48000 -a 0.7
```

### Command-Line Options

- `-o, --output <FILE>`: Output WAV file path (default: "brown_noise.wav")
- `-d, --duration <SECONDS>`: Duration in seconds (default: 10.0)
- `-s, --sample-rate <RATE>`: Sample rate in Hz (default: 44100)
- `-a, --amplitude <LEVEL>`: Amplitude between 0.0 and 1.0 (default: 0.5)

## How It Works

This generator creates brown noise using a leaky integrator applied to white noise:

1. Generate white noise samples from a normal distribution
2. Apply a leaky integrator: `y[n] = alpha * y[n-1] + (1-alpha) * x[n]`
   - Where `x[n]` is white noise and `alpha` is close to 1 (0.99 in this implementation)
3. Normalize the output to prevent clipping
4. Save as a 16-bit WAV file

## Use Cases

- Sound masking for improved concentration
- Sleep aid
- Tinnitus relief
- Relaxation and stress reduction
- Audio testing and calibration

## License

This project is licensed under the CC0 License - see the LICENSE file for details.
