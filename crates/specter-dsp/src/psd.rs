use crate::fft::{FFTConfig, FFTProcessor, WindowFunction};
use crate::DSPResult;

#[derive(Debug, Clone)]
pub struct PSDConfig {
    pub fft_size: u32,
    pub sample_rate: u32,
    pub window: WindowFunction,
    pub overlap: f64,
    pub averaging: u32,
}

impl Default for PSDConfig {
    fn default() -> Self {
        Self {
            fft_size: 4096,
            sample_rate: 2_400_000,
            window: WindowFunction::Hann,
            overlap: 0.5,
            averaging: 10,
        }
    }
}

pub struct PSDProcessor {
    fft: FFTProcessor,
    config: PSDConfig,
    accumulator: Vec<f64>,
    frame_count: u32,
}

impl PSDProcessor {
    pub fn new(config: PSDConfig) -> DSPResult<Self> {
        let fft = FFTProcessor::new(FFTConfig {
            size: config.fft_size,
            window: config.window,
            overlap: config.overlap,
        })?;
        let n_bins = config.fft_size as usize / 2;
        Ok(Self {
            fft,
            config,
            accumulator: vec![0.0; n_bins],
            frame_count: 0,
        })
    }

    pub fn process(&mut self, samples: &[f64]) -> DSPResult<Vec<f64>> {
        let magnitudes = self.fft.process(samples)?;
        let n_bins = magnitudes.len();
        for (acc, mag) in self.accumulator.iter_mut().zip(magnitudes.iter()) {
            *acc += mag;
        }
        self.frame_count += 1;
        if self.frame_count >= self.config.averaging {
            let avg: Vec<f64> = self
                .accumulator
                .iter()
                .map(|a| *a / self.frame_count as f64)
                .collect();
            self.accumulator.iter_mut().for_each(|a| *a = 0.0);
            self.frame_count = 0;
            return Ok(avg);
        }
        Ok(magnitudes)
    }

    pub fn get_psd_db(&self) -> Vec<f64> {
        self.accumulator
            .iter()
            .map(|m| {
                if *m > 0.0 {
                    10.0 * m.log10()
                } else {
                    -200.0
                }
            })
            .collect()
    }

    pub fn reset(&mut self) {
        self.accumulator.iter_mut().for_each(|a| *a = 0.0);
        self.frame_count = 0;
    }
}

pub fn magnitudes_to_db(magnitudes: &[f64]) -> Vec<f64> {
    magnitudes
        .iter()
        .map(|m| {
            if *m > 0.0 {
                20.0 * m.log10()
            } else {
                -200.0
            }
        })
        .collect()
}

pub fn compute_psd(
    samples: &[f64],
    fft_size: u32,
    sample_rate: u32,
    window: WindowFunction,
) -> DSPResult<(Vec<f64>, Vec<f64>)> {
    let processor = FFTProcessor::new(FFTConfig {
        size: fft_size,
        window,
        overlap: 0.5,
    })?;
    let magnitudes = processor.process(samples)?;
    let freqs: Vec<f64> = (0..magnitudes.len())
        .map(|i| (i as f64 * sample_rate as f64) / fft_size as f64)
        .collect();
    let psd_db = magnitudes_to_db(&magnitudes);
    Ok((freqs, psd_db))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psd_processor() {
        let config = PSDConfig {
            fft_size: 256,
            sample_rate: 1000,
            window: WindowFunction::Hann,
            overlap: 0.5,
            averaging: 1,
        };
        let mut processor = PSDProcessor::new(config).unwrap();
        let samples: Vec<f64> = (0..256).map(|i| (i as f64 * 0.1).sin()).collect();
        let result = processor.process(&samples).unwrap();
        assert_eq!(result.len(), 128);
    }
}
