//! Tools ▸ Altium Command Explorer modal — searchable browser over all 3,004
//! Altium Designer `.rcs` UI catalog entries (1,190 unique commands across 39 modules).

use iced::widget::{Space, button, column, container, row, scrollable, text, text_input};
use iced::{Background, Border, Element, Length};
use oxide_types::theme::{ThemeId, ThemeTokens};

use crate::app::view::dialogs::{
    MODAL_HEADER_HEIGHT, MODAL_HEADER_PADDING, MODAL_HEADER_TITLE_SIZE, close_x_button,
};
use crate::app::{Message, OverlayMsg};
use crate::styles;
use crate::ui_catalog::{CommandRegistry, UiCommand, map_altium_command};

const MODAL_W: f32 = 820.0;
const MODAL_H: f32 = 580.0;
const MAX_ROWS: usize = 80;

#[derive(Debug, Clone, Default)]
pub struct AltiumCatalogModalState {
    pub open: bool,
    pub query: String,
    pub namespace_filter: String,
    pub last_status: Option<String>,
}

const NAMESPACE_CHIPS: &[&str] = &[
    "",
    "PCB",
    "Sch",
    "WorkspaceManager",
    "CAMTASTIC",
    "@PLSCH",
    "Client",
    "LayerStackManager",
    "ConstraintsManager",
    "ADVSIM",
    "PCB3D",
];

pub fn view<'a>(
    tokens: &'a ThemeTokens,
    theme_id: ThemeId,
    state: &'a AltiumCatalogModalState,
    registry: &'a CommandRegistry,
) -> Element<'a, Message> {
    let text_primary = styles::ti(tokens.text);
    let text_secondary = styles::ti(tokens.text_secondary);
    let accent = styles::ti(tokens.accent);
    let border_c = styles::ti(tokens.border);
    let panel_bg = styles::ti(tokens.panel_bg);

    let header = container(
        row![
            text(format!(
                "Altium Command Explorer ({} commands / {} unique)",
                registry.catalog_count(),
                registry.unique_command_count()
            ))
            .size(MODAL_HEADER_TITLE_SIZE)
            .color(text_primary),
            Space::new().width(Length::Fill),
            close_x_button(
                Message::Overlay(OverlayMsg::CloseAltiumCatalog),
                theme_id,
                text_secondary,
            ),
        ]
        .align_y(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(MODAL_HEADER_HEIGHT)
    .padding(MODAL_HEADER_PADDING)
    .style(styles::modal_header_strip(tokens));

    let search_bar = text_input(
        "Filter 3,004 Altium commands (e.g. PCB:Autoroute, Sch:PlaceWire, Undo, Gerber)...",
        &state.query,
    )
    .on_input(|q| Message::Overlay(OverlayMsg::AltiumCatalogQuery(q)))
    .size(12)
    .width(Length::Fill);

    let mut chips_row = row![].spacing(4).wrap(true);
    for &ns in NAMESPACE_CHIPS {
        let label = if ns.is_empty() { "All" } else { ns };
        let active = state.namespace_filter == ns;
        let bg = if active { accent } else { panel_bg };
        let fg = if active { iced::Color::WHITE } else { text_primary };
        chips_row = chips_row.push(
            button(text(label).size(10).color(fg))
                .padding([3, 7])
                .on_press(Message::Overlay(OverlayMsg::AltiumCatalogNamespace(
                    ns.to_string(),
                )))
                .style(move |_, _| button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: fg,
                    border: Border {
                        color: border_c,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                }),
        );
    }

    let q_lower = state.query.trim().to_ascii_lowercase();
    let ns_filter = state.namespace_filter.as_str();
    let mut matches: Vec<(String, &UiCommand)> = Vec::new();
    let mut file_names = registry.catalog().file_names();
    file_names.sort();

    for file in file_names {
        for cmd in registry.catalog().commands_from(&file) {
            if !ns_filter.is_empty() && !cmd.namespace().eq_ignore_ascii_case(ns_filter) {
                continue;
            }
            if !q_lower.is_empty() {
                let hay_cmd = cmd.command.to_ascii_lowercase();
                let hay_cap = cmd.plain_caption().to_ascii_lowercase();
                let hay_desc = cmd.description.to_ascii_lowercase();
                if !hay_cmd.contains(&q_lower)
                    && !hay_cap.contains(&q_lower)
                    && !hay_desc.contains(&q_lower)
                {
                    continue;
                }
            }
            matches.push((file.clone(), cmd));
            if matches.len() >= MAX_ROWS {
                break;
            }
        }
        if matches.len() >= MAX_ROWS {
            break;
        }
    }

    let mut list_col = column![].spacing(3).width(Length::Fill);
    for (file, cmd) in matches {
        let is_wired = map_altium_command(&cmd.command).is_some();
        let badge_text = if is_wired { "LIVE" } else { "CATALOG" };
        let badge_color = if is_wired { accent } else { text_secondary };
        let cmd_name = cmd.command.clone();
        let caption = cmd.plain_caption();
        let shortcut = if cmd.shortcut1.is_empty() {
            "—".to_string()
        } else {
            cmd.shortcut1.clone()
        };

        let row_btn = button(
            row![
                text(badge_text)
                    .size(9)
                    .color(badge_color)
                    .width(Length::Fixed(52.0)),
                text(cmd.command.clone())
                    .size(11)
                    .color(text_primary)
                    .width(Length::FillPortion(3)),
                text(caption)
                    .size(11)
                    .color(text_secondary)
                    .width(Length::FillPortion(2)),
                text(shortcut)
                    .size(10)
                    .color(text_secondary)
                    .width(Length::Fixed(90.0)),
                text(file)
                    .size(10)
                    .color(text_secondary)
                    .width(Length::FillPortion(2)),
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
        )
        .padding([4, 8])
        .width(Length::Fill)
        .on_press(Message::Overlay(OverlayMsg::DispatchAltiumCommand(cmd_name)))
        .style(styles::menu_item(tokens));

        list_col = list_col.push(row_btn);
    }

    let status_line = text(
        state
            .last_status
            .as_deref()
            .unwrap_or("Click any command to dispatch it through CommandRegistry."),
    )
    .size(11)
    .color(text_secondary);

    let body = column![
        search_bar,
        chips_row,
        scrollable(list_col).height(Length::Fill),
        status_line,
    ]
    .spacing(8)
    .padding(12)
    .width(Length::Fill)
    .height(Length::Fill);

    let card = container(column![header, body].spacing(0))
        .width(Length::Fixed(MODAL_W))
        .height(Length::Fixed(MODAL_H))
        .clip(true)
        .style(styles::modal_card(tokens));

    container(card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
