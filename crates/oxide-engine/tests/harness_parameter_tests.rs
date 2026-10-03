use oxide_engine::harness::{HarnessConnector, HarnessDefinition, HarnessManager};
use oxide_engine::parameter_manager::{ParameterManager, ParameterUpdate};
use oxide_types::schematic::{Point, SchematicSheet, Symbol};

#[test]
fn test_wire_harness_connector_and_signal_bundle_validation() {
    let mut hm = HarnessManager::new();

    // 1. Define an I2S Audio Harness Type
    hm.register_definition(HarnessDefinition {
        harness_type: "I2S_AUDIO".to_string(),
        member_signals: vec![
            "BCLK".to_string(),
            "LRCLK".to_string(),
            "SDIN".to_string(),
            "SDOUT".to_string(),
        ],
    });

    // 2. Connector 1: Complete connector with all 4 signals
    let mut conn1 = HarnessConnector::new("HC1", "I2S_AUDIO", Point::new(100.0, 50.0));
    conn1.add_entry("BCLK", "AUDIO_BCLK");
    conn1.add_entry("LRCLK", "AUDIO_LRCLK");
    conn1.add_entry("SDIN", "AUDIO_SDIN");
    conn1.add_entry("SDOUT", "AUDIO_SDOUT");
    hm.add_connector(conn1);

    // Initial validation should pass with zero errors
    assert!(hm.validate_connectors().is_empty());

    // 3. Connector 2: Incomplete connector missing "SDOUT"
    let mut conn2 = HarnessConnector::new("HC2", "I2S_AUDIO", Point::new(200.0, 50.0));
    conn2.add_entry("BCLK", "CODEC_BCLK");
    conn2.add_entry("LRCLK", "CODEC_LRCLK");
    conn2.add_entry("SDIN", "CODEC_SDIN");
    hm.add_connector(conn2);

    let errors = hm.validate_connectors();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("missing required signal 'SDOUT'"));
}

use std::collections::HashMap;
use uuid::Uuid;

fn empty_test_sheet() -> SchematicSheet {
    SchematicSheet {
        uuid: Uuid::new_v4(),
        version: 0,
        generator: String::new(),
        generator_version: String::new(),
        paper_size: "A4".to_string(),
        root_sheet_page: "1".to_string(),
        symbols: Vec::new(),
        wires: Vec::new(),
        junctions: Vec::new(),
        labels: Vec::new(),
        child_sheets: Vec::new(),
        no_connects: Vec::new(),
        text_notes: Vec::new(),
        buses: Vec::new(),
        bus_entries: Vec::new(),
        drawings: Vec::new(),
        no_erc_directives: Vec::new(),
        title_block: HashMap::new(),
        lib_symbols: HashMap::new(),
    }
}

#[test]
fn test_parameter_manager_batch_symbol_editing() {
    let mut sheet = empty_test_sheet();

    let mut sym1 = Symbol::empty();
    sym1.reference = "U1".to_string();
    sym1.value = "STM32F401".to_string();
    sym1.footprint = "LQFP-64".to_string();
    sheet.symbols.push(sym1);

    let mut sym2 = Symbol::empty();
    sym2.reference = "R1".to_string();
    sym2.value = "10k".to_string();
    sym2.footprint = "0805".to_string();
    sheet.symbols.push(sym2);

    // 1. Extract table
    let rows = ParameterManager::extract_parameters_from_sheet("Main", &sheet);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].designator, "U1");
    assert_eq!(rows[0].comment_or_value, "STM32F401");

    // 2. Apply batch updates
    let updates = vec![
        ParameterUpdate {
            designator: "R1".to_string(),
            parameter_name: "Value".to_string(),
            old_value: Some("10k".to_string()),
            new_value: "4.7k".to_string(),
        },
        ParameterUpdate {
            designator: "R1".to_string(),
            parameter_name: "Footprint".to_string(),
            old_value: Some("0805".to_string()),
            new_value: "0402".to_string(),
        },
        ParameterUpdate {
            designator: "U1".to_string(),
            parameter_name: "Manufacturer".to_string(),
            old_value: None,
            new_value: "STMicroelectronics".to_string(),
        },
    ];

    let applied = ParameterManager::apply_updates(&mut sheet, &updates);
    assert_eq!(applied, 3);

    // Verify modifications in sheet
    let r1 = sheet.symbols.iter().find(|s| s.reference == "R1").unwrap();
    assert_eq!(r1.value, "4.7k");
    assert_eq!(r1.footprint, "0402");

    let u1 = sheet.symbols.iter().find(|s| s.reference == "U1").unwrap();
    let mfr_prop = u1
        .custom_properties
        .iter()
        .find(|p| p.key == "Manufacturer")
        .unwrap();
    assert_eq!(mfr_prop.value, "STMicroelectronics");
}
