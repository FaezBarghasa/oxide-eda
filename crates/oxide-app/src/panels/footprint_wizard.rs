//! IPC-7351 Footprint Generator Wizard.
//!
//! Provides interactive generation of standardized footprints for SOIC, QFP, BGA, and Chip components
//! based on package dimensions and IPC-7351C density levels (A = Most, B = Nominal, C = Least).

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use oxide_bake::ipc7351::{DensityLevel, Ipc7351Generator};
use oxide_library::harvester::PackageDimensions;
use oxide_library::primitive::footprint::Footprint;
use oxide_types::theme::ThemeTokens;

use crate::panels::PanelMsg;
use crate::styles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardPackageType {
    Chip,
    Soic,
    Qfp,
    Bga,
}

impl WizardPackageType {
    pub const ALL: [Self; 4] = [Self::Chip, Self::Soic, Self::Qfp, Self::Bga];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Chip => "Chip (R/C)",
            Self::Soic => "SOIC / SOP",
            Self::Qfp => "QFP / QFN",
            Self::Bga => "BGA Array",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FootprintWizardState {
    pub package_type: WizardPackageType,
    pub density: DensityLevel,
    pub pin_count: u32,
    pub pitch_mm: f64,
    pub body_length_mm: f64,
    pub body_width_mm: f64,
    pub lead_length_mm: f64,
    pub lead_width_mm: f64,
    pub name: String,
    pub last_generated: Option<Footprint>,
    pub status: String,
}

impl Default for FootprintWizardState {
    fn default() -> Self {
        Self {
            package_type: WizardPackageType::Soic,
            density: DensityLevel::Nominal,
            pin_count: 8,
            pitch_mm: 1.27,
            body_length_mm: 4.9,
            body_width_mm: 3.9,
            lead_length_mm: 0.6,
            lead_width_mm: 0.4,
            name: "SOIC-8_IPC".to_string(),
            last_generated: None,
            status: "Ready".to_string(),
        }
    }
}

impl FootprintWizardState {
    pub fn generate(&mut self) -> Result<Footprint, String> {
        let mut dims = match self.package_type {
            WizardPackageType::Chip => {
                PackageDimensions::standard_chip(self.body_length_mm, self.body_width_mm, 0.8)
            }
            WizardPackageType::Soic => {
                PackageDimensions::standard_soic(self.pin_count as usize)
            }
            WizardPackageType::Qfp => {
                PackageDimensions::standard_qfn(self.pin_count as usize, self.body_width_mm, self.pitch_mm, None)
            }
            WizardPackageType::Bga => {
                let mut bga = PackageDimensions::standard_qfn(self.pin_count as usize, self.body_width_mm, self.pitch_mm, None);
                bga.package_class = "BGA".to_string();
                bga
            }
        };

        dims.body_length_mm = [
            self.body_length_mm * 0.95,
            self.body_length_mm,
            self.body_length_mm * 1.05,
        ];
        dims.body_width_mm = [
            self.body_width_mm * 0.95,
            self.body_width_mm,
            self.body_width_mm * 1.05,
        ];
        dims.lead_length_mm = [
            self.lead_length_mm * 0.9,
            self.lead_length_mm,
            self.lead_length_mm * 1.1,
        ];
        dims.lead_width_mm = [
            self.lead_width_mm * 0.9,
            self.lead_width_mm,
            self.lead_width_mm * 1.1,
        ];
        dims.lead_pitch_mm = self.pitch_mm;

        let fp = Ipc7351Generator::generate_footprint(&self.name, &dims, self.density);
        self.status = format!(
            "Generated '{}' with {} pads",
            fp.name,
            fp.pads.len()
        );
        self.last_generated = Some(fp.clone());
        Ok(fp)
    }
}

pub fn view<'a>(
    state: &'a FootprintWizardState,
    tokens: &ThemeTokens,
) -> Element<'a, PanelMsg> {
    let title = text("IPC-7351 Footprint Wizard")
        .size(14)
        .color(styles::ti(tokens.accent));

    let pkg_type_row = row![
        text("Type:").size(12).color(styles::ti(tokens.text_secondary)),
        text(state.package_type.label())
            .size(12)
            .color(styles::ti(tokens.text)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let density_label = match state.density {
        DensityLevel::Most => "A (Most / Robust)",
        DensityLevel::Nominal => "B (Nominal / General)",
        DensityLevel::Least => "C (Least / High Density)",
    };

    let density_row = row![
        text("Density:").size(12).color(styles::ti(tokens.text_secondary)),
        text(density_label)
            .size(12)
            .color(styles::ti(tokens.text)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let stats_col = column![
        text(format!("Pins: {}", state.pin_count)).size(11).color(styles::ti(tokens.text)),
        text(format!("Pitch: {:.2} mm", state.pitch_mm)).size(11).color(styles::ti(tokens.text)),
        text(format!("Body: {:.2} x {:.2} mm", state.body_width_mm, state.body_length_mm))
            .size(11)
            .color(styles::ti(tokens.text)),
    ]
    .spacing(4);

    let status_text = text(&state.status)
        .size(11)
        .color(styles::ti(tokens.text_secondary));

    let generate_btn = button(
        text("Synthesize Footprint")
            .size(12)
            .color(styles::ti(tokens.text)),
    )
    .padding([6, 12])
    .style(button::primary)
    .on_press(PanelMsg::GenerateIpcFootprint);

    let content = column![
        title,
        pkg_type_row,
        density_row,
        stats_col,
        status_text,
        generate_btn,
    ]
    .spacing(12)
    .padding(12);

    container(content)
        .width(Length::Fill)
        .style(styles::panel_card_elevated(tokens))
        .into()
}
