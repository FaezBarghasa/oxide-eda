use uuid::Uuid;

use oxide_types::pcb::{Footprint, LayerDef, NetDef, PcbBoard, Point, Segment, Via, ViaType};

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::record::{AltiumRecord, parse_record_stream};

/// Import an Altium `.PcbDoc` file from raw binary bytes.
pub fn import_pcbdoc_bytes(bytes: &[u8]) -> Result<PcbBoard, AltiumImportError> {
    let cfb = CfbContainer::parse(bytes)?;
    let mut board = PcbBoard {
        uuid: Uuid::now_v7(),
        version: 1,
        generator: "Oxide Altium Importer".to_string(),
        thickness: 1.6,
        outline: Vec::new(),
        layers: vec![
            LayerDef {
                id: 0,
                name: "Top Layer".to_string(),
                layer_type: "signal".to_string(),
            },
            LayerDef {
                id: 1,
                name: "Bottom Layer".to_string(),
                layer_type: "signal".to_string(),
            },
            LayerDef {
                id: 2,
                name: "Top Solder".to_string(),
                layer_type: "mask".to_string(),
            },
            LayerDef {
                id: 3,
                name: "Bottom Solder".to_string(),
                layer_type: "mask".to_string(),
            },
            LayerDef {
                id: 4,
                name: "Top Overlay".to_string(),
                layer_type: "silkscreen".to_string(),
            },
            LayerDef {
                id: 5,
                name: "Bottom Overlay".to_string(),
                layer_type: "silkscreen".to_string(),
            },
        ],
        setup: None,
        nets: Vec::new(),
        footprints: Vec::new(),
        segments: Vec::new(),
        vias: Vec::new(),
        zones: Vec::new(),
        graphics: Vec::new(),
        texts: Vec::new(),
    };

    // 1. Process Board stream if available (outline, layers, nets)
    if let Ok(board_data) = cfb
        .get_stream("Board")
        .or_else(|_| cfb.get_stream("Board6"))
        && let Ok(records) = parse_record_stream(&board_data)
    {
        parse_board_metadata(&records, &mut board)?;
    }

    // 1b. Process Nets stream if available
    if let Ok(nets_data) = cfb.get_stream("Nets").or_else(|_| cfb.get_stream("Nets6"))
        && let Ok(records) = parse_record_stream(&nets_data)
    {
        parse_board_metadata(&records, &mut board)?;
    }

    // 2. Process Tracks stream (text records or PCB 6 54-byte binary records)
    if let Ok(tracks_data) = cfb
        .get_stream("Tracks")
        .or_else(|_| cfb.get_stream("Tracks6"))
    {
        if let Ok(records) = parse_record_stream(&tracks_data)
            && !records.is_empty()
        {
            parse_tracks(&records, &mut board);
        }
        if tracks_data.len() >= 54 && tracks_data.len() % 54 == 0 {
            parse_binary_tracks(&tracks_data, &mut board);
        }
    }

    // 3. Process Vias stream
    if let Ok(vias_data) = cfb.get_stream("Vias").or_else(|_| cfb.get_stream("Vias6"))
        && let Ok(records) = parse_record_stream(&vias_data)
    {
        parse_vias(&records, &mut board);
    }

    // 4. Process Components & Pads stream
    if let Ok(comp_data) = cfb
        .get_stream("Components")
        .or_else(|_| cfb.get_stream("Components6"))
        && let Ok(records) = parse_record_stream(&comp_data)
    {
        parse_components(&records, &mut board);
    }

    // 4b. Process Pads stream if separate
    if let Ok(pads_data) = cfb.get_stream("Pads").or_else(|_| cfb.get_stream("Pads6")) {
        if let Ok(records) = parse_record_stream(&pads_data)
            && !records.is_empty()
        {
            parse_pads(&records, &mut board);
        }
        if pads_data.windows(3).any(|w| w == b"|&|") {
            parse_binary_pad_parts(&pads_data, &mut board);
        }
    }

    // 5. Check FileHeader stream for all records combined
    if let Ok(header_data) = cfb
        .get_stream("FileHeader")
        .or_else(|_| cfb.get_stream("FileHeaderSix"))
        && let Ok(records) = parse_record_stream(&header_data)
    {
        parse_all_pcb_records(&records, &mut board);
    }

    Ok(board)
}

