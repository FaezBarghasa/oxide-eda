//! Telecommunications & RF Analysis Panel for Oxide EDA.
//! Renders S-Parameters & Smith Chart, Eye Diagrams with Jitter metrics, and Constellation plots with EVM.

pub mod canvas;

use iced::widget::{Column, Row, Space, button, canvas as iced_canvas, container, row, scrollable, text};
use iced::{Element, Length};
use oxide_rf::constellation::ConstellationDiagram;
use oxide_rf::eye_diagram::EyeDiagram;
use oxide_rf::s_param::Network2Port;
use oxide_widgets::theme_ext;

use super::context::PanelContext;
use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use self::canvas::{SmithChartCanvas, EyeDiagramCanvas, ConstellationCanvas};

/// Active sub-tab for the Telecom Panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TelecomTab {
    #[default]
    SmithChart,
    EyeDiagram,
    Constellation,
}

/// State for the Telecom & RF Panel.
#[derive(Debug, Clone, Default)]
pub struct TelecomPanelState {
    pub active_tab: TelecomTab,
    pub s_params: Option<Network2Port>,
    pub eye_diagram: Option<EyeDiagram>,
    pub constellation: Option<ConstellationDiagram>,
    pub carrier_freq_hz: f64,
    pub data_rate_bps: f64,
    pub modulation_scheme: String,
}

pub fn view_telecom<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new()
        .spacing(4)
        .padding(6)
        .width(Length::Fill)
        .height(Length::Fill);

    let state = &ctx.telecom_state;

    // Header Toolbar with Tab Selectors
    col = col.push(
        row![
            section_title("RF & Telecom Analysis", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Smith Chart (S-Params)")
                    .size(10)
                    .color(if state.active_tab == TelecomTab::SmithChart {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetTelecomTab(TelecomTab::SmithChart))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(4).height(Length::Shrink),
            button(
                text("Eye Diagram")
                    .size(10)
                    .color(if state.active_tab == TelecomTab::EyeDiagram {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetTelecomTab(TelecomTab::EyeDiagram))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(4).height(Length::Shrink),
            button(
                text("Constellation (I/Q)")
                    .size(10)
                    .color(if state.active_tab == TelecomTab::Constellation {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetTelecomTab(TelecomTab::Constellation))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(8).height(Length::Shrink),
            button(
                text("Run RF (F10)")
                    .size(10)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 8])
            .on_press(PanelMsg::RunRfSimulation)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Metric Summary Bar
    let metric_banner = match state.active_tab {
        TelecomTab::SmithChart => {
            let pts = state.s_params.as_ref().map(|s| s.frequencies.len()).unwrap_or(0);
            row![
                text(format!("Points: {} | Z0: 50.0 Ω", pts))
                    .size(10)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            ]
        }
        TelecomTab::EyeDiagram => {
            if let Some(eye) = &state.eye_diagram {
                row![
                    text(format!("Eye Height: {:.2} mV", eye.eye_height * 1e3))
                        .size(10)
                        .color(iced::Color::from_rgb(0.2, 0.8, 0.2)),
                    Space::new().width(12).height(Length::Shrink),
                    text(format!("Eye Width: {:.2} ps", eye.eye_width * 1e12))
                        .size(10)
                        .color(iced::Color::from_rgb(0.2, 0.6, 1.0)),
                    Space::new().width(12).height(Length::Shrink),
                    text(format!("Jitter RMS: {:.2} ps (P-P: {:.2} ps)", eye.jitter_rms * 1e12, eye.jitter_pp * 1e12))
                        .size(10)
                        .color(iced::Color::from_rgb(0.9, 0.7, 0.1)),
                ]
            } else {
                row![text("No Eye Diagram simulated").size(10).color(theme_ext::text_secondary(&ctx.tokens))]
            }
        }
        TelecomTab::Constellation => {
            if let Some(cons) = &state.constellation {
                row![
                    text(format!("Modulation: {}", cons.scheme.name()))
                        .size(10)
                        .color(theme_ext::accent(&ctx.tokens)),
                    Space::new().width(12).height(Length::Shrink),
                    text(format!("EVM RMS: {:.2}% (Peak: {:.2}%)", cons.evm_rms_percent, cons.evm_peak_percent))
                        .size(10)
                        .color(iced::Color::from_rgb(0.9, 0.4, 0.1)),
                    Space::new().width(12).height(Length::Shrink),
                    text(format!("SNR: {:.1} dB", cons.snr_db))
                        .size(10)
                        .color(iced::Color::from_rgb(0.2, 0.8, 0.2)),
                ]
            } else {
                row![text("No Constellation I/Q data").size(10).color(theme_ext::text_secondary(&ctx.tokens))]
            }
        }
    };

    col = col.push(
        container(metric_banner.align_y(iced::Alignment::Center).padding([2, 6]))
            .style(crate::styles::panel_card(&ctx.tokens)),
    );

    // Canvas Container
    let canvas_element: Element<'a, PanelMsg> = match state.active_tab {
        TelecomTab::SmithChart => iced_canvas(SmithChartCanvas {
            s_params: state.s_params.as_ref(),
            tokens: &ctx.tokens,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),

        TelecomTab::EyeDiagram => iced_canvas(EyeDiagramCanvas {
            eye_diagram: state.eye_diagram.as_ref(),
            tokens: &ctx.tokens,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),

        TelecomTab::Constellation => iced_canvas(ConstellationCanvas {
            constellation: state.constellation.as_ref(),
            tokens: &ctx.tokens,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    };

    col = col.push(
        container(canvas_element)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::styles::panel_card(&ctx.tokens)),
    );

    col.into()
}
