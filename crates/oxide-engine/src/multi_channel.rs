//! Multi-Channel Hierarchy & Parameterized Sheet Replication.
//!
//! Evaluates `Repeat(SheetName, StartIndex, EndIndex)` expressions, generates channel instances,
//! and binds multi-channel schematic hierarchy to 2D PCB room layout definitions.

use std::collections::HashMap;
use oxide_types::schematic::{ChildSheet, SchematicSheet};

/// A single instantiated channel resulting from a multi-channel expansion.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelInstance {
    pub channel_index: usize,
    pub channel_suffix: String,
    pub sheet_name: String,
    pub designator_map: HashMap<String, String>,
    pub net_prefix: String,
}

/// Expands a multi-channel sheet instance (e.g. `Repeat(AudioChannel, 1, 8)`).
pub fn parse_and_expand_repeat(
    sheet_name: &str,
    child_sheet: &ChildSheet,
    child_content: &SchematicSheet,
) -> Vec<ChannelInstance> {
    let name_trimmed = sheet_name.trim();
    if !name_trimmed.starts_with("Repeat(") && !name_trimmed.starts_with("repeat(") {
        return vec![ChannelInstance {
            channel_index: 1,
            channel_suffix: String::new(),
            sheet_name: child_sheet.name.clone(),
            designator_map: HashMap::new(),
            net_prefix: child_sheet.name.clone(),
        }];
    }

    // Parse Repeat(BaseName, Start, End)
    let inner = name_trimmed
        .trim_start_matches("Repeat(")
        .trim_start_matches("repeat(")
        .trim_end_matches(')');
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    let (base_name, start_idx, end_idx) = if parts.len() == 3 {
        let b = parts[0];
        let s = parts[1].parse::<usize>().unwrap_or(1);
        let e = parts[2].parse::<usize>().unwrap_or(s);
        (b, s, e)
    } else if parts.len() == 2 {
        let b = parts[0];
        let count = parts[1].parse::<usize>().unwrap_or(1);
        (b, 1, count)
    } else {
        (child_sheet.name.as_str(), 1, 1)
    };

    let mut instances = Vec::new();

    for idx in start_idx..=end_idx {
        let suffix = format!("_CH{}", idx);
        let mut des_map = HashMap::new();

        for sym in &child_content.symbols {
            let original_ref = sym.reference.clone();
            let new_ref = format!("{}{}", original_ref, suffix);
            des_map.insert(original_ref, new_ref);
        }

        instances.push(ChannelInstance {
            channel_index: idx,
            channel_suffix: suffix.clone(),
            sheet_name: format!("{}{}", base_name, suffix),
            designator_map: des_map,
            net_prefix: format!("{}{}", base_name, suffix),
        });
    }

    instances
}
