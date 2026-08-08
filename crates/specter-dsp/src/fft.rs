use thiserror::Error;

#[derive(Error, Debug)]
pub enum DSPError {
    #[error("Invalid FFT size: {0} (must be power of 2)")]
    InvalidFFTSize(u32),

    #[error("Buffer underflow: expected {expected} samples, got {got}")]
    BufferUnderflow { expected: usize, got: usize },

    #[error("Processing error: {0}")]
    ProcessingError(String),
}

pub type DSPResult<T> = std::result::Result<T, DSPError>;

#[derive(Debug, Clone)]
pub struct FFTConfig {
    pub size: u32,
    pub window: WindowFunction,
    pub overlap: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowFunction {
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Rectangular,
}

impl WindowFunction {
    pub fn apply(&self, sample: f64, index: usize, n: usize) -> f64 {
        let n_f = n as f64;
        let idx_f = index as f64;
        let coeff = match self {
            WindowFunction::Hann => 0.5 * (1.0 - (2.0 * std::f64::consts::PI * idx_f / n_f).cos()),
            WindowFunction::Hamming => {
                0.54 - 0.46 * (2.0 * std::f64::consts::PI * idx_f / n_f).cos()
            }
            WindowFunction::Blackman => {
                0.42 - 0.5 * (2.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    + 0.08 * (4.0 * std::f64::consts::PI * idx_f / n_f).cos()
            }
            WindowFunction::BlackmanHarris => {
                0.35875 - 0.48829 * (2.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    + 0.14128 * (4.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    - 0.01168 * (6.0 * std::f64::consts::PI * idx_f / n_f).cos()
            }
            WindowFunction::FlatTop => {
                0.21557895
                    - 0.41663158 * (2.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    + 0.277263158 * (4.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    - 0.083578947 * (6.0 * std::f64::consts::PI * idx_f / n_f).cos()
                    + 0.006947368 * (8.0 * std::f64::consts::PI * idx_f / n_f).cos()
            }
            WindowFunction::Rectangular => 1.0,
        };
        sample * coeff
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "hamming" => WindowFunction::Hamming,
            "blackman" => WindowFunction::Blackman,
            "blackmanharris" | "blackman-harris" => WindowFunction::BlackmanHarris,
            "flattop" | "flat-top" => WindowFunction::FlatTop,
            "rectangular" | "none" => WindowFunction::Rectangular,
            _ => WindowFunction::Hann,
        }
    }
}

pub struct FFTProcessor {
    config: FFTConfig,
    window_coeffs: Vec<f64>,
}

impl FFTProcessor {
    pub fn new(config: FFTConfig) -> DSPResult<Self> {
        if !config.size.is_power_of_two() {
            return Err(DSPError::InvalidFFTSize(config.size));
        }
        let n = config.size as usize;
        let window_coeffs: Vec<f64> = (0..n)
            .map(|i| config.window.apply(1.0, i, n))
            .collect();
        Ok(Self {
            config,
            window_coeffs,
        })
    }

    pub fn process(&self, input: &[f64]) -> DSPResult<Vec<f64>> {
        let n = self.config.size as usize;
        if input.len() < n {
            return Err(DSPError::BufferUnderflow {
                expected: n,
                got: input.len(),
            });
        }
        let mut buffer: Vec<f64> = input[..n]
            .iter()
            .zip(self.window_coeffs.iter())
            .map(|(s, w)| s * w)
            .collect();
        self.fft_in_place(&mut buffer);
        let magnitudes: Vec<f64> = buffer
            .chunks(2)
            .map(|c| (c[0].powi(2) + c[1].powi(2)).sqrt())
            .collect();
        Ok(magnitudes)
    }

    fn fft_in_place(&self, data: &mut [f64]) {
        let n = data.len() / 2;
        let mut j = 0;
        for i in 0..n {
            if i < j {
                data.swap(i * 2, j * 2);
                data.swap(i * 2 + 1, j * 2 + 1);
            }
            let mut m = n >> 1;
            while m >= 1 && j >= m {
                j -= m;
                m >>= 1;
            }
            j += m;
        }
        let mut len = 2;
        while len <= n {
            let half = len / 2;
            let angle = -2.0 * std::f64::consts::PI / len as f64;
            let w_re = angle.cos();
            let w_im = angle.sin();
            let mut i = 0;
            while i < n {
                let mut cur_re = 1.0;
                let mut cur_im = 0.0;
                for j in 0..half {
                    let u_re = data[(i + j) * 2];
                    let u_im = data[(i + j) * 2 + 1];
                    let v_re = data[(i + j + half) * 2] * cur_re
                        - data[(i + j + half) * 2 + 1] * cur_im;
                    let v_im = data[(i + j + half) * 2] * cur_im
                        + data[(i + j + half) * 2 + 1] * cur_re;
                    data[(i + j) * 2] = u_re + v_re;
                    data[(i + j) * 2 + 1] = u_im + v_im;
                    data[(i + j + half) * 2] = u_re - v_re;
                    data[(i + j + half) * 2 + 1] = u_im - v_im;
                    let new_re = cur_re * w_re - cur_im * w_im;
                    let new_im = cur_re * w_im + cur_im * w_re;
                    cur_re = new_re;
                    cur_im = new_im;
                }
                i += len;
            }
            len <<= 1;
        }
    }

    pub fn config(&self) -> &FFTConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_size_validation() {
        assert!(FFTProcessor::new(FFTConfig {
            size: 1024,
            window: WindowFunction::Hann,
            overlap: 0.5,
        })
        .is_ok());
        assert!(FFTProcessor::new(FFTConfig {
            size: 1000,
            window: WindowFunction::Hann,
            overlap: 0.5,
        })
        .is_err());
    }

    #[test]
    fn test_window_functions() {
        let hann = WindowFunction::Hann;
        let n = 100;
        assert!((hann.apply(1.0, 0, n) - 0.0).abs() < 0.01);
        assert!((hann.apply(1.0, n / 2, n) - 1.0).abs() < 0.01);
    }
}
