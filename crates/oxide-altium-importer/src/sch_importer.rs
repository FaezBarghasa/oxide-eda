//! Altium Designer Schematic (.SchDoc) importer.

use std::collections::HashMap;
use uuid::Uuid;

use oxide_types::schematic::{
    Bus, BusEntry, ChildSheet, HAlign, Junction, Label, LabelType, Point, SchematicSheet, Symbol,
    VAlign, Wire,
};

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::record::{parse_record_stream, AltiumRecord};

/// Import an Altium `.SchDoc` file from raw binary bytes.
pub fn import_schdoc_bytes(bytes: &[u8]) -> Result<SchematicSheet, AltiumImportError> {
    let cfb = CfbContainer::parse(bytes)?;
    let stream_data = cfb.get_stream("FileHeader")?;
    let records = parse_record_stream(&stream_data)?;
    parse_schdoc_records(&records)
}

/// Convert parsed Altium schematic records into an Oxide [`SchematicSheet`].
pub fn parse_schdoc_records(records: &[AltiumRecord]) -> Result<SchematicSheet, AltiumImportError> {
    let mut sheet = SchematicSheet {
        uuid: Uuid::now_v7(),
        version: 1,
        generator: "Oxide Altium Importer".to_string(),
        generator_version: "1.0".to_string(),
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
    };

    // Keep track of parent components to link parameters/designators (OwnerIndex -> Symbol)
    let mut component_indices: HashMap<usize, usize> = HashMap::new();

    for (rec_idx, rec) in records.iter().enumerate() {
        let record_type = rec.get_i64("RECORD").unwrap_or(-1);

        match record_type {
            // RECORD=31: Sheet header & Title Block
            31 => {
                if let Some(title) = rec.get("TITLE") {
                    sheet.title_block.insert("title".to_string(), title.to_string());
                }
                if let Some(rev) = rec.get("REVISION") {
                    sheet.title_block.insert("rev".to_string(), rev.to_string());
                }
                if let Some(doc_num) = rec.get("DOCUMENTNUMBER") {
                    sheet.title_block.insert("doc_number".to_string(), doc_num.to_string());
                }
            }

            // RECORD=1: Component instance
            1 => {
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                let lib_ref = rec.get("LIBREFERENCE").unwrap_or("").to_string();
                let designator = rec.get("DESIGNATOR").unwrap_or("").to_string();
                let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);
                let mirrored = rec.get_bool("MIRRORED");

                let mut symbol = Symbol::empty();
                symbol.uuid = Uuid::now_v7();
                symbol.lib_id = lib_ref;
                symbol.reference = designator;
                symbol.position = Point::new(x, y);
                symbol.rotation = rotation;
                symbol.mirror_x = mirrored;

                let symbol_idx = sheet.symbols.len();
                sheet.symbols.push(symbol);
                component_indices.insert(rec_idx, symbol_idx);
            }

            // RECORD=34: Parameter / Designator property linked to Component
            34 => {
                let name = rec.get("NAME").unwrap_or("");
                let text = rec.get("TEXT").unwrap_or("");

                if let Some(owner_idx) = rec.get_i64("OWNERINDEX") {
                    if let Some(&sym_idx) = component_indices.get(&(owner_idx as usize)) {
                        if let Some(sym) = sheet.symbols.get_mut(sym_idx) {
                            if name.eq_ignore_ascii_case("Comment") || name.eq_ignore_ascii_case("Value") {
                                sym.value = text.to_string();
                            } else if name.eq_ignore_ascii_case("Footprint") {
                                sym.footprint = text.to_string();
                            } else if name.eq_ignore_ascii_case("Designator") {
                                if sym.reference.is_empty() {
                                    sym.reference = text.to_string();
                                }
                            } else {
                                sym.fields.insert(name.to_string(), text.to_string());
                            }
                        }
                    }
                }
            }

            // RECORD=27: Wire segment
            27 => {
                let count = rec.get_i64("LOCATIONCOUNT").unwrap_or(2) as usize;
                let mut points = Vec::new();

                if count >= 2 {
                    for i in 1..=count {
                        let x_key = format!("X{i}");
                        let y_key = format!("Y{i}");
                        if let (Some(x), Some(y)) = (rec.get_coord_mm(&x_key), rec.get_coord_mm(&y_key)) {
                            points.push(Point::new(x, y));
                        }
                    }
                }

                if points.len() < 2 {
                    let x1 = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                    let y1 = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                    let x2 = rec.get_coord_mm("CORNER.X").unwrap_or(x1 + 10.0);
                    let y2 = rec.get_coord_mm("CORNER.Y").unwrap_or(y1);
                    points = vec![Point::new(x1, y1), Point::new(x2, y2)];
                }

                for w in points.windows(2) {
                    sheet.wires.push(Wire {
                        uuid: Uuid::now_v7(),
                        start: w[0],
                        end: w[1],
                        stroke_width: 0.0,
                    });
                }
            }

            // RECORD=29: Junction
            29 => {
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                sheet.junctions.push(Junction {
                    uuid: Uuid::now_v7(),
                    position: Point::new(x, y),
                    diameter: 0.0,
                    minted: false,
                });
            }

            // RECORD=4: Net Label
            4 => {
                let text = rec.get("TEXT").unwrap_or("").to_string();
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);

                sheet.labels.push(Label {
                    uuid: Uuid::now_v7(),
                    text,
                    position: Point::new(x, y),
                    rotation,
                    label_type: LabelType::Net,
                    shape: String::new(),
                    font_size: 0.0,
                    justify: HAlign::Left,
                    justify_v: VAlign::Bottom,
                });
            }

            // RECORD=17: Power Port (mapped to Power label in Oxide)
            17 => {
                let text = rec.get("TEXT").unwrap_or("GND").to_string();
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);

                sheet.labels.push(Label {
                    uuid: Uuid::now_v7(),
                    text,
                    position: Point::new(x, y),
                    rotation,
                    label_type: LabelType::Power,
                    shape: String::new(),
                    font_size: 0.0,
                    justify: HAlign::Left,
                    justify_v: VAlign::Bottom,
                });
            }

            // RECORD=26: Bus
            26 => {
                let count = rec.get_i64("LOCATIONCOUNT").unwrap_or(2) as usize;
                let mut points = Vec::new();
                for i in 1..=count {
                    let x_key = format!("X{i}");
                    let y_key = format!("Y{i}");
                    if let (Some(x), Some(y)) = (rec.get_coord_mm(&x_key), rec.get_coord_mm(&y_key)) {
                        points.push(Point::new(x, y));
                    }
                }
                for w in points.windows(2) {
                    sheet.buses.push(Bus {
                        uuid: Uuid::now_v7(),
                        start: w[0],
                        end: w[1],
                    });
                }
            }

            // RECORD=25: Bus Entry
            25 => {
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                let cx = rec.get_coord_mm("CORNER.X").unwrap_or(x + 2.54);
                let cy = rec.get_coord_mm("CORNER.Y").unwrap_or(y + 2.54);

                sheet.bus_entries.push(BusEntry {
                    uuid: Uuid::now_v7(),
                    position: Point::new(x, y),
                    size: (cx - x, cy - y),
                });
            }

            // RECORD=14: Child Sheet / Hierarchical Sheet Symbol
            14 => {
                let sheet_name = rec.get("SHEETNAME").unwrap_or("").to_string();
                let file_name = rec.get("FILENAME").unwrap_or("").to_string();
                let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                let xs = rec.get_coord_mm("XSIZE").unwrap_or(50.0);
                let ys = rec.get_coord_mm("YSIZE").unwrap_or(30.0);

                sheet.child_sheets.push(ChildSheet {
                    uuid: Uuid::now_v7(),
                    name: sheet_name,
                    filename: file_name,
                    position: Point::new(x, y),
                    size: (xs, ys),
                    stroke_width: 0.0,
                    fill: oxide_types::schematic::FillType::None,
                    stroke_color: None,
                    fill_color: None,
                    fields_autoplaced: false,
                    pins: Vec::new(),
                    instances: Vec::new(),
                });
            }

            _ => {}
        }
    }

    Ok(sheet)
}
