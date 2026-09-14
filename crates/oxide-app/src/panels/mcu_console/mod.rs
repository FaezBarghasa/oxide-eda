//! MCU & Network Protocol Console Panel for Oxide EDA.
//! Renders Virtual UART Terminal, Live Embedded MQTT Inspector, Virtual Ethernet/Wi-Fi/BLE monitors, and Co-Sim controls.

use iced::widget::{Column, Row, Space, button, container, row, scrollable, text, text_input};
use iced::{Element, Length};
use oxide_widgets::theme_ext;

use super::context::PanelContext;
use super::messages::PanelMsg;
use super::widgets::{section_title, separator};

/// Active sub-tab for the MCU Console Panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum McuConsoleTab {
    #[default]
    UartTerminal,
    MqttInspector,
    NetworkStatus,
}

/// State for the MCU & Protocol Console Panel.
#[derive(Debug, Clone, Default)]
pub struct McuConsolePanelState {
    pub active_tab: McuConsoleTab,
    pub uart_output: Vec<String>,
    pub uart_input_buffer: String,
    pub mqtt_messages: Vec<(String, String)>, // (topic, payload)
    pub is_qemu_running: bool,
    pub gdb_port: u16,
    pub cosim_time_us: u64,
    pub eth_packet_count: usize,
    pub wifi_rssi_dbm: f64,
    pub ble_connected: bool,
}

