use std::collections::BTreeMap;

use crate::harvester::types::{DiscoveredPin, ElectricalPinType};
use crate::primitive::symbol::{
    PinDirection, PinOrientation, PinSymbolKind, Symbol, SymbolGraphic, SymbolGraphicKind,
    SymbolPin,
};

/// 100 mil grid quantum in millimeters.
pub const GRID_100_MIL_MM: f64 = 2.54;

/// Partitioning strategy for schematic symbol generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitioningStrategy {
    /// Single unitary symbol body.
    SingleUnit,
    /// Homogeneous functional units (e.g. Dual/Quad Op-Amps, Hex Inverters).
    HomogeneousArray { units: usize, share_power_unit: bool },
    /// Heterogeneous bank-based partitioning (e.g. MCUs, FPGAs, SoCs).
    HeterogeneousBanked,
}

/// Procedural Schematic Symbol Synthesizer with Multi-Gate Partitioning and Grid Snapping.
pub struct MultiGateSymbolGenerator;

impl MultiGateSymbolGenerator {
    /// Maps Harvester `ElectricalPinType` to primitive `PinDirection`.
    pub fn map_electrical_type(el: ElectricalPinType) -> PinDirection {
        match el {
            ElectricalPinType::Input => PinDirection::Input,
            ElectricalPinType::Output => PinDirection::Output,
            ElectricalPinType::Bidirectional => PinDirection::Bidirectional,
            ElectricalPinType::Power => PinDirection::Power,
            ElectricalPinType::Ground => PinDirection::Power,
            ElectricalPinType::Passive => PinDirection::Passive,
            ElectricalPinType::TriState => PinDirection::Tristate,
            ElectricalPinType::OpenCollector => PinDirection::OpenCollector,
            ElectricalPinType::OpenEmitter => PinDirection::OpenEmitter,
            ElectricalPinType::NoConnect => PinDirection::NotConnected,
        }
    }

    /// Detects whether a pin name represents active-low logic.
    pub fn is_active_low(name: &str) -> bool {
        let n = name.trim();
        n.ends_with('#')
            || n.ends_with("_N")
            || n.ends_with("_B")
            || n.starts_with('/')
            || n.starts_with('\\')
            || n.starts_with('~')
            || (n.starts_with('n') && n.len() > 1 && n.chars().nth(1).unwrap_or(' ').is_ascii_uppercase())
    }

    /// Automatically detects partitioning strategy based on pin count, names, and description.
    pub fn detect_partitioning(pins: &[DiscoveredPin], description: &str) -> PartitioningStrategy {
        let desc_upper = description.to_uppercase();
        let pin_count = pins.len();

        if desc_upper.contains("DUAL OP") || desc_upper.contains("DUAL OPERATIONAL") || (pin_count == 8 && desc_upper.contains("OPAMP")) {
            PartitioningStrategy::HomogeneousArray { units: 2, share_power_unit: true }
        } else if desc_upper.contains("QUAD OP") || desc_upper.contains("QUAD OPERATIONAL") || (pin_count == 14 && desc_upper.contains("OPAMP")) {
            PartitioningStrategy::HomogeneousArray { units: 4, share_power_unit: true }
        } else if desc_upper.contains("HEX INVERTER") || desc_upper.contains("HEX BUFFER") {
            PartitioningStrategy::HomogeneousArray { units: 6, share_power_unit: true }
        } else if pin_count > 32 || pins.iter().any(|p| p.io_bank.is_some()) {
            PartitioningStrategy::HeterogeneousBanked
        } else {
            PartitioningStrategy::SingleUnit
        }
    }

