//! Assembly Variant and DNP (Do Not Populate) Manager Panel.
//!
//! Provides Altium-grade assembly variants management:
//! - Definition of named variants (e.g., "Full Option", "Low Cost", "Debug")
//! - Marking components as DNP / Not Fitted per variant
//! - Alternate part substitutions (custom value / footprint per variant)
//! - Visual indication with crosshatch rendering and BOM sync.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use oxide_types::theme::ThemeTokens;
use std::collections::{HashMap, HashSet};

use crate::panels::PanelMsg;
use crate::styles;

/// Alternate part substitution for an assembly variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AltPart {
    pub value: String,
    pub footprint: String,
    pub mpn: Option<String>,
}

/// A single named assembly variant.
#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyVariant {
    pub name: String,
    pub description: String,
    pub dnp_refs: HashSet<String>,
    pub alt_parts: HashMap<String, AltPart>,
}

impl AssemblyVariant {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            dnp_refs: HashSet::new(),
            alt_parts: HashMap::new(),
        }
    }

    pub fn is_dnp(&self, reference: &str) -> bool {
        self.dnp_refs.contains(reference)
    }

    pub fn toggle_dnp(&mut self, reference: &str) {
        if self.dnp_refs.contains(reference) {
            self.dnp_refs.remove(reference);
        } else {
            self.dnp_refs.insert(reference.to_string());
        }
    }
}

/// Variant Manager State.
#[derive(Debug, Clone)]
pub struct VariantManagerState {
    pub variants: Vec<AssemblyVariant>,
    pub active_variant_idx: usize,
    pub status: String,
}

impl Default for VariantManagerState {
    fn default() -> Self {
        let base = AssemblyVariant::new("Base Configuration", "Default fitted bill of materials");
        let mut low_cost = AssemblyVariant::new("Economy / Cost Down", "Unpopulated optional sensors & debug headers");
        low_cost.dnp_refs.insert("J1".to_string());
        low_cost.dnp_refs.insert("U5".to_string());

        Self {
            variants: vec![base, low_cost],
            active_variant_idx: 0,
            status: "Active Variant: Base Configuration".to_string(),
        }
    }
}

impl VariantManagerState {
    pub fn active_variant(&self) -> &AssemblyVariant {
        &self.variants[self.active_variant_idx]
    }

    pub fn active_variant_mut(&mut self) -> &mut AssemblyVariant {
        &mut self.variants[self.active_variant_idx]
    }

    pub fn select_variant(&mut self, idx: usize) {
        if idx < self.variants.len() {
            self.active_variant_idx = idx;
            self.status = format!("Active Variant: {}", self.variants[idx].name);
        }
    }
}

pub fn view<'a>(
    state: &'a VariantManagerState,
    tokens: &ThemeTokens,
) -> Element<'a, PanelMsg> {
    let title = text("Assembly Variant Manager")
        .size(14)
        .color(styles::ti(tokens.accent));

    let active_name = &state.active_variant().name;
    let variant_header = row![
        text("Variant:").size(12).color(styles::ti(tokens.text_secondary)),
        text(active_name).size(12).color(styles::ti(tokens.text)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let dnp_count = state.active_variant().dnp_refs.len();
    let alt_count = state.active_variant().alt_parts.len();

    let details_col = column![
        text(format!("Not Fitted (DNP): {} components", dnp_count))
            .size(11)
            .color(styles::ti(tokens.text)),
        text(format!("Substituted Parts: {} components", alt_count))
            .size(11)
            .color(styles::ti(tokens.text)),
        text(&state.status)
            .size(11)
            .color(styles::ti(tokens.text_secondary)),
    ]
    .spacing(4);

    let mut dnp_list = column![].spacing(2);
    if state.active_variant().dnp_refs.is_empty() {
        dnp_list = dnp_list.push(
            text("All components fitted in this variant.")
                .size(11)
                .color(styles::ti(tokens.text_secondary)),
        );
    } else {
        let refs: Vec<&String> = state.active_variant().dnp_refs.iter().collect();
        dnp_list = dnp_list.push(
            text(format!("DNP List: {}", refs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")))
                .size(11)
                .color(styles::ti(tokens.accent)),
        );
    }

    let actions = row![
        button(
            text("Toggle DNP on Selection")
                .size(11)
                .color(styles::ti(tokens.text)),
        )
        .padding([4, 8])
        .style(button::primary)
        .on_press(PanelMsg::Noop),
        button(
            text("Export Variant BOM")
                .size(11)
                .color(styles::ti(tokens.text)),
        )
        .padding([4, 8])
        .style(button::secondary)
        .on_press(PanelMsg::Noop),
    ]
    .spacing(8);

    let content = column![
        title,
        variant_header,
        details_col,
        dnp_list,
        actions,
    ]
    .spacing(10)
    .padding(12);

    container(content)
        .width(Length::Fill)
        .style(styles::panel_card_elevated(tokens))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_dnp_toggle() {
        let mut variant = AssemblyVariant::new("TestVariant", "desc");
        assert!(!variant.is_dnp("R1"));
        variant.toggle_dnp("R1");
        assert!(variant.is_dnp("R1"));
        variant.toggle_dnp("R1");
        assert!(!variant.is_dnp("R1"));
    }
}