fn parse_board_metadata(
    records: &[AltiumRecord],
    board: &mut PcbBoard,
) -> Result<(), AltiumImportError> {
    for rec in records {
        if let Some(net_name) = rec.get("NETNAME").or_else(|| rec.get("NAME")) {
            let net_id = rec
                .get_i64("NETID")
                .or_else(|| rec.get_i64("ID"))
                .unwrap_or(board.nets.len() as i64) as u32;
            if !board.nets.iter().any(|n| n.name == net_name) {
                board.nets.push(NetDef {
                    number: net_id,
                    name: net_name.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn parse_tracks(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let x1 = rec
            .get_coord_mm("X1")
            .or_else(|| rec.get_coord_mm("LOCATION.X"))
            .unwrap_or(0.0);
        let y1 = rec
            .get_coord_mm("Y1")
            .or_else(|| rec.get_coord_mm("LOCATION.Y"))
            .unwrap_or(0.0);
        let x2 = rec
            .get_coord_mm("X2")
            .or_else(|| rec.get_coord_mm("CORNER.X"))
            .unwrap_or(x1);
        let y2 = rec
            .get_coord_mm("Y2")
            .or_else(|| rec.get_coord_mm("CORNER.Y"))
            .unwrap_or(y1);
        let width = rec.get_coord_mm("WIDTH").unwrap_or(0.25);
        let layer = rec.get("LAYER").unwrap_or("Top Layer").to_string();
        let net = rec.get_i64("NET").unwrap_or(0) as u32;

        board.segments.push(Segment {
            uuid: Uuid::now_v7(),
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
            width,
            layer,
            net,
        });
    }
}

fn parse_vias(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let x = rec
            .get_coord_mm("LOCATION.X")
            .or_else(|| rec.get_coord_mm("X"))
            .unwrap_or(0.0);
        let y = rec
            .get_coord_mm("LOCATION.Y")
            .or_else(|| rec.get_coord_mm("Y"))
            .unwrap_or(0.0);
        let diameter = rec.get_coord_mm("DIAMETER").unwrap_or(0.6);
        let drill = rec.get_coord_mm("HOLESIZE").unwrap_or(0.3);
        let net = rec.get_i64("NET").unwrap_or(0) as u32;

        board.vias.push(Via {
            uuid: Uuid::now_v7(),
            position: Point::new(x, y),
            diameter,
            drill,
            layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
            net,
            via_type: ViaType::Through,
            via_span: None,
        });
    }
}

fn parse_components(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let mut designator = rec
            .get("SOURCEDESIGNATOR")
            .or_else(|| rec.get("SOURCECOMPONENTNAME"))
            .or_else(|| rec.get("NAME"))
            .unwrap_or("")
            .to_string();
        let footprint_id = rec.get("PATTERN").unwrap_or("").to_string();
        let comment = rec
            .get("COMMENT")
            .or_else(|| rec.get("SOURCEDESCRIPTION"))
            .unwrap_or("")
            .to_string();
        let x = rec
            .get_coord_mm("LOCATION.X")
            .or_else(|| rec.get_coord_mm("X"))
            .unwrap_or(0.0);
        let y = rec
            .get_coord_mm("LOCATION.Y")
            .or_else(|| rec.get_coord_mm("Y"))
            .unwrap_or(0.0);
        let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);
        let layer = rec.get("LAYER").unwrap_or("Top Layer").to_string();

        if designator.is_empty() && !footprint_id.is_empty() {
            designator = format!("U{}", board.footprints.len() + 1);
        }

        if !designator.is_empty() || !footprint_id.is_empty() {
            board.footprints.push(Footprint {
                uuid: Uuid::now_v7(),
                reference: designator,
                value: comment,
                footprint_id,
                position: Point::new(x, y),
                rotation,
                layer,
                locked: false,
                pads: Vec::new(),
                graphics: Vec::new(),
                properties: Vec::new(),
            });
        }
    }
}

/// Parse Altium PCB 6 54-byte fixed-size binary track records.
fn parse_binary_tracks(data: &[u8], board: &mut PcbBoard) {
    let rec_size = 54;
    let count = data.len() / rec_size;
    for i in 0..count {
        let rec = &data[i * rec_size..(i + 1) * rec_size];
        if rec[0] != 0x04 {
            continue; // Record type must be 4 (Track)
        }
        let x1_raw = i32::from_le_bytes(rec[18..22].try_into().unwrap_or_default());
        let y1_raw = i32::from_le_bytes(rec[22..26].try_into().unwrap_or_default());
        let x2_raw = i32::from_le_bytes(rec[26..30].try_into().unwrap_or_default());
        let y2_raw = i32::from_le_bytes(rec[30..34].try_into().unwrap_or_default());
        let w_raw = i32::from_le_bytes(rec[34..38].try_into().unwrap_or_default());

        // Convert internal units (1/10000 mil = 0.00000254 mm)
        let x1 = x1_raw as f64 * 0.00000254;
        let y1 = y1_raw as f64 * 0.00000254;
        let x2 = x2_raw as f64 * 0.00000254;
        let y2 = y2_raw as f64 * 0.00000254;
        let width = (w_raw as f64 * 0.00000254).abs().max(0.1);

        let net_raw = u16::from_le_bytes(rec[8..10].try_into().unwrap_or_default());
        let net = if net_raw == 0xFFFF { 0 } else { net_raw as u32 };

        let layer_id = rec[46];
        let layer_name = match layer_id {
            1 => "Top Layer".to_string(),
            32 => "Bottom Layer".to_string(),
            6 => "Keep-Out Layer".to_string(),
            _ => format!("Layer_{layer_id}"),
        };

        board.segments.push(Segment {
            uuid: Uuid::now_v7(),
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
            width,
            layer: layer_name,
            net,
        });
    }
}

/// Parse Altium PCB 6 binary pad records separated by `|&|`.
fn parse_binary_pad_parts(data: &[u8], board: &mut PcbBoard) {
    let sep = b"|&|";
    let mut cursor = 0;
    let mut part_slices = Vec::new();
    while let Some(pos) = data[cursor..].windows(3).position(|w| w == sep) {
        part_slices.push(&data[cursor..cursor + pos]);
        cursor += pos + 3;
    }
    if cursor < data.len() {
        part_slices.push(&data[cursor..]);
    }

    for (idx, part) in part_slices.iter().enumerate() {
        if part.len() < 58 {
            continue;
        }
        let x_raw = i32::from_le_bytes(part[22..26].try_into().unwrap_or_default());
        let y_raw = i32::from_le_bytes(part[26..30].try_into().unwrap_or_default());
        let sx_raw = i32::from_le_bytes(part[30..34].try_into().unwrap_or_default());
        let sy_raw = i32::from_le_bytes(part[34..38].try_into().unwrap_or_default());
        let hole_raw = i32::from_le_bytes(part[54..58].try_into().unwrap_or_default());

        let x = x_raw as f64 * 0.00000254;
        let y = y_raw as f64 * 0.00000254;
        let size_x = (sx_raw as f64 * 0.00000254).abs().max(0.5);
        let size_y = (sy_raw as f64 * 0.00000254).abs().max(0.5);
        let hole = (hole_raw as f64 * 0.00000254).abs();

        let pad_type = if hole > 0.0 {
            oxide_types::pcb::PadType::Thru
        } else {
            oxide_types::pcb::PadType::Smd
        };

        let drill_def = if hole > 0.0 {
            Some(oxide_types::pcb::DrillDef {
                diameter: hole,
                shape: "round".to_string(),
            })
        } else {
            None
        };

        let pad = oxide_types::pcb::Pad {
            uuid: Uuid::now_v7(),
            number: format!("{}", idx),
            pad_type,
            shape: oxide_types::pcb::PadShape::Circle,
            position: Point::new(x, y),
            size: Point::new(size_x, size_y),
            drill: drill_def,
            layers: vec!["Multi-Layer".to_string()],
            net: None,
            roundrect_ratio: 0.0,
        };

        if let Some(fp) = board.footprints.last_mut() {
            fp.pads.push(pad);
        }
    }
}

/// Parse Altium Pad records and assign to footprints or board elements.
pub fn parse_pads(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let record_type = rec
            .get("RECORD")
            .or_else(|| rec.get("OBJECTTYPE"))
            .unwrap_or("");
        if !record_type.eq_ignore_ascii_case("Pad") && record_type != "4" {
            continue;
        }

        let name = rec
            .get("NAME")
            .or_else(|| rec.get("PADNAME"))
            .unwrap_or("1")
            .to_string();
        let x = rec
            .get_coord_mm("LOCATION.X")
            .or_else(|| rec.get_coord_mm("X"))
            .unwrap_or(0.0);
        let y = rec
            .get_coord_mm("LOCATION.Y")
            .or_else(|| rec.get_coord_mm("Y"))
            .unwrap_or(0.0);
        let size_x = rec
            .get_coord_mm("TOPXSIZE")
            .or_else(|| rec.get_coord_mm("XSIZE"))
            .unwrap_or(1.5);
        let size_y = rec
            .get_coord_mm("TOPYSIZE")
            .or_else(|| rec.get_coord_mm("YSIZE"))
            .unwrap_or(1.5);
        let hole_size = rec.get_coord_mm("HOLESIZE").unwrap_or(0.0);
        let layer_str = rec.get("LAYER").unwrap_or("Multi-Layer");
        let net_id = rec.get_i64("NET").unwrap_or(0) as u32;

        let pad_type = if layer_str.eq_ignore_ascii_case("Multi-Layer") || hole_size > 0.0 {
            oxide_types::pcb::PadType::Thru
        } else {
            oxide_types::pcb::PadType::Smd
        };

        // Determine shape and corner radius
        let shape_raw = rec
            .get("TOPSHAPE")
            .or_else(|| rec.get("SHAPE"))
            .unwrap_or("Round");
        let (shape, ratio) = match shape_raw {
            "0" | "Round" | "Circle" => (oxide_types::pcb::PadShape::Circle, 0.0),
            "1" | "Rectangular" | "Rectangle" => (oxide_types::pcb::PadShape::Rect, 0.0),
            "2" | "Oval" => (oxide_types::pcb::PadShape::Oval, 0.0),
            "3" | "RoundedRectangle" | "RoundRect" => {
                let r_pct = rec.get_f64("ROUNDRECTANGULARRADIUS").unwrap_or(25.0) / 100.0;
                (oxide_types::pcb::PadShape::RoundRect, r_pct)
            }
            "4" | "ChamferedRectangle" | "Custom" | "5" => {
                (oxide_types::pcb::PadShape::Custom, 0.0)
            }
            _ => (oxide_types::pcb::PadShape::Rect, 0.0),
        };

        let drill_def = if hole_size > 0.0 {
            Some(oxide_types::pcb::DrillDef {
                diameter: hole_size,
                shape: "circle".to_string(),
            })
        } else {
            None
        };

        let pad = oxide_types::pcb::Pad {
            uuid: Uuid::now_v7(),
            number: name,
            pad_type,
            shape,
            position: Point::new(x, y),
            size: Point::new(size_x, size_y),
            drill: drill_def,
            layers: vec![layer_str.to_string()],
            net: if net_id > 0 {
                Some(oxide_types::pcb::PadNet {
                    number: net_id,
                    name: format!("NET_{net_id}"),
                })
            } else {
                None
            },
            roundrect_ratio: ratio,
        };

        // Attach pad to closest footprint or last footprint
        if let Some(fp) = board.footprints.last_mut() {
            fp.pads.push(pad);
        }
    }
}

fn parse_all_pcb_records(records: &[AltiumRecord], board: &mut PcbBoard) {
    parse_pads(records, board);

    for rec in records {
        let record_type = rec
            .get("RECORD")
            .or_else(|| rec.get("OBJECTTYPE"))
            .unwrap_or("");
        if record_type.eq_ignore_ascii_case("Track") || record_type == "1" {
            if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (
                rec.get_coord_mm("X1"),
                rec.get_coord_mm("Y1"),
                rec.get_coord_mm("X2"),
                rec.get_coord_mm("Y2"),
            ) {
                let width = rec.get_coord_mm("WIDTH").unwrap_or(0.25);
                let layer = rec.get("LAYER").unwrap_or("Top Layer").to_string();
                let net = rec.get_i64("NET").unwrap_or(0) as u32;
                board.segments.push(Segment {
                    uuid: Uuid::now_v7(),
                    start: Point::new(x1, y1),
                    end: Point::new(x2, y2),
                    width,
                    layer,
                    net,
                });
            }
        } else if (record_type.eq_ignore_ascii_case("Via") || record_type == "2")
            && let (Some(x), Some(y)) = (
                rec.get_coord_mm("X")
                    .or_else(|| rec.get_coord_mm("LOCATION.X")),
                rec.get_coord_mm("Y")
                    .or_else(|| rec.get_coord_mm("LOCATION.Y")),
            )
        {
            let diameter = rec.get_coord_mm("DIAMETER").unwrap_or(0.6);
            let drill = rec.get_coord_mm("HOLESIZE").unwrap_or(0.3);
            let net = rec.get_i64("NET").unwrap_or(0) as u32;
            board.vias.push(Via {
                uuid: Uuid::now_v7(),
                position: Point::new(x, y),
                diameter,
                drill,
                layers: vec!["Top Layer".to_string(), "Bottom Layer".to_string()],
                net,
                via_type: ViaType::Through,
                via_span: None,
            });
        }
    }
}
