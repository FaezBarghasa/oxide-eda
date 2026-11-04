//! High-Speed SI Eye Diagram Texture Generator and Heatmap Accumulator.
//!
//! Accumulates high-density Unit Interval (UI) signal transitions into a 2D intensity heatmap
//! representation for real-time false-colour rendering and BER contour extraction.

/// 2D Eye Diagram Accumulator Grid.
#[derive(Debug, Clone)]
pub struct EyeDiagramHeatmap {
    pub width: usize,
    pub height: usize,
    pub samples_accumulated: usize,
    pub max_density: u32,
    pub data: Vec<u32>,
}

impl EyeDiagramHeatmap {
    /// Creates a new empty accumulator grid (typically 512x256 or 1024x512).
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples_accumulated: 0,
            max_density: 0,
            data: vec![0; width * height],
        }
    }

    /// Accumulates 2-UI normalized time-domain waveforms into the grid.
    /// `time_norm`: [0.0, 2.0] representing 2 UI periods.
    /// `voltage_norm`: [-1.0, 1.0] representing differential voltage range.
    pub fn accumulate_sample(&mut self, time_norm: f64, voltage_norm: f64) {
        if time_norm < 0.0 || time_norm > 2.0 || voltage_norm < -1.0 || voltage_norm > 1.0 {
            return;
        }

        let px = ((time_norm / 2.0) * (self.width as f64 - 1.0)).clamp(0.0, (self.width - 1) as f64) as usize;
        // Invert Y: 1.0 (top) -> 0, -1.0 (bottom) -> height - 1
        let py = (((1.0 - voltage_norm) / 2.0) * (self.height as f64 - 1.0)).clamp(0.0, (self.height - 1) as f64) as usize;

        let idx = py * self.width + px;
        self.data[idx] = self.data[idx].saturating_add(1);
        if self.data[idx] > self.max_density {
            self.max_density = self.data[idx];
        }
        self.samples_accumulated += 1;
    }

    /// Converts normalized density into an RGBA8 buffer using an Inferno/Turbo false-colour palette.
    pub fn to_rgba8(&self) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(self.width * self.height * 4);
        let max = self.max_density.max(1) as f64;

        for &val in &self.data {
            if val == 0 {
                // Background transparent / deep black
                rgba.extend_from_slice(&[13, 17, 23, 255]); // #0D1117 (Oxide Dark v2)
            } else {
                let intensity = (val as f64 / max).powf(0.5); // gamma boost
                let (r, g, b) = Self::inferno_colormap(intensity);
                rgba.extend_from_slice(&[r, g, b, 255]);
            }
        }
        rgba
    }

    /// High-contrast Inferno false-colour transfer function.
    fn inferno_colormap(t: f64) -> (u8, u8, u8) {
        let t = t.clamp(0.0, 1.0);
        if t < 0.25 {
            let k = t / 0.25;
            let r = (k * 80.0) as u8;
            let g = (k * 18.0) as u8;
            let b = (k * 120.0) as u8;
            (r, g, b)
        } else if t < 0.5 {
            let k = (t - 0.25) / 0.25;
            let r = (80.0 + k * 106.0) as u8;
            let g = (18.0 + k * 38.0) as u8;
            let b = (120.0 - k * 38.0) as u8;
            (r, g, b)
        } else if t < 0.75 {
            let k = (t - 0.5) / 0.25;
            let r = (186.0 + k * 65.0) as u8;
            let g = (56.0 + k * 86.0) as u8;
            let b = (82.0 - k * 62.0) as u8;
            (r, g, b)
        } else {
            let k = (t - 0.75) / 0.25;
            let r = 251;
            let g = (142.0 + k * 110.0) as u8;
            let b = (20.0 + k * 130.0) as u8;
            (r, g, b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_diagram_heatmap_accumulation() {
        let mut heatmap = EyeDiagramHeatmap::new(64, 32);
        heatmap.accumulate_sample(1.0, 0.0); // exact center of 2-UI eye
        assert_eq!(heatmap.samples_accumulated, 1);
        assert_eq!(heatmap.max_density, 1);

        let rgba = heatmap.to_rgba8();
        assert_eq!(rgba.len(), 64 * 32 * 4);
    }
}
