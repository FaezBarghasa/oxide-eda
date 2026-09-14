//! Waveform Viewer Panel implementation for Oxide EDA.
//! Displays analog & digital waveforms, signal list, cursor delta readouts, and simulation controls.

pub mod canvas;

use iced::widget::{Column, Row, Space, button, canvas as iced_canvas, container, row, scrollable, text};
use iced::{Element, Length};
use oxide_types::sim::WaveformDataset;
use oxide_widgets::theme_ext;

use super::context::PanelContext;
use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use self::canvas::WaveformCanvas;

/// State for the Waveform Panel.
#[derive(Debug, Clone, Default)]
pub struct WaveformPanelState {
    pub dataset: Option<WaveformDataset>,
    pub selected_traces: Vec<String>,
    pub cursor_a: Option<f64>,
    pub cursor_b: Option<f64>,
    pub simulation_running: bool,
    pub status_message: String,
}

pub fn view_waveform<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill).height(Length::Fill);

    let state = &ctx.waveform_state;

    // Header Toolbar
    col = col.push(
        row![
            section_title("Waveform Viewer (PSpice / Mixed-Signal)", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            if state.simulation_running {
                text("Simulating...")
                    .size(11)
                    .color(theme_ext::accent(&ctx.tokens))
            } else {
                text(&state.status_message)
                    .size(10)
                    .color(theme_ext::text_secondary(&ctx.tokens))
            },
            Space::new().width(10).height(Length::Shrink),
            button(
                text("Run (F9)")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::RunSimulation)
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(6).height(Length::Shrink),
            button(
                text("Setup...")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::OpenSimulationSetup)
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(6).height(Length::Shrink),
            button(
                text("Clear")
                    .size(11)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::ClearWaveforms)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Cursor Readout Banner
    if let (Some(ca), Some(cb)) = (state.cursor_a, state.cursor_b) {
        let delta = (cb - ca).abs();
        let freq = if delta > 1e-15 { format!("{:.3} kHz", 1.0 / (delta * 1e3)) } else { "---".to_string() };
        col = col.push(
            container(
                row![
                    text(format!("Cursor A: {:.3e} s", ca))
                        .size(10)
                        .color(iced::Color::from_rgb(0.9, 0.4, 0.1)),
                    Space::new().width(16).height(Length::Shrink),
                    text(format!("Cursor B: {:.3e} s", cb))
                        .size(10)
                        .color(iced::Color::from_rgb(0.1, 0.7, 0.9)),
                    Space::new().width(16).height(Length::Shrink),
                    text(format!("Δ: {:.3e} s (1/Δ: {})", delta, freq))
                        .size(10)
                        .color(theme_ext::accent(&ctx.tokens)),
                ]
                .align_y(iced::Alignment::Center)
                .padding([2, 6]),
            )
            .style(crate::styles::panel_card(&ctx.tokens)),
        );
    }

    // Main Content: Left Trace List + Right Canvas
    let mut trace_col = Column::new().spacing(4).width(Length::Fixed(140.0));
    trace_col = trace_col.push(
        text("Traces / Probes")
            .size(10)
            .color(theme_ext::text_secondary(&ctx.tokens)),
    );

    if let Some(ds) = &state.dataset {
        for trace in &ds.traces {
            let is_selected = state.selected_traces.is_empty() || state.selected_traces.contains(&trace.name);
            let name = trace.name.clone();
            trace_col = trace_col.push(
                button(
                    row![
                        text(if is_selected { "●" } else { "○" })
                            .size(10)
                            .color(if is_selected { theme_ext::accent(&ctx.tokens) } else { theme_ext::text_secondary(&ctx.tokens) }),
                        Space::new().width(4).height(Length::Shrink),
                        text(&trace.name)
                            .size(10)
                            .color(theme_ext::text_primary(&ctx.tokens)),
                    ]
                    .align_y(iced::Alignment::Center),
                )
                .padding([2, 4])
                .on_press(PanelMsg::ToggleWaveformTrace(name))
                .style(crate::styles::menu_item(&ctx.tokens)),
            );
        }
    } else {
        trace_col = trace_col.push(
            text("No active traces")
                .size(9)
                .color(theme_ext::text_secondary(&ctx.tokens)),
        );
    }

    let waveform_canvas = iced_canvas(WaveformCanvas {
        dataset: state.dataset.as_ref(),
        selected_traces: &state.selected_traces,
        cursor_a: state.cursor_a,
        cursor_b: state.cursor_b,
        tokens: &ctx.tokens,
    })
    .width(Length::Fill)
    .height(Length::Fill);

    let content_row = row![
        scrollable(trace_col).height(Length::Fill),
        Space::new().width(6).height(Length::Shrink),
        container(waveform_canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::styles::panel_card(&ctx.tokens)),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    col = col.push(content_row);
    col.into()
}
