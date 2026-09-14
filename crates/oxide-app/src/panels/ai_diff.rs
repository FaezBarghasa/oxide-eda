//! Visual Diff Review panel implementation for Oxide EDA.
//! Side-by-side visual comparison and explicit engineer approval for AI-proposed changes.

use iced::widget::{Column, Row, Space, button, container, row, scrollable, text};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use super::context::PanelContext;

pub fn view_ai_diff<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill);

    col = col.push(
        row![
            section_title("Visual Diff Review: AI Proposal", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Accept & Apply Proposal")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::AcceptAiProposal)
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(6).height(Length::Shrink),
            button(
                text("Discard Proposal")
                    .size(11)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::DiscardAiProposal)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Proposed changes table
    col = col.push(
        row![
            text("Proposed Circuit Modifications")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text("3 Components | 5 Nets | 100% Validated Parts")
                .size(10)
                .color(theme_ext::accent(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    let changes = [
        ("+ Add Component", "U1 (TI TPS7A4700RGWR)", "Low-Noise LDO Regulator", "Verified (DigiKey #296-30232-1-ND)"),
        ("+ Add Component", "C1 (Murata GRM188R61E106MA73D)", "10µF 25V X5R 0603", "Verified (Mouser #81-GRM188R61E106MA73)"),
        ("+ Add Component", "C2 (Murata GRM188R61E106MA73D)", "10µF 25V X5R 0603", "Verified (Mouser #81-GRM188R61E106MA73)"),
        ("+ Add Connection", "VIN -> U1.IN, C1.1", "Power Input Net", "Clearance: 250µm rule verified"),
        ("+ Add Connection", "VOUT -> U1.OUT, C2.1", "Regulated 3.3V Net", "Width: 400µm rule applied"),
    ];

    let mut diff_col = Column::new().spacing(4);

    for (action, target, desc, validation) in changes {
        diff_col = diff_col.push(
            container(
                row![
                    text(action).size(10).color(theme_ext::accent(&ctx.tokens)).width(Length::FillPortion(2)),
                    text(target).size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::FillPortion(3)),
                    text(desc).size(10).color(theme_ext::text_secondary(&ctx.tokens)).width(Length::FillPortion(4)),
                    text(validation).size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::FillPortion(4)),
                ]
                .align_y(iced::Alignment::Center)
                .padding([4, 6]),
            )
            .style(crate::styles::panel_card(&ctx.tokens)),
        );
    }

    col = col.push(scrollable(diff_col).height(Length::Fill));

    col.into()
}
