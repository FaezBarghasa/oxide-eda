//! Design Rule Check (DRC) panel implementation for Oxide EDA.

use iced::widget::{Column, Space, button, row, scrollable, text};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::context::PanelContext;
use super::messages::PanelMsg;
use super::widgets::{section_title, separator};

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

    let count = ctx.drc_violations.len();
    let count_label = if count == 1 {
        "1 Violation".to_string()
    } else {
        format!("{} Violations", count)
    };

    // Summary count badge
    col = col.push(
        row![
            text("Altium-Grade Hierarchical Rule Engine")
                .size(10)
                .color(theme_ext::text_secondary(&ctx.tokens)),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text(count_label).size(10).color(if count > 0 {
                iced::Color::from_rgb(0.95, 0.35, 0.35)
            } else {
                theme_ext::text_secondary(&ctx.tokens)
            }),
        ]
        .align_y(iced::Alignment::Center),
    );

    let mut body_col = Column::new().spacing(6);

    if ctx.drc_violations.is_empty() {
        body_col = body_col
            .push(
                text("No DRC violations detected. Click 'Run DRC (F9)' to evaluate all multilayer geometric and electrical constraints.")
                    .size(10)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .push(
                text("Rules: Clearance Matrix, Trace Widths, Differential Pairs, Annular Rings, HDI Microvias, and Keepouts.")
                    .size(9)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            );
    } else {
        for (i, v) in ctx.drc_violations.iter().enumerate() {
            let item = Column::new()
                .spacing(2)
                .padding([4, 6])
                .push(
                    row![
                        text(format!("{}. {:?}", i + 1, v.violation_type))
                            .size(10)
                            .color(iced::Color::from_rgb(0.95, 0.35, 0.35)),
                        Space::new().width(Length::Fill).height(Length::Shrink),
                        text(format!(
                            "Req: {} | Act: {}",
                            v.required_value, v.actual_value
                        ))
                        .size(9)
                        .color(theme_ext::text_secondary(&ctx.tokens)),
                    ]
                    .align_y(iced::Alignment::Center),
                )
                .push(
                    text(&v.message)
                        .size(9)
                        .color(theme_ext::text_primary(&ctx.tokens)),
                );
            body_col = body_col.push(item);
        }
    }

    col = col.push(scrollable(body_col).height(Length::Fill));
    col.into()
}
