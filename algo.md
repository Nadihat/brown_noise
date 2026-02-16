# Brown Noise Algorithm Analysis

The current implementation in `src/main.rs` is ported from `script.js`. It uses a leaky integrator approach to generate brown noise.

## The Algorithm

```rust
last = (last + 0.02 * white) / 1.02;
output = last * 3.5;
```

Where `white` is uniform random noise between -1.0 and 1.0.

## Analysis

### 1. Mathematical Structure
This is a first-order Infinite Impulse Response (IIR) low-pass filter applied to white noise.
The formula can be rewritten as:
`y[n] = (1/1.02) * y[n-1] + (0.02/1.02) * x[n]`

Let $\alpha = 1/1.02 \approx 0.98039$.
The equation becomes:
`y[n] = \alpha * y[n-1] + (1 - \alpha) * x[n]`

This is a standard leaky integrator.

### 2. Spectral Characteristics
*   **Ideal Brown Noise:** Should have a spectral density that decreases by 6 dB per octave ($1/f^2$). This is achieved by mathematical integration of white noise.
*   **Leaky Integrator:** Acts as a low-pass filter with a pole at $z = \alpha$.
    *   **High Frequencies:** The rolloff approximates 6 dB/octave, characteristic of brown noise.
    *   **Low Frequencies:** Below the cutoff frequency determined by $\alpha$, the spectrum flattens out (becomes white).
    *   **Cutoff Frequency:** With $\alpha \approx 0.98$, the cutoff is relatively high relative to the sample rate. For 44.1kHz, the cutoff is roughly $f_c \approx -\ln(\alpha) \cdot f_s / (2\pi) \approx 139$ Hz.

### 3. Comparison to "True" Brown Noise
True brown noise is a random walk (perfect integration), where $\alpha = 1$. However, perfect integration drifts infinitely.
*   **Pros of this algorithm:** Ideally stable. The "leak" ($\alpha < 1$) keeps the amplitude bounded around 0, preventing the signal from wandering off (DC offset drift) which causes clipping in audio buffers.
*   **Cons:** It flattens out at very low frequencies (acting like white noise below ~140Hz), whereas true brown noise continues to rise in power as frequency decreases.

### 4. Implementation Details
*   **Uniform vs Gaussian:** The code uses Uniform distribution (-1 to 1). While Gaussian is often preferred for natural signals, the spectral color (frequency content) is determined by the filter, not the amplitude distribution. The Central Limit Theorem suggests the output of the IIR filter will approach a Gaussian distribution regardless of the input distribution.
*   **Coefficients:** The coefficients `0.02` and `1.02` are specific to the implementation in `script.js`.
*   **Gain:** The factor `3.5` attempts to normalize the volume perceptually or relative to digital full scale, though peak amplitude is stochastic.

## Conclusion
The algorithm is a **Leaky Integrator**. It is a standard approximation for Brown noise in audio synthesis because it is computationally cheap and numerically stable (prevents infinite drift). It sounds "brown" enough for most relaxation purposes, though it technically lacks the extreme low-end power of mathematical Brownian motion.

## Multiplier & Low Pass Filter
In `script.js`, a "multiplier" slider adjusts the brightness of the noise. This is implemented via a dynamic Low Pass Filter (LPF) using the Web Audio API's `BiquadFilterNode` (type: 'lowpass', Q: 1).

The Rust implementation (`src/main.rs`) replicates this behavior by:
1.  **Mapping Multiplier to Frequency:** It uses the same exponential mapping logic to convert the multiplier (1.0 - 35.0) into a cutoff frequency (Hz).
2.  **Digital Biquad Implementation:** It implements a standard digital Biquad filter (Direct Form I) manually.
    *   It calculates the coefficients ($b_0, b_1, b_2, a_0, a_1, a_2$) using the Audio EQ Cookbook formulas for a Lowpass filter with Q=1.
    *   It applies the difference equation: $y[n] = b_0 x[n] + b_1 x[n-1] + b_2 x[n-2] - a_1 y[n-1] - a_2 y[n-2]$.

This ensures the Rust output spectrally matches the browser implementation when the multiplier is adjusted.

## Deviations from Reference (script.js)

While the core generation and filtering are identical, the signal chain differs slightly at the final output stage:

1.  **Limiter / Soft-Clipper:**
    *   **script.js:** Uses a `WaveShaperNode` with a `tanh` curve (`Math.tanh(x*2)`) *after* the gain stage to soft-clip peaks. This adds harmonic distortion when the signal is loud and prevents harsh digital clipping.
    *   **Rust:** Uses hard clamping (`output.clamp(-1.0, 1.0)`). If the generated brown noise (which is stochastic) exceeds ±1.0 after amplification, it is hard-clipped.

2.  **Gain Staging:**
    *   **script.js:** The browser implementation has a `MAX_SAFE_GAIN` of 0.9 applied to the master gain.
    *   **Rust:** The `amplitude` argument is applied directly as a linear multiplier before clamping.

3.  **Buffer Loop vs Streaming:**
    *   **script.js:** Generates a static 6-second buffer and loops it.
    *   **Rust:** Generates fresh random samples for the entire duration requested. This is generally superior for long files as it avoids audible loop points.
