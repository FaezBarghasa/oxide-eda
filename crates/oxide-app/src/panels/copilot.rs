//! AI Copilot panel implementation for Oxide EDA.
//! Natural language circuit generation, part validation, and interactive design suggestions.

use iced::widget::{Column, Row, Space, button, container, row, scrollable, text, text_input};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::messages::PanelMsg;
use super::widgets::{section_title, separator};
use super::context::PanelContext;

#[derive(Debug, Clone)]
pub struct CopilotMessageEntry {
    pub is_user: bool,
    pub content: String,
    pub verified_parts: Vec<String>,
}

pub fn view_copilot<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new().spacing(4).padding(6).width(Length::Fill);

    col = col.push(
        row![
            section_title("AI Copilot (ProtoFlow Engine)", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("Clear Chat")
                    .size(11)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            )
            .padding([3, 10])
            .on_press(PanelMsg::ClearCopilotChat)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Zero-trust notice banner
    col = col.push(
        container(
            row![
                text("AI Proposes, Deterministic Rules Validate.")
                    .size(9)
                    .color(theme_ext::accent(&ctx.tokens)),
                Space::new().width(Length::Fill).height(Length::Shrink),
                text("Zero Hallucination Guarantee")
                    .size(9)
                    .color(theme_ext::text_secondary(&ctx.tokens)),
            ]
            .align_y(iced::Alignment::Center)
            .padding([3, 6]),
        )
        .style(crate::styles::panel_card(&ctx.tokens)),
    );

    // Chat history area
    let mut chat_col = Column::new().spacing(6);

    chat_col = chat_col.push(
        container(
            Column::new()
                .spacing(2)
                .push(text("Oxide Copilot:").size(10).color(theme_ext::accent(&ctx.tokens)))
                .push(
                    text("Hello! I can help you draft circuits, select verified distributor parts, place decoupling networks, or generate impedance-matched buses. How can I assist your design?")
                        .size(10)
                        .color(theme_ext::text_primary(&ctx.tokens)),
                ),
        )
        .padding(6)
        .style(crate::styles::panel_card(&ctx.tokens)),
    );

    col = col.push(scrollable(chat_col).height(Length::Fill));

    // Prompt input bar
    col = col.push(
        row![
            text_input(
                "Ask Copilot (e.g. 'Add 3.3V LDO regulator with 10uF decoupling')...",
                &ctx.component_filter,
            )
            .size(10)
            .on_input(PanelMsg::SetCopilotPromptInput)
            .padding([4, 8])
            .width(Length::Fill),
            button(
                text("Generate")
                    .size(10)
                    .color(theme_ext::text_primary(&ctx.tokens)),
            )
            .padding([4, 12])
            .on_press(PanelMsg::SubmitCopilotPrompt)
            .style(crate::styles::menu_item(&ctx.tokens)),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center),
    );

    col.into()
}
