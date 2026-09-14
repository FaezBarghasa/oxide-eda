//! S-Parameter Network Analysis and Smith Chart projection for RF circuits.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Complex number representation for RF impedance and scattering parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn to_db(&self) -> f64 {
        let mag = self.norm();
        if mag <= 1e-15 {
            -300.0
        } else {
            20.0 * mag.log10()
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn div(&self, other: &Self) -> Option<Self> {
        let denom = other.norm_sq();
        if denom == 0.0 {
            None
        } else {
            Some(Self {
                re: (self.re * other.re + self.im * other.im) / denom,
                im: (self.im * other.re - self.re * other.im) / denom,
            })
        }
    }
}

/// 2-Port Scattering Parameters Matrix ($S_{11}, S_{21}, S_{12}, S_{22}$).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameters2Port {
    pub freq_hz: f64,
    pub s11: Complex64,
    pub s21: Complex64,
    pub s12: Complex64,
    pub s22: Complex64,
    pub z0: f64,
}

impl SParameters2Port {
    /// Return Loss (dB) at Port 1: $RL_1 = -20 \log_{10} |S_{11}|$
    pub fn return_loss_port1_db(&self) -> f64 {
        -self.s11.to_db()
    }

    /// Insertion Loss (dB) from Port 1 to Port 2: $IL = -20 \log_{10} |S_{21}|$
    pub fn insertion_loss_db(&self) -> f64 {
        -self.s21.to_db()
    }

    /// Voltage Standing Wave Ratio (VSWR) at Port 1: $VSWR = \frac{1 + |S_{11}|}{1 - |S_{11}|}$
    pub fn vswr_port1(&self) -> f64 {
        let mag = self.s11.norm();
        if mag >= 0.99999 {
            100.0
        } else {
            (1.0 + mag) / (1.0 - mag)
        }
    }

    /// Converts $S_{11}$ reflection coefficient $\Gamma$ into normalized input impedance:
    /// $z_{in} = \frac{1 + \Gamma}{1 - \Gamma}$
    pub fn normalized_z_in(&self) -> Option<Complex64> {
        let num = Complex64::one().add(&self.s11);
        let den = Complex64::one().sub(&self.s11);
        num.div(&den)
    }

    /// Projected coordinate on a normalized Smith Chart (reflection coefficient $\Gamma = u + jv$).
    pub fn smith_chart_point(&self) -> (f64, f64) {
        (self.s11.re, self.s11.im)
    }
}

/// Complete S-parameter frequency dataset over a sweep.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SParameterDataset {
    pub z0: f64,
    pub points: Vec<SParameters2Port>,
}

impl SParameterDataset {
    pub fn new(z0: f64) -> Self {
        Self {
            z0,
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, point: SParameters2Port) {
        self.points.push(point);
    }

    /// Generates standard Touchstone v1.1 `.s2p` file string.
    pub fn to_touchstone_s2p(&self) -> String {
        let mut out = String::new();
        out.push_str("! Touchstone 2-Port S-Parameters exported from Oxide EDA\n");
        out.push_str(&format!("# HZ S DB R {}\n", self.z0));
        out.push_str("! freq  |S11| ang(S11)  |S21| ang(S21)  |S12| ang(S12)  |S22| ang(S22)\n");

        for p in &self.points {
            let s11_db = p.s11.to_db();
            let s11_deg = p.s11.arg() * 180.0 / PI;
            let s21_db = p.s21.to_db();
            let s21_deg = p.s21.arg() * 180.0 / PI;
            let s12_db = p.s12.to_db();
            let s12_deg = p.s12.arg() * 180.0 / PI;
            let s22_db = p.s22.to_db();
            let s22_deg = p.s22.arg() * 180.0 / PI;

            out.push_str(&format!(
                "{:e} {:.3} {:.2} {:.3} {:.2} {:.3} {:.2} {:.3} {:.2}\n",
                p.freq_hz, s11_db, s11_deg, s21_db, s21_deg, s12_db, s12_deg, s22_db, s22_deg
            ));
        }

        out
    }
}
