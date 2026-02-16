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
