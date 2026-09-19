//! Altium Designer Schematic Symbol Library (.SchLib) importer.

use std::collections::BTreeMap;
use chrono::Utc;
use uuid::Uuid;

use oxide_library::primitive::symbol::{
    ComponentType, PinDirection, PinOrientation, PinSymbolKind, Symbol as LibSymbol,
    SymbolGraphic, SymbolGraphicKind, SymbolPin,
};

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::record::{parse_record_stream, AltiumRecord};

/// Import all symbols from an Altium `.SchLib` file byte slice.
pub fn import_schlib_bytes(bytes: &[u8]) -> Result<Vec<LibSymbol>, AltiumImportError> {
    let cfb = CfbContainer::parse(bytes)?;
    let mut symbols = Vec::new();

    // Check for component streams or FileHeader stream
    if let Ok(header_data) = cfb.get_stream("FileHeader") {
        let records = parse_record_stream(&header_data)?;
        let parsed = parse_symbols_from_records(&records)?;
        symbols.extend(parsed);
    }

    // Check individual storage streams (often named after the component name or index)
    for (name, stream_bytes) in &cfb.streams {
        if name != "FileHeader" && name != "Storage" && !name.starts_with('/') {
            if let Ok(records) = parse_record_stream(stream_bytes) {
                if let Ok(parsed) = parse_symbols_from_records(&records) {
                    for sym in parsed {
                        if !symbols.iter().any(|s| s.name == sym.name) {
                            symbols.push(sym);
                        }
                    }
                }
            }
        }
    }

    Ok(symbols)
}

/// Parse multiple [`LibSymbol`]s from a record slice.
pub fn parse_symbols_from_records(records: &[AltiumRecord]) -> Result<Vec<LibSymbol>, AltiumImportError> {
    let mut symbols = Vec::new();
    let mut current_symbol: Option<LibSymbol> = None;

    for rec in records {
        let record_type = rec.get_i64("RECORD").unwrap_or(-1);

        match record_type {
            // RECORD=1: Component Header in Library
            1 => {
                if let Some(prev) = current_symbol.take() {
                    symbols.push(prev);
                }

                let name = rec
                    .get("LIBREFERENCE")
                    .or_else(|| rec.get("NAME"))
                    .unwrap_or("ALT_SYM")
                    .to_string();
                let desc = rec.get("DESCRIPTION").unwrap_or("").to_string();
                let designator = rec.get("DESIGNATOR").unwrap_or("U?").to_string();
                let comment = rec.get("COMMENT").unwrap_or("*").to_string();
                let now = Utc::now();

                current_symbol = Some(LibSymbol {
                    uuid: Uuid::now_v7(),
                    name,
                    anchor: [0.0, 0.0],
                    pins: Vec::new(),
                    graphics: Vec::new(),
                    schematic_params: BTreeMap::new(),
                    designator,
                    comment,
                    description: desc,
                    component_type: ComponentType::Standard,
                    mirrored: false,
                    local_fill_color: None,
                    local_line_color: None,
                    local_pin_color: None,
                    version: "0.0.1".to_string(),
                    released: false,
                    part_count: 1,
                    created: now,
                    updated: now,
                });
            }

            // RECORD=2: Pin
            2 => {
                if let Some(sym) = current_symbol.as_mut() {
                    let number = rec.get("DESIGNATOR").unwrap_or("1").to_string();
                    let name = rec.get("NAME").unwrap_or("").to_string();
                    let x = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                    let y = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                    let len_mm = rec.get_coord_mm("PINLENGTH").unwrap_or(2.54);

                    let orientation = match rec.get_i64("PINCONGLOMERATE").unwrap_or(0) & 0x03 {
                        0 => PinOrientation::Right,
                        1 => PinOrientation::Up,
                        2 => PinOrientation::Left,
                        3 => PinOrientation::Down,
                        _ => PinOrientation::Right,
                    };

                    let electrical = match rec.get_i64("ELECTRICAL").unwrap_or(0) {
                        0 => PinDirection::Input,
                        1 => PinDirection::Bidirectional,
                        2 => PinDirection::Output,
                        3 => PinDirection::OpenCollector,
                        4 => PinDirection::Passive,
                        7 => PinDirection::Power,
                        _ => PinDirection::Passive,
                    };

                    sym.pins.push(SymbolPin {
                        number,
                        name,
                        electrical,
                        position: [x, y],
                        orientation,
                        length: len_mm,
                        description: String::new(),
                        function: Vec::new(),
                        pin_package_length: None,
                        propagation_delay_ns: None,
                        designator_visible: true,
                        name_visible: true,
                        inside_symbol: PinSymbolKind::None,
                        inside_edge_symbol: PinSymbolKind::None,
                        outside_edge_symbol: PinSymbolKind::None,
                        outside_symbol: PinSymbolKind::None,
                        hidden: false,
                        locked: false,
                        part_number: 0,
                    });
                }
            }

            // RECORD=6: Rectangle Graphic
            6 => {
                if let Some(sym) = current_symbol.as_mut() {
                    let x1 = rec.get_coord_mm("LOCATION.X").unwrap_or(0.0);
                    let y1 = rec.get_coord_mm("LOCATION.Y").unwrap_or(0.0);
                    let x2 = rec.get_coord_mm("CORNER.X").unwrap_or(x1 + 10.0);
                    let y2 = rec.get_coord_mm("CORNER.Y").unwrap_or(y1 + 10.0);

                    sym.graphics.push(SymbolGraphic {
                        kind: SymbolGraphicKind::Rectangle {
                            from: [x1, y1],
                            to: [x2, y2],
                        },
                        stroke_width: 0.15,
                        fill: None,
                        part_number: 0,
                    });
                }
            }

            // RECORD=5: Line / Polyline Graphic
            5 => {
                if let Some(sym) = current_symbol.as_mut() {
                    let count = rec.get_i64("LOCATIONCOUNT").unwrap_or(2) as usize;
                    let mut points = Vec::new();
                    for i in 1..=count {
                        let x_key = format!("X{i}");
                        let y_key = format!("Y{i}");
                        if let (Some(x), Some(y)) = (rec.get_coord_mm(&x_key), rec.get_coord_mm(&y_key)) {
                            points.push([x, y]);
                        }
                    }
                    if points.len() >= 2 {
                        for w in points.windows(2) {
                            sym.graphics.push(SymbolGraphic {
                                kind: SymbolGraphicKind::Line {
                                    from: w[0],
                                    to: w[1],
                                },
                                stroke_width: 0.15,
                                fill: None,
                                part_number: 0,
                            });
                        }
                    }
                }
            }

            _ => {}
        }
    }

    if let Some(last) = current_symbol {
        symbols.push(last);
    }

    Ok(symbols)
}