    /// Synthesizes a standardized, electrically partitioned multi-gate symbol.
    pub fn synthesize(
        name: &str,
        designator_prefix: &str,
        description: &str,
        pins: &[DiscoveredPin],
    ) -> Symbol {
        let strategy = Self::detect_partitioning(pins, description);
        let mut sym = Symbol::empty(name);
        sym.designator = format!("{designator_prefix}?");
        sym.description = description.into();

        match strategy {
            PartitioningStrategy::SingleUnit => {
                Self::build_single_unit_symbol(&mut sym, pins, 1);
            }
            PartitioningStrategy::HomogeneousArray { units, share_power_unit: _ } => {
                Self::build_homogeneous_multi_gate(&mut sym, pins, units);
            }
            PartitioningStrategy::HeterogeneousBanked => {
                Self::build_heterogeneous_banked(&mut sym, pins);
            }
        }

        sym
    }

    fn build_single_unit_symbol(sym: &mut Symbol, pins: &[DiscoveredPin], part_number: u8) {
        let mut left_pins: Vec<&DiscoveredPin> = Vec::new();
        let mut right_pins: Vec<&DiscoveredPin> = Vec::new();
        let mut top_pins: Vec<&DiscoveredPin> = Vec::new();
        let mut bottom_pins: Vec<&DiscoveredPin> = Vec::new();

        for pin in pins {
            match pin.electrical_type {
                ElectricalPinType::Input | ElectricalPinType::Bidirectional => left_pins.push(pin),
                ElectricalPinType::Output
                | ElectricalPinType::TriState
                | ElectricalPinType::OpenCollector
                | ElectricalPinType::OpenEmitter => right_pins.push(pin),
                ElectricalPinType::Power => top_pins.push(pin),
                ElectricalPinType::Ground | ElectricalPinType::NoConnect | ElectricalPinType::Passive => {
                    bottom_pins.push(pin)
                }
            }
        }

        // If passive 2-pin part (e.g. Resistor / Capacitor)
        if pins.len() == 2 && top_pins.is_empty() && bottom_pins.is_empty() {
            let mut p1 = SymbolPin::new(&pins[0].number, &pins[0].name);
            p1.electrical = Self::map_electrical_type(pins[0].electrical_type);
            p1.position = [-GRID_100_MIL_MM * 2.0, 0.0];
            p1.length = GRID_100_MIL_MM;
            p1.orientation = PinOrientation::Right;
            p1.part_number = part_number;
            sym.pins.push(p1);

            let mut p2 = SymbolPin::new(&pins[1].number, &pins[1].name);
            p2.electrical = Self::map_electrical_type(pins[1].electrical_type);
            p2.position = [GRID_100_MIL_MM * 2.0, 0.0];
            p2.length = GRID_100_MIL_MM;
            p2.orientation = PinOrientation::Left;
            p2.part_number = part_number;
            sym.pins.push(p2);

            sym.graphics.push(SymbolGraphic {
                kind: SymbolGraphicKind::Rectangle {
                    from: [-GRID_100_MIL_MM, -GRID_100_MIL_MM * 0.5],
                    to: [GRID_100_MIL_MM, GRID_100_MIL_MM * 0.5],
                },
                stroke_width: 0.15,
                fill: None,
                part_number,
            });
            return;
        }

        let max_vertical = left_pins.len().max(right_pins.len()).max(2);
        let body_h = (max_vertical as f64 + 1.0) * GRID_100_MIL_MM;
        let body_w = (GRID_100_MIL_MM * 6.0).max((top_pins.len().max(bottom_pins.len()) as f64 + 2.0) * GRID_100_MIL_MM);

        let half_w = body_w / 2.0;
        let half_h = body_h / 2.0;

        // Snapped rectangular body
        sym.graphics.push(SymbolGraphic {
            kind: SymbolGraphicKind::Rectangle {
                from: [-half_w, -half_h],
                to: [half_w, half_h],
            },
            stroke_width: 0.20,
            fill: None,
            part_number,
        });

        // Left pins (Inputs / Enables)
        let y_start_left = half_h - GRID_100_MIL_MM;
        for (i, pin) in left_pins.iter().enumerate() {
            let y = y_start_left - (i as f64 * GRID_100_MIL_MM);
            let mut sp = SymbolPin::new(&pin.number, &pin.name);
            sp.electrical = Self::map_electrical_type(pin.electrical_type);
            sp.position = [-half_w - GRID_100_MIL_MM, y];
            sp.length = GRID_100_MIL_MM;
            sp.orientation = PinOrientation::Right;
            sp.part_number = part_number;
            if Self::is_active_low(&pin.name) {
                sp.outside_edge_symbol = PinSymbolKind::Dot;
            }
            sym.pins.push(sp);
        }

        // Right pins (Outputs)
        let y_start_right = half_h - GRID_100_MIL_MM;
        for (i, pin) in right_pins.iter().enumerate() {
            let y = y_start_right - (i as f64 * GRID_100_MIL_MM);
            let mut sp = SymbolPin::new(&pin.number, &pin.name);
            sp.electrical = Self::map_electrical_type(pin.electrical_type);
            sp.position = [half_w + GRID_100_MIL_MM, y];
            sp.length = GRID_100_MIL_MM;
            sp.orientation = PinOrientation::Left;
            sp.part_number = part_number;
            if Self::is_active_low(&pin.name) {
                sp.outside_edge_symbol = PinSymbolKind::Dot;
            }
            sym.pins.push(sp);
        }

        // Top pins (Power)
        for (i, pin) in top_pins.iter().enumerate() {
            let x = -half_w + GRID_100_MIL_MM * (i as f64 + 1.0);
            let mut sp = SymbolPin::new(&pin.number, &pin.name);
            sp.electrical = Self::map_electrical_type(pin.electrical_type);
            sp.position = [x, half_h + GRID_100_MIL_MM];
            sp.length = GRID_100_MIL_MM;
            sp.orientation = PinOrientation::Down;
            sp.part_number = part_number;
            sym.pins.push(sp);
        }

        // Bottom pins (Ground / Returns)
        for (i, pin) in bottom_pins.iter().enumerate() {
            let x = -half_w + GRID_100_MIL_MM * (i as f64 + 1.0);
            let mut sp = SymbolPin::new(&pin.number, &pin.name);
            sp.electrical = Self::map_electrical_type(pin.electrical_type);
            sp.position = [x, -half_h - GRID_100_MIL_MM];
            sp.length = GRID_100_MIL_MM;
            sp.orientation = PinOrientation::Up;
            sp.part_number = part_number;
            sym.pins.push(sp);
        }
    }

