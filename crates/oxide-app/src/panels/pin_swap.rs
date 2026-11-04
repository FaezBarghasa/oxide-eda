//! Interactive Topological Pin & Part Gate Swapping Panel.
//!
//! Exposes Kuhn-Munkres optimal bipartite matching to untangle crossed rat-nest wires,
//! minimize routing Euclidean distance, respect FPGA/MCU I/O voltage bank boundaries,
//! and generate back-annotated ECO changes directly into the schematic.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use oxide_net::eco::EcoReport;
use oxide_net::swapping::{PinSwapAssignment, PinSwappingEngine, SwappablePin};
use oxide_types::theme::ThemeTokens;

use crate::panels::PanelMsg;
use crate::styles;

#[derive(Debug, Clone)]
pub struct PinSwapState {
    pub target_component: String,
    pub candidate_pins: Vec<SwappablePin>,
    pub proposed_swaps: Vec<PinSwapAssignment>,
    pub eco_report: Option<EcoReport>,
    pub length_reduction_pct: f64,
    pub status: String,
}

impl Default for PinSwapState {
    fn default() -> Self {
        Self {
            target_component: "U1".to_string(),
            candidate_pins: Vec::new(),
            proposed_swaps: Vec::new(),
            eco_report: None,
            length_reduction_pct: 0.0,
            status: "No active component selected".to_string(),
        }
    }
}

impl PinSwapState {
    /// Loads swappable pins for a component and computes optimal Kuhn-Munkres matching.
    pub fn run_optimization(
        &mut self,
        component_ref: &str,
        pins: &[SwappablePin],
        target_positions: &[[f64; 2]],
    ) {
        self.target_component = component_ref.to_string();
        self.candidate_pins = pins.to_vec();

        if pins.is_empty() || target_positions.is_empty() {
            self.status = "Component has no swappable pins or connected targets".to_string();
            self.proposed_swaps.clear();
            self.eco_report = None;
            return;
        }

        let (swaps, eco) =
            PinSwappingEngine::optimize_pin_swaps(component_ref, pins, target_positions);

        self.proposed_swaps = swaps;
        self.eco_report = Some(eco);
        self.status = format!(
            "Optimized: {} pin swaps proposed with bank voltage isolation preserved",
            self.proposed_swaps.len()
        );
    }
}

pub fn view<'a>(state: &'a PinSwapState, tokens: &ThemeTokens) -> Element<'a, PanelMsg> {
    let title = text(format!("Pin / Gate Swap: {}", state.target_component))
        .size(14)
        .color(styles::ti(tokens.accent));

    let status = text(&state.status)
        .size(11)
        .color(styles::ti(tokens.text_secondary));

    let mut swap_rows = column![].spacing(4);
    if state.proposed_swaps.is_empty() {
        swap_rows = swap_rows.push(
            text("No swaps proposed. Select an IC and run optimizer.")
                .size(11)
                .color(styles::ti(tokens.text_secondary)),
        );
    } else {
        for swap in &state.proposed_swaps {
            let row_el = row![
                text(format!("Pin {} -> Pin {}", swap.original_pin, swap.new_pin))
                    .size(11)
                    .color(styles::ti(tokens.text)),
                text(format!("Net: {}", swap.net_name))
                    .size(11)
                    .color(styles::ti(tokens.accent)),
            ]
            .spacing(12)
            .align_y(Alignment::Center);
            swap_rows = swap_rows.push(row_el);
        }
    }

    let actions = row![
        button(
            text("Run Optimizer")
                .size(11)
                .color(styles::ti(tokens.text)),
        )
        .padding([4, 8])
        .style(button::primary)
        .on_press(PanelMsg::RunPinSwapOptimization),
        button(
            text("Apply ECO to Schematic")
                .size(11)
                .color(styles::ti(tokens.text)),
        )
        .padding([4, 8])
        .style(button::secondary)
        .on_press(PanelMsg::ApplyPinSwapEco),
    ]
    .spacing(8);

    let content = column![title, status, swap_rows, actions,]
        .spacing(10)
        .padding(12);

    container(content)
        .width(Length::Fill)
        .style(styles::panel_card_elevated(tokens))
        .into()
}
