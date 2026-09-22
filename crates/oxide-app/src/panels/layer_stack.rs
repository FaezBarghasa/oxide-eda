//! Multilayer Layer Stackup Manager panel implementation for Oxide EDA.

use iced::widget::{Column, Space, button, container, row, scrollable, text};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use super::context::PanelContext;

pub fn view_layer_stack<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill);

    col = col.push(
        row![
            section_title("Layer Stackup Manager", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Recalculate Impedance")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::RecalculateStackupImpedance)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Stackup summary
    col = col.push(
        row![
            text("4-Layer Standard FR-4 Controlled Impedance (1.6mm Total)")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text("Z₀ = 50.0 Ω | Zdiff = 100.0 Ω")
                .size(10)
                .color(theme_ext::text_primary(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    // Layer cross section list
    let mut layers_col = Column::new().spacing(4);

    let stackup_layers = [
        ("Top Layer (F.Cu)", "Signal", "35 µm (1 oz Cu)", "Z₀ = 50.6 Ω (w = 140µm)"),
        ("Prepreg (Core)", "Dielectric", "100 µm (FR-4 Standard)", "Dk = 4.4, Df = 0.02"),
        ("Inner Layer 1 (In1.Cu)", "Plane (GND)", "35 µm (1 oz Cu)", "Reference Plane"),
        ("Core", "Dielectric", "1200 µm (FR-4 Standard)", "Dk = 4.4, Df = 0.02"),
        ("Inner Layer 2 (In2.Cu)", "Plane (PWR)", "35 µm (1 oz Cu)", "Power Plane (3.3V)"),
        ("Prepreg (Core)", "Dielectric", "100 µm (FR-4 Standard)", "Dk = 4.4, Df = 0.02"),
        ("Bottom Layer (B.Cu)", "Signal", "35 µm (1 oz Cu)", "Z₀ = 50.6 Ω (w = 140µm)"),
    ];

    for (name, kind, thickness, note) in stackup_layers {
        layers_col = layers_col.push(
            container(
                row![
                    text(name).size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::FillPortion(3)),
                    text(kind).size(10).color(theme_ext::text_secondary(&ctx.tokens)).width(Length::FillPortion(2)),
                    text(thickness).size(10).color(theme_ext::text_secondary(&ctx.tokens)).width(Length::FillPortion(3)),
                    text(note).size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::FillPortion(4)),
                ]
                .align_y(iced::Alignment::Center)
                .padding([4, 6]),
            )
            .style(crate::styles::panel_card(&ctx.tokens)),
        );
    }

    col = col.push(scrollable(layers_col).height(Length::Fill));

    col.into()
}