    fn build_homogeneous_multi_gate(sym: &mut Symbol, pins: &[DiscoveredPin], units: usize) {
        // Group pins into functional unit slices and global power pins (Part 0)
        let mut power_ground_pins = Vec::new();
        let mut unit_pins: Vec<Vec<DiscoveredPin>> = vec![Vec::new(); units];

        for pin in pins {
            if pin.electrical_type == ElectricalPinType::Power || pin.electrical_type == ElectricalPinType::Ground {
                power_ground_pins.push(pin.clone());
            } else {
                // Heuristic matching unit index from pin name (e.g. IN1+ -> unit 0, IN2+ -> unit 1)
                let unit_idx = pin
                    .name
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .and_then(|c| c.to_digit(10))
                    .map(|d| (d as usize).saturating_sub(1).min(units - 1))
                    .unwrap_or(0);
                unit_pins[unit_idx].push(pin.clone());
            }
        }

        // Build each functional unit gate
        for (idx, u_pins) in unit_pins.iter().enumerate() {
            let part_num = (idx + 1) as u8;
            Self::build_single_unit_symbol(sym, u_pins, part_num);
        }

        // Shared Power/Ground pins on Part 0 (visible on all parts or separate power unit)
        for pin in power_ground_pins {
            let mut sp = SymbolPin::new(&pin.number, &pin.name);
            sp.electrical = Self::map_electrical_type(pin.electrical_type);
            sp.part_number = 0; // Part 0 = shared across all sub-parts
            sp.position = [0.0, if pin.electrical_type == ElectricalPinType::Power { GRID_100_MIL_MM * 3.0 } else { -GRID_100_MIL_MM * 3.0 }];
            sp.length = GRID_100_MIL_MM;
            sp.orientation = if pin.electrical_type == ElectricalPinType::Power { PinOrientation::Down } else { PinOrientation::Up };
            sym.pins.push(sp);
        }
    }