pub fn view_mcu_console<'a>(ctx: &'a PanelContext) -> Element<'a, PanelMsg> {
    let mut col: Column<'a, PanelMsg> = Column::new()
        .spacing(4)
        .padding(6)
        .width(Length::Fill)
        .height(Length::Fill);

    let state = &ctx.mcu_console_state;

    // Header Toolbar
    col = col.push(
        row![
            section_title("MCU & Protocol Co-Sim Console", &ctx.tokens),
            Space::new().width(Length::Fill).height(Length::Shrink),
            button(
                text("UART Terminal")
                    .size(10)
                    .color(if state.active_tab == McuConsoleTab::UartTerminal {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetMcuConsoleTab(McuConsoleTab::UartTerminal))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(4).height(Length::Shrink),
            button(
                text("MQTT Broker")
                    .size(10)
                    .color(if state.active_tab == McuConsoleTab::MqttInspector {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetMcuConsoleTab(McuConsoleTab::MqttInspector))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(4).height(Length::Shrink),
            button(
                text("Net/RF Links")
                    .size(10)
                    .color(if state.active_tab == McuConsoleTab::NetworkStatus {
                        theme_ext::accent(&ctx.tokens)
                    } else {
                        theme_ext::text_primary(&ctx.tokens)
                    }),
            )
            .padding([3, 8])
            .on_press(PanelMsg::SetMcuConsoleTab(McuConsoleTab::NetworkStatus))
            .style(crate::styles::menu_item(&ctx.tokens)),
            Space::new().width(8).height(Length::Shrink),
            if state.is_qemu_running {
                button(
                    text("Stop Co-Sim")
                        .size(10)
                        .color(iced::Color::from_rgb(1.0, 0.3, 0.3)),
                )
                .padding([3, 8])
                .on_press(PanelMsg::StopCoSimulation)
                .style(crate::styles::menu_item(&ctx.tokens))
            } else {
                button(
                    text("Start Co-Sim (QEMU)")
                        .size(10)
                        .color(theme_ext::accent(&ctx.tokens)),
                )
                .padding([3, 8])
                .on_press(PanelMsg::RunCoSimulation)
                .style(crate::styles::menu_item(&ctx.tokens))
            },
        ]
        .align_y(iced::Alignment::Center),
    );

    col = col.push(separator(&ctx.tokens));

    // Status Banner
    let status_row = row![
        text(if state.is_qemu_running { "● QEMU Cortex-M Running" } else { "○ QEMU Idle" })
            .size(10)
            .color(if state.is_qemu_running { iced::Color::from_rgb(0.2, 0.8, 0.2) } else { theme_ext::text_secondary(&ctx.tokens) }),
        Space::new().width(12).height(Length::Shrink),
        text(format!("GDB: :{}", state.gdb_port))
            .size(10)
            .color(theme_ext::text_secondary(&ctx.tokens)),
        Space::new().width(12).height(Length::Shrink),
        text(format!("Sim Time: {} µs", state.cosim_time_us))
            .size(10)
            .color(theme_ext::accent(&ctx.tokens)),
        Space::new().width(12).height(Length::Shrink),
        text(format!("Eth Pkts: {}", state.eth_packet_count))
            .size(10)
            .color(theme_ext::text_secondary(&ctx.tokens)),
        Space::new().width(12).height(Length::Shrink),
        text(format!("Wi-Fi RSSI: {:.1} dBm", state.wifi_rssi_dbm))
            .size(10)
            .color(theme_ext::text_secondary(&ctx.tokens)),
    ]
    .align_y(iced::Alignment::Center)
    .padding([2, 6]);

    col = col.push(
        container(status_row)
            .style(crate::styles::panel_card(&ctx.tokens)),
    );

    // Tab Body
    let body_element: Element<'a, PanelMsg> = match state.active_tab {
        McuConsoleTab::UartTerminal => {
            let mut term_col = Column::new().spacing(2).padding(6).width(Length::Fill);
            if state.uart_output.is_empty() {
                term_col = term_col.push(
                    text("[UART Console Ready - Start Co-Sim to view MCU stdout]")
                        .size(10)
                        .color(theme_ext::text_secondary(&ctx.tokens)),
                );
            } else {
                for line in &state.uart_output {
                    term_col = term_col.push(
                        text(line)
                            .size(10)
                            .color(iced::Color::from_rgb(0.2, 0.9, 0.3)), // Terminal Green
                    );
                }
            }

            let input_bar = row![
                text_input("Send to UART...", &state.uart_input_buffer)
                    .size(11)
                    .padding(4)
                    .on_input(PanelMsg::SetUartInputBuffer)
                    .on_submit(PanelMsg::SubmitUartInput),
                Space::new().width(4).height(Length::Shrink),
                button(text("Send").size(10))
                    .padding([4, 8])
                    .on_press(PanelMsg::SubmitUartInput)
                    .style(crate::styles::menu_item(&ctx.tokens)),
            ]
            .align_y(iced::Alignment::Center);

            Column::new()
                .spacing(4)
                .push(scrollable(term_col).height(Length::Fill))
                .push(input_bar)
                .height(Length::Fill)
                .into()
        }

        McuConsoleTab::MqttInspector => {
            let mut mqtt_col = Column::new().spacing(4).padding(6).width(Length::Fill);
            mqtt_col = mqtt_col.push(
                row![
                    text("Topic").size(10).color(theme_ext::accent(&ctx.tokens)).width(Length::Fixed(160.0)),
                    text("Payload").size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::Fill),
                ]
            );
            mqtt_col = mqtt_col.push(separator(&ctx.tokens));

            if state.mqtt_messages.is_empty() {
                mqtt_col = mqtt_col.push(
                    text("No MQTT messages published yet (In-Process Broker active)").size(10).color(theme_ext::text_secondary(&ctx.tokens))
                );
            } else {
                for (topic, payload) in &state.mqtt_messages {
                    mqtt_col = mqtt_col.push(
                        row![
                            text(topic).size(10).color(iced::Color::from_rgb(0.2, 0.7, 1.0)).width(Length::Fixed(160.0)),
                            text(payload).size(10).color(theme_ext::text_primary(&ctx.tokens)).width(Length::Fill),
                        ]
                    );
                }
            }
            scrollable(mqtt_col).height(Length::Fill).into()
        }

        McuConsoleTab::NetworkStatus => {
            let net_col = Column::new()
                .spacing(8)
                .padding(8)
                .push(text("Virtual Network Interfaces:").size(11).color(theme_ext::accent(&ctx.tokens)))
                .push(text(format!("• 10/100M Virtual Ethernet Switch: {} packets routed (PCAP logging active)", state.eth_packet_count)).size(10))
                .push(text(format!("• 802.11b/g/n Wi-Fi AP: SSID 'Oxide-Sim-AP', RSSI: {:.1} dBm", state.wifi_rssi_dbm)).size(10))
                .push(text(format!("• BLE 5.0 Controller: {}", if state.ble_connected { "Connected (GATT server ready)" } else { "Advertising" })).size(10));
            scrollable(net_col).height(Length::Fill).into()
        }
    };

    col = col.push(
        container(body_element)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(crate::styles::panel_card(&ctx.tokens)),
    );

    col.into()
}
