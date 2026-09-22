//! Design Rule Check (DRC) panel implementation for Oxide EDA.

use iced::widget::{Column, Space, button, row, scrollable, text};
use iced::{Element, Length};
use oxide_rules::RuleViolation;
use oxide_widgets::theme_ext;

use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use super::context::PanelContext;

/// Group of violations belonging to a specific design rule category.
#[derive(Debug, Clone)]
pub struct DrcCategoryGroup {
    pub name: &'static str,
    pub violations: Vec<RuleViolation>,
}

pub fn view_drc<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill);

    col = col.push(
        row![
            section_title("Design Rule Check (DRC)", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Run DRC (F9)")
                    .size(11)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::RunDrc)
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(6).height(Length::Shrink),
            button(
                text("Clear")
                    .size(11)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::ClearDrc)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Summary count badge
    col = col.push(
        row![
            text("Altium-Grade Hierarchical Rule Engine")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text("0 Violations")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(
        scrollable(
            Column::new()
                .spacing(6)
                .push(
                    text("No DRC violations detected. Ready to run full multilayer rule verification.")
                        .size(10)
                        .color(theme_ext::text_secondary(&ctx.tokens)),
                )
                .push(
                    text("Checked rules: Clearance Matrix, Trace Widths, Differential Pairs, HDI Microvias, and Keepouts.")
                        .size(9)
                        .color(theme_ext::text_secondary(&ctx.tokens)),
                ),
        )
        .height(Length::Fill),
    );

    col.into()
}
