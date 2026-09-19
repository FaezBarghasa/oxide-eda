//! Altium Designer PCB Document (.PcbDoc) importer.

use std::collections::HashMap;
use uuid::Uuid;

use oxide_types::pcb::{
    DrillDef, Footprint, FpGraphic, LayerDef, NetDef, Pad, PadNet, PadShape, PadType, PcbBoard,
    Point, Segment, Via, ViaType, Zone,
};

use crate::cfb::CfbContainer;
use crate::error::AltiumImportError;
use crate::record::{parse_record_stream, AltiumRecord};

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
    if let Ok(board_data) = cfb.get_stream("Board") {
        if let Ok(records) = parse_record_stream(&board_data) {
            parse_board_metadata(&records, &mut board)?;
        }
    }

    // 2. Process Tracks stream
    if let Ok(tracks_data) = cfb.get_stream("Tracks") {
        if let Ok(records) = parse_record_stream(&tracks_data) {
            parse_tracks(&records, &mut board);
        }
    }

    // 3. Process Vias stream
    if let Ok(vias_data) = cfb.get_stream("Vias") {
        if let Ok(records) = parse_record_stream(&vias_data) {
            parse_vias(&records, &mut board);
        }
    }

    // 4. Process Components & Pads stream
    if let Ok(comp_data) = cfb.get_stream("Components") {
        if let Ok(records) = parse_record_stream(&comp_data) {
            parse_components(&records, &mut board);
        }
    }

    // 5. Check FileHeader stream for all records combined
    if let Ok(header_data) = cfb.get_stream("FileHeader") {
        if let Ok(records) = parse_record_stream(&header_data) {
            parse_all_pcb_records(&records, &mut board);
        }
    }

    Ok(board)
}

fn parse_board_metadata(records: &[AltiumRecord], board: &mut PcbBoard) -> Result<(), AltiumImportError> {
    for rec in records {
        if let Some(net_name) = rec.get("NETNAME").or_else(|| rec.get("NAME")) {
            let net_id = rec.get_i64("NETID").or_else(|| rec.get_i64("ID")).unwrap_or(board.nets.len() as i64) as u32;
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
        let x1 = rec.get_coord_mm("X1").or_else(|| rec.get_coord_mm("LOCATION.X")).unwrap_or(0.0);
        let y1 = rec.get_coord_mm("Y1").or_else(|| rec.get_coord_mm("LOCATION.Y")).unwrap_or(0.0);
        let x2 = rec.get_coord_mm("X2").or_else(|| rec.get_coord_mm("CORNER.X")).unwrap_or(x1);
        let y2 = rec.get_coord_mm("Y2").or_else(|| rec.get_coord_mm("CORNER.Y")).unwrap_or(y1);
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
        let x = rec.get_coord_mm("LOCATION.X").or_else(|| rec.get_coord_mm("X")).unwrap_or(0.0);
        let y = rec.get_coord_mm("LOCATION.Y").or_else(|| rec.get_coord_mm("Y")).unwrap_or(0.0);
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
        });
    }
}

fn parse_components(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let designator = rec.get("SOURCECOMPONENTNAME").or_else(|| rec.get("NAME")).unwrap_or("").to_string();
        let footprint_id = rec.get("PATTERN").unwrap_or("").to_string();
        let comment = rec.get("COMMENT").unwrap_or("").to_string();
        let x = rec.get_coord_mm("LOCATION.X").or_else(|| rec.get_coord_mm("X")).unwrap_or(0.0);
        let y = rec.get_coord_mm("LOCATION.Y").or_else(|| rec.get_coord_mm("Y")).unwrap_or(0.0);
        let rotation = rec.get_f64("ROTATION").unwrap_or(0.0);
        let layer = rec.get("LAYER").unwrap_or("Top Layer").to_string();

        if !designator.is_empty() {
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

fn parse_all_pcb_records(records: &[AltiumRecord], board: &mut PcbBoard) {
    for rec in records {
        let record_type = rec.get("RECORD").or_else(|| rec.get("OBJECTTYPE")).unwrap_or("");
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
        } else if record_type.eq_ignore_ascii_case("Via") || record_type == "2" {
            if let (Some(x), Some(y)) = (rec.get_coord_mm("X").or_else(|| rec.get_coord_mm("LOCATION.X")), rec.get_coord_mm("Y").or_else(|| rec.get_coord_mm("LOCATION.Y"))) {
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
                });
            }
        }
    }
}
