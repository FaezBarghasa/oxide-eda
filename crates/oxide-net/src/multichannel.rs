//! Multi-Channel Schematic Hierarchy and Net Bus Expansion Engine.
//!
//! Handles `Repeat(SheetName, StartIdx, EndIdx)` instantiation, channel-scoped net naming,
//! component suffix assignment (`U1_CH1`, `U1_CH2`), and multi-channel bus breakout.

use std::collections::HashMap;
use oxide_types::net::{Net, NetId, Netlist, Terminal};
use oxide_types::schematic::SchematicSheet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Channel instantiation descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelInstantiation {
    pub sheet_name: String,
    pub channel_prefix: String,
    pub index_start: u32,
    pub index_end: u32,
}

/// Expanded channel component metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelComponent {
    pub original_reference: String,
    pub channel_reference: String,
    pub channel_index: u32,
    pub symbol_uuid: Uuid,
}

/// Multi-channel hierarchy expansion engine.
pub struct MultiChannelEngine;

impl MultiChannelEngine {
    /// Expand a template schematic sheet across N repeated channels.
    pub fn expand_channels(
        template_sheet: &SchematicSheet,
        instantiation: &ChannelInstantiation,
    ) -> (Vec<SchematicSheet>, Vec<ChannelComponent>) {
        let mut sheets = Vec::new();
        let mut components = Vec::new();

        for ch in instantiation.index_start..=instantiation.index_end {
            let mut sheet_clone = template_sheet.clone();
            let ch_suffix = format!("{}_{ch}", instantiation.channel_prefix);

            for sym in &mut sheet_clone.symbols {
                let orig_ref = sym.reference.clone();
                let expanded_ref = format!("{orig_ref}_{ch_suffix}");
                sym.reference = expanded_ref.clone();
                let new_uuid = Uuid::new_v4();
                sym.uuid = new_uuid;

                components.push(ChannelComponent {
                    original_reference: orig_ref,
                    channel_reference: expanded_ref,
                    channel_index: ch,
                    symbol_uuid: new_uuid,
                });
            }

            // Scope all local net labels with channel suffix
            for lbl in &mut sheet_clone.labels {
                lbl.text = format!("{}_{ch_suffix}", lbl.text);
            }

            sheets.push(sheet_clone);
        }

        (sheets, components)
    }

    /// Generate channel-scoped Netlist from an existing flat netlist and channel instances.
    pub fn scope_netlist_for_channels(
        base_netlist: &Netlist,
        channel_count: u32,
        prefix: &str,
    ) -> Netlist {
        let mut expanded_nets = Vec::new();
        let mut next_net_id: u32 = 1000;

        for ch in 1..=channel_count {
            let ch_tag = format!("{prefix}_{ch}");
            for net in &base_netlist.nets {
                // Global/Power nets (GND, VCC) stay global without suffix
                let is_global = net.name.eq_ignore_ascii_case("GND")
                    || net.name.eq_ignore_ascii_case("VCC")
                    || net.name.eq_ignore_ascii_case("+3V3")
                    || net.name.eq_ignore_ascii_case("+5V");

                let scoped_name = if is_global {
                    net.name.clone()
                } else {
                    format!("{}_{ch_tag}", net.name)
                };

                let scoped_terminals: Vec<Terminal> = net
                    .terminals
                    .iter()
                    .map(|t| Terminal {
                        symbol: Uuid::new_v4(),
                        reference: format!("{}_{ch_tag}", t.reference),
                        pin: t.pin.clone(),
                        internal_delay_ps: t.internal_delay_ps,
                    })
                    .collect();

                expanded_nets.push(Net {
                    id: NetId(next_net_id),
                    name: scoped_name,
                    class: net.class.clone(),
                    wires: Vec::new(),
                    junctions: Vec::new(),
                    terminals: scoped_terminals,
                });
                next_net_id += 1;
            }
        }

        Netlist {
            nets: expanded_nets,
            xsignals: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multichannel_netlist_scoping() {
        let base = Netlist {
            nets: vec![
                Net {
                    id: NetId(1),
                    name: "AUDIO_IN".to_string(),
                    class: None,
                    wires: vec![],
                    junctions: vec![],
                    terminals: vec![Terminal {
                        symbol: Uuid::new_v4(),
                        reference: "C1".to_string(),
                        pin: "1".to_string(),
                        internal_delay_ps: 0.0,
                    }],
                },
                Net {
                    id: NetId(2),
                    name: "GND".to_string(),
                    class: None,
                    wires: vec![],
                    junctions: vec![],
                    terminals: vec![Terminal {
                        symbol: Uuid::new_v4(),
                        reference: "C1".to_string(),
                        pin: "2".to_string(),
                        internal_delay_ps: 0.0,
                    }],
                },
            ],
            xsignals: vec![],
        };

        let scoped = MultiChannelEngine::scope_netlist_for_channels(&base, 4, "CH");
        assert_eq!(scoped.nets.len(), 8); // 4 channels * 2 nets

        let ch1_audio = scoped.nets.iter().find(|n| n.name == "AUDIO_IN_CH_1").unwrap();
        assert_eq!(ch1_audio.terminals[0].reference, "C1_CH_1");

        let ch1_gnd = scoped.nets.iter().find(|n| n.name == "GND" && n.terminals[0].reference == "C1_CH_1").unwrap();
        assert_eq!(ch1_gnd.name, "GND"); // GND preserved as global
    }
}
