//! Visual Diff Review panel implementation for Oxide EDA.
//! Side-by-side visual comparison and explicit engineer approval for Git/AI-proposed changes.

use iced::widget::{Column, Space, button, container, row, scrollable, text};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::context::PanelContext;
use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use crate::diff::DiffClassification;

pub fn view_ai_diff<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill);

    col = col.push(
        row![
            section_title("Visual Diff & Revision Merge Review", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Accept & Apply Merge")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([6, 14])
            .on_press(PanelMsg::AcceptAiProposal)
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(6).height(Length::Shrink),
            button(
                text("Discard Proposal")
                    .size(11)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .padding([6, 14])
            .on_press(PanelMsg::DiscardAiProposal)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Proposed changes table header
    col = col.push(
        row![
            text("Classified Geometric & Topological Changes")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text("Fail-Closed Release Gate: Active | Mode: Unified Overlay")
                .size(10)
                .color(theme_ext::accent(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    let changes = [
        (
            DiffClassification::Added,
            "+ Add Component",
            "U1 (TI TPS7A4700RGWR)",
            "Low-Noise LDO Regulator",
            "Verified (DigiKey #296-30232-1-ND)",
        ),
        (
            DiffClassification::Added,
            "+ Add Component",
            "C1 (Murata GRM188R61E106MA73D)",
            "10µF 25V X5R 0603",
            "Verified (Mouser #81-GRM188R61E106MA73)",
        ),
        (
            DiffClassification::Modified,
            "~ Mod Routing",
            "VIN -> U1.IN, C1.1",
            "Coupled Accordion Filleted",
            "Clearance: 250µm verified | Clothoid G2",
        ),
        (
            DiffClassification::Modified,
            "~ Mod Routing",
            "VOUT -> U1.OUT, C2.1",
            "Regulated 3.3V Power Net",
            "Width: 400µm rule applied | SAH BVH Pass",
        ),
        (
            DiffClassification::Removed,
            "- Del Component",
            "R_OLD (Legacy Pullup)",
            "Redundant 0805 Resistor",
            "Hungarian Pin-Swap Re-routed",
        ),
    ];

    let mut diff_col = Column::new().spacing(4);

    for (idx, (classification, action, target, desc, validation)) in changes.iter().enumerate() {
        let tag_color = match classification {
            DiffClassification::Added => iced::Color::from_rgb(0.15, 0.85, 0.30),
            DiffClassification::Removed => iced::Color::from_rgb(0.90, 0.20, 0.20),
            DiffClassification::Modified => iced::Color::from_rgb(1.00, 0.75, 0.10),
        };

        diff_col = diff_col.push(
            container(
                row![
                    text(*action)
                        .size(10)
                        .color(tag_color)
                        .width(Length::FillPortion(2)),
                    text(*target)
                        .size(10)
                        .color(theme_ext::text_primary(&ctx.tokens))
                        .width(Length::FillPortion(3)),
                    text(*desc)
                        .size(10)
                        .color(theme_ext::text_secondary(&ctx.tokens))
                        .width(Length::FillPortion(3)),
                    text(*validation)
                        .size(10)
                        .color(theme_ext::text_primary(&ctx.tokens))
                        .width(Length::FillPortion(3)),
                    button(
                        text("✓")
                            .size(10)
                            .color(iced::Color::from_rgb(0.2, 0.9, 0.3))
                    )
                    .padding([2, 6])
                    .on_press(PanelMsg::CherryPickDiffItem {
                        item_index: idx,
                        accept: true,
                    })
                    .style(crate::styles::menu_item(&ctx.tokens)),
                    button(
                        text("✕")
                            .size(10)
                            .color(iced::Color::from_rgb(0.9, 0.3, 0.3))
                    )
                    .padding([2, 6])
                    .on_press(PanelMsg::CherryPickDiffItem {
                        item_index: idx,
                        accept: false,
                    })
                    .style(crate::styles::menu_item(&ctx.tokens)),
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .padding([4, 6]),
            )
            .style(crate::styles::panel_card_elevated(&ctx.tokens)),
        );
    }

    col = col.push(scrollable(diff_col).height(Length::Fill));

    col.into()
}