    fn build_heterogeneous_banked(sym: &mut Symbol, pins: &[DiscoveredPin]) {
        // Partition pins by IO Bank or functional group
        let mut banks: BTreeMap<u32, Vec<DiscoveredPin>> = BTreeMap::new();
        let mut unassigned = Vec::new();

        for pin in pins {
            if let Some(bank) = pin.io_bank {
                banks.entry(bank).or_default().push(pin.clone());
            } else if pin.electrical_type == ElectricalPinType::Power || pin.electrical_type == ElectricalPinType::Ground {
                banks.entry(99).or_default().push(pin.clone()); // Bank 99 = Power & Ground
            } else {
                unassigned.push(pin.clone());
            }
        }

        if banks.is_empty() {
            // Slice into chunks of 24 pins per unit
            let chunk_size = 24;
            for (part_idx, chunk) in pins.chunks(chunk_size).enumerate() {
                Self::build_single_unit_symbol(sym, chunk, (part_idx + 1) as u8);
            }
        } else {
            let mut part_counter = 1u8;
            for (_bank, bank_pins) in banks {
                Self::build_single_unit_symbol(sym, &bank_pins, part_counter);
                part_counter += 1;
            }
            if !unassigned.is_empty() {
                Self::build_single_unit_symbol(sym, &unassigned, part_counter);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opamp_multi_gate_partitioning() {
        let pins = vec![
            DiscoveredPin::new("1", "OUT1").with_type(ElectricalPinType::Output),
            DiscoveredPin::new("2", "IN1-").with_type(ElectricalPinType::Input),
            DiscoveredPin::new("3", "IN1+").with_type(ElectricalPinType::Input),
            DiscoveredPin::new("4", "VEE").with_type(ElectricalPinType::Ground),
            DiscoveredPin::new("5", "IN2+").with_type(ElectricalPinType::Input),
            DiscoveredPin::new("6", "IN2-").with_type(ElectricalPinType::Input),
            DiscoveredPin::new("7", "OUT2").with_type(ElectricalPinType::Output),
            DiscoveredPin::new("8", "VCC").with_type(ElectricalPinType::Power),
        ];

        let sym = MultiGateSymbolGenerator::synthesize("LM358", "U", "Dual Operational Amplifier", &pins);
        assert_eq!(sym.pins.len(), 8);

        // Power pins should be on Part 0
        let vcc = sym.pins.iter().find(|p| p.number == "8").expect("VCC present");
        assert_eq!(vcc.part_number, 0);

        let out1 = sym.pins.iter().find(|p| p.number == "1").expect("OUT1 present");
        assert_eq!(out1.part_number, 1);

        let out2 = sym.pins.iter().find(|p| p.number == "7").expect("OUT2 present");
        assert_eq!(out2.part_number, 2);
    }

    #[test]
    fn grid_snapping_invariance() {
        let pins = vec![
            DiscoveredPin::new("1", "IN").with_type(ElectricalPinType::Input),
            DiscoveredPin::new("2", "OUT").with_type(ElectricalPinType::Output),
        ];
        let sym = MultiGateSymbolGenerator::synthesize("BUF", "U", "Buffer", &pins);
        for pin in &sym.pins {
            let rem_x = pin.position[0].abs() % GRID_100_MIL_MM;
            let rem_y = pin.position[1].abs() % GRID_100_MIL_MM;
            assert!(rem_x < 1e-4 || (GRID_100_MIL_MM - rem_x).abs() < 1e-4);
            assert!(rem_y < 1e-4 || (GRID_100_MIL_MM - rem_y).abs() < 1e-4);
        }
    }
}
