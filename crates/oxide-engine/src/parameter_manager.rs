//! Global Parameter Manager for multi-sheet schematic symbol properties.
//!
//! Provides Altium Designer Parameter Manager parity:
//! - Global multi-sheet component parameter inspection and batch editing.
//! - Parameter synchronization, diff generation, and batch modification.

use oxide_types::schematic::SchematicSheet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single parameter entry for a component symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolParameter {
    pub name: String,
    pub value: String,
    pub visible: bool,
    pub read_only: bool,
}

/// A row in the global Parameter Manager table representing one component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterTableRow {
    pub sheet_name: String,
    pub designator: String,
    pub comment_or_value: String,
    pub footprint: String,
    pub library_ref: String,
    pub custom_parameters: HashMap<String, String>,
}

/// Parameter update instruction for batch application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterUpdate {
    pub designator: String,
    pub parameter_name: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

/// Parameter Manager coordinating project-wide symbol parameter editing.
#[derive(Debug, Default, Clone)]
pub struct ParameterManager;

impl ParameterManager {
    /// Extract a tabular view of all components and their parameters from a schematic sheet.
    pub fn extract_parameters_from_sheet(
        sheet_name: &str,
        sheet: &SchematicSheet,
    ) -> Vec<ParameterTableRow> {
        let mut rows = Vec::new();

        for sym in &sheet.symbols {
            let mut custom_params = sym.fields.clone();
            for prop in &sym.custom_properties {
                custom_params.insert(prop.key.clone(), prop.value.clone());
            }

            rows.push(ParameterTableRow {
                sheet_name: sheet_name.to_string(),
                designator: sym.reference.clone(),
                comment_or_value: sym.value.clone(),
                footprint: sym.footprint.clone(),
                library_ref: sym.lib_id.clone(),
                custom_parameters: custom_params,
            });
        }

        rows
    }

    /// Apply batch parameter updates to a schematic sheet.
    pub fn apply_updates(sheet: &mut SchematicSheet, updates: &[ParameterUpdate]) -> usize {
        let mut applied_count = 0;

        for update in updates {
            for sym in &mut sheet.symbols {
                if sym.reference == update.designator {
                    match update.parameter_name.as_str() {
                        "Value" | "Comment" => {
                            sym.value = update.new_value.clone();
                            applied_count += 1;
                        }
                        "Footprint" => {
                            sym.footprint = update.new_value.clone();
                            applied_count += 1;
                        }
                        "Reference" => {
                            sym.reference = update.new_value.clone();
                            applied_count += 1;
                        }
                        custom => {
                            sym.fields
                                .insert(custom.to_string(), update.new_value.clone());
                            if let Some(prop) =
                                sym.custom_properties.iter_mut().find(|p| p.key == custom)
                            {
                                prop.value = update.new_value.clone();
                            } else {
                                sym.custom_properties.push(
                                    oxide_types::property::SchematicProperty {
                                        key: custom.to_string(),
                                        value: update.new_value.clone(),
                                        id: None,
                                        text: None,
                                        show_name: Some(true),
                                        do_not_autoplace: None,
                                        variant_overrides: Default::default(),
                                    },
                                );
                            }
                            applied_count += 1;
                        }
                    }
                }
            }
        }

        applied_count
    }
}
