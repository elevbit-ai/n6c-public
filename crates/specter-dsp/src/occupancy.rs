use crate::DSPResult;

#[derive(Debug, Clone)]
pub struct ChannelOccupancyCalculator {
    channel_bandwidth_hz: u64,
    center_freq_hz: u64,
    sample_rate: u32,
    threshold_db: f64,
}

impl ChannelOccupancyCalculator {
    pub fn new(
        channel_bandwidth_hz: u64,
        center_freq_hz: u64,
        sample_rate: u32,
        threshold_db: f64,
    ) -> Self {
        Self {
            channel_bandwidth_hz,
            center_freq_hz,
            sample_rate,
            threshold_db,
        }
    }

    pub fn calculate(
        &self,
        psd_db: &[f64],
        freq_resolution: f64,
        base_freq: f64,
        noise_floor_db: f64,
    ) -> DSPResult<ChannelOccupancyResult> {
        if psd_db.is_empty() {
            return Err(crate::DSPError::ProcessingError(
                "Empty PSD data".to_string(),
            ));
        }
        let channel_start = self.center_freq_hz as f64 - self.channel_bandwidth_hz as f64 / 2.0;
        let channel_end = self.center_freq_hz as f64 + self.channel_bandwidth_hz as f64 / 2.0;
        let mut bins_in_channel = 0;
        let mut bins_above_threshold = 0;
        let mut total_power = 0.0;
        let mut peak_power = f64::NEG_INFINITY;
        for (i, &power) in psd_db.iter().enumerate() {
            let freq = base_freq + i as f64 * freq_resolution;
            if freq >= channel_start && freq <= channel_end {
                bins_in_channel += 1;
                total_power += power;
                if power > peak_power {
                    peak_power = power;
                }
                if power > noise_floor_db + self.threshold_db {
                    bins_above_threshold += 1;
                }
            }
        }
        let occupancy = if bins_in_channel > 0 {
            bins_above_threshold as f64 / bins_in_channel as f64
        } else {
            0.0
        };
        let mean_power = if bins_in_channel > 0 {
            total_power / bins_in_channel as f64
        } else {
            noise_floor_db
        };
        let snr = mean_power - noise_floor_db;
        Ok(ChannelOccupancyResult {
            frequency_hz: self.center_freq_hz,
            bandwidth_hz: self.channel_bandwidth_hz,
            occupancy,
            mean_power_dbm: mean_power,
            peak_power_dbm: peak_power,
            noise_floor_dbm: noise_floor_db,
            snr_db: snr,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChannelOccupancyResult {
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub occupancy: f64,
    pub mean_power_dbm: f64,
    pub peak_power_dbm: f64,
    pub noise_floor_dbm: f64,
    pub snr_db: f64,
}

pub struct MultiChannelOccupancy {
    calculators: Vec<ChannelOccupancyCalculator>,
}

impl MultiChannelOccupancy {
    pub fn new(channels: &[(u64, u64)], sample_rate: u32, threshold_db: f64) -> Self {
        let calculators = channels
            .iter()
            .map(|&(freq, bw)| {
                ChannelOccupancyCalculator::new(bw, freq, sample_rate, threshold_db)
            })
            .collect();
        Self { calculators }
    }

    pub fn calculate_all(
        &self,
        psd_db: &[f64],
        freq_resolution: f64,
        base_freq: f64,
        noise_floor_db: f64,
    ) -> DSPResult<Vec<ChannelOccupancyResult>> {
        self.calculators
            .iter()
            .map(|c| c.calculate(psd_db, freq_resolution, base_freq, noise_floor_db))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_occupancy() {
        let calc = ChannelOccupancyCalculator::new(25_000, 433_000_000, 2_400_000, 10.0);
        let psd_db: Vec<f64> = (0..128).map(|i| -80.0 + (i as f64 * 0.5)).collect();
        let freq_res = 2_400_000.0 / 256.0;
        let base = 432_000_000.0;
        let result = calc.calculate(&psd_db, freq_res, base, -90.0).unwrap();
        assert!(result.occupancy >= 0.0 && result.occupancy <= 1.0);
    }
}
