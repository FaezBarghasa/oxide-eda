//! Physical units and metric/imperial conversion helpers for Oxide EDA.
//!
//! Internal representation uses integer `Microns` (`i64`) to guarantee
//! exact arithmetic and eliminate floating-point drift.

/// Integer micrometers (µm). 1 mm = 1,000 µm; 1 mil = 25.4 µm.
pub type Microns = i64;

/// Integer nanometers (nm). 1 µm = 1,000 nm.
pub type Nanometers = i64;

/// Standard 1 oz copper thickness in micrometers (~35 µm / 1.37 mil).
pub const COPPER_1OZ_MICRONS: Microns = 35;

/// Standard 0.5 oz (half-ounce) copper thickness in micrometers (~17.5-18 µm / 0.7 mil).
pub const COPPER_HALF_OZ_MICRONS: Microns = 18;

/// Standard 2 oz copper thickness in micrometers (~70 µm / 2.8 mil).
pub const COPPER_2OZ_MICRONS: Microns = 70;

/// Convert millimetres to integer micrometers.
#[inline]
pub fn mm_to_microns(mm: f64) -> Microns {
    (mm * 1000.0).round() as Microns
}

/// Convert integer micrometers to millimetres.
#[inline]
pub fn microns_to_mm(microns: Microns) -> f64 {
    microns as f64 / 1000.0
}

/// Convert mils (thousandths of an inch) to integer micrometers.
#[inline]
pub fn mils_to_microns(mils: f64) -> Microns {
    (mils * 25.4).round() as Microns
}

/// Convert integer micrometers to mils.
#[inline]
pub fn microns_to_mils(microns: Microns) -> f64 {
    microns as f64 / 25.4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversions() {
        assert_eq!(mm_to_microns(1.6), 1600);
        assert_eq!(mm_to_microns(0.15), 150);
        assert_eq!(microns_to_mm(1600), 1.6);
        assert_eq!(mils_to_microns(10.0), 254);
        assert!((microns_to_mils(254) - 10.0).abs() < 1e-6);
    }
}
