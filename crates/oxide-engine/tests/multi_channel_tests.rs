use oxide_engine::multi_channel::parse_and_expand_repeat;
use oxide_types::schematic::{ChildSheet, FillType, Point, SchematicSheet, Symbol};
use uuid::Uuid;
use std::collections::HashMap;

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
fn test_multi_channel_repeat_expansion() {
    let mut child_content = empty_test_sheet();
    let mut sym = Symbol::empty();
    sym.lib_id = "Device:R".to_string();
    sym.reference = "R1".to_string();
    sym.value = "10k".to_string();
    sym.footprint = "Resistor_SMD:R_0603".to_string();
    sym.position = Point::new(10.0, 10.0);
    child_content.symbols.push(sym);

    let child_sheet = ChildSheet {
        uuid: Uuid::new_v4(),
        name: "AudioChannel".to_string(),
        filename: "audio.snxsch".to_string(),
        position: Point::new(0.0, 0.0),
        size: (30.0, 30.0),
        stroke_width: 0.12,
        fill: FillType::None,
        stroke_color: None,
        fill_color: None,
        fields_autoplaced: false,
        pins: Vec::new(),
        instances: Vec::new(),
    };

    let instances = parse_and_expand_repeat("Repeat(Audio, 1, 4)", &child_sheet, &child_content);
    assert_eq!(instances.len(), 4);
    assert_eq!(instances[0].sheet_name, "Audio_CH1");
    assert_eq!(instances[0].designator_map.get("R1"), Some(&"R1_CH1".to_string()));
    assert_eq!(instances[3].sheet_name, "Audio_CH4");
    assert_eq!(instances[3].designator_map.get("R1"), Some(&"R1_CH4".to_string()));
}
