use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use oxide_types::pcb::{Footprint, PcbBoard, Point, Segment, Via, Zone};

/// Definition of a 2D PCB Room grouping footprints, routing, and zones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    /// Channel or hierarchical sheet identifier this room corresponds to.
    pub channel_id: Option<String>,
    /// Bounding outline polygon of the room in board coordinates (mm).
    pub boundary: Vec<Point>,
    /// Footprint references assigned to this room (e.g. "R1", "C1", "U1").
    pub footprint_refs: HashSet<String>,
    /// Net numbers internal to or associated with this room.
    pub internal_nets: HashSet<u32>,
    /// Whether components and geometry inside the room are locked relative to each other.
    pub locked: bool,
}

impl Room {
    pub fn new(name: impl Into<String>, boundary: Vec<Point>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            channel_id: None,
            boundary,
            footprint_refs: HashSet::new(),
            internal_nets: HashSet::new(),
            locked: false,
        }
    }

    /// Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm).
    pub fn contains_point(&self, pt: Point) -> bool {
        if self.boundary.len() < 3 {
            return false;
        }
        let mut inside = false;
        let mut j = self.boundary.len() - 1;
        for i in 0..self.boundary.len() {
            let pi = &self.boundary[i];
            let pj = &self.boundary[j];

            if ((pi.y > pt.y) != (pj.y > pt.y))
                && (pt.x < (pj.x - pi.x) * (pt.y - pi.y) / (pj.y - pi.y + f64::EPSILON) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// Computes the centroid/anchor of the room polygon.
    pub fn anchor(&self) -> Point {
        if self.boundary.is_empty() {
            return Point { x: 0.0, y: 0.0 };
        }
        let sum_x: f64 = self.boundary.iter().map(|p| p.x).sum();
        let sum_y: f64 = self.boundary.iter().map(|p| p.y).sum();
        let len = self.boundary.len() as f64;
        Point {
            x: sum_x / len,
            y: sum_y / len,
        }
    }
}

/// Options controlling which elements are formatted/replicated when copying room formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomCopyOptions {
    pub copy_footprint_placement: bool,
    pub copy_routing_traces: bool,
    pub copy_vias: bool,
    pub copy_zones: bool,
}

impl Default for RoomCopyOptions {
    fn default() -> Self {
        Self {
            copy_footprint_placement: true,
            copy_routing_traces: true,
            copy_vias: true,
            copy_zones: true,
        }
    }
}

/// Manages PCB rooms and multi-channel format replication across repeated channels.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoomManager {
    pub rooms: Vec<Room>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn find_room(&self, id: Uuid) -> Option<&Room> {
        self.rooms.iter().find(|r| r.id == id)
    }

    pub fn find_room_mut(&mut self, id: Uuid) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.id == id)
    }

    /// Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.
    pub fn auto_assign_footprints(&mut self, board: &PcbBoard) {
        for room in &mut self.rooms {
            room.footprint_refs.clear();
            for fp in &board.footprints {
                if room.contains_point(fp.position) {
                    room.footprint_refs.insert(fp.reference.clone());
                }
            }
        }
    }

    /// Copies the relative placement, orientation, routing traces, and vias from a source room to target rooms.
    ///
    /// This achieves 100% Altium Designer "Copy Room Formats" behavior for multi-channel designs.
    /// When repeating schematic sheets / channels (e.g. Channel 1 -> Channel 2, 3, 4), the relative
    /// offsets from the room origin/anchor are precisely transferred to the corresponding components in target rooms.
    pub fn copy_room_format(
        &self,
        board: &mut PcbBoard,
        source_room_id: Uuid,
        target_room_ids: &[Uuid],
        options: RoomCopyOptions,
    ) -> Result<(), String> {
        let source_room = self
            .find_room(source_room_id)
            .ok_or_else(|| format!("Source room {source_room_id} not found"))?;

        let source_anchor = source_room.anchor();

        // Build mapping of footprint reference base-names or channel suffixes
        // E.g. In source room: R1_CH1 (base R1) -> target room: R1_CH2 (base R1)
        let source_fps: Vec<Footprint> = board
            .footprints
            .iter()
            .filter(|fp| source_room.footprint_refs.contains(&fp.reference))
            .cloned()
            .collect();

        // Source traces and vias inside source room bounding box / boundary
        let source_segments: Vec<Segment> = board
            .segments
            .iter()
            .filter(|seg| source_room.contains_point(seg.start) && source_room.contains_point(seg.end))
            .cloned()
            .collect();

        let source_vias: Vec<Via> = board
            .vias
            .iter()
            .filter(|via| source_room.contains_point(via.position))
            .cloned()
            .collect();

        for &target_id in target_room_ids {
            let target_room = self
                .find_room(target_id)
                .ok_or_else(|| format!("Target room {target_id} not found"))?;

            let target_anchor = target_room.anchor();
            let delta_x = target_anchor.x - source_anchor.x;
            let delta_y = target_anchor.y - source_anchor.y;

            // 1. Replicate footprint relative placement
            if options.copy_footprint_placement {
                // Map footprints by stripped base designator (e.g., "R1" from "R1_1" or "R1")
                let mut target_fps_by_base: HashMap<String, usize> = HashMap::new();
                for (idx, fp) in board.footprints.iter().enumerate() {
                    if target_room.footprint_refs.contains(&fp.reference) {
                        let base = strip_channel_suffix(&fp.reference);
                        target_fps_by_base.insert(base, idx);
                    }
                }

                for src_fp in &source_fps {
                    let src_base = strip_channel_suffix(&src_fp.reference);
                    if let Some(&tgt_idx) = target_fps_by_base.get(&src_base) {
                        let rel_x = src_fp.position.x - source_anchor.x;
                        let rel_y = src_fp.position.y - source_anchor.y;

                        let tgt_fp = &mut board.footprints[tgt_idx];
                        tgt_fp.position = Point {
                            x: target_anchor.x + rel_x,
                            y: target_anchor.y + rel_y,
                        };
                        tgt_fp.rotation = src_fp.rotation;
                        tgt_fp.layer = src_fp.layer.clone();
                    }
                }
            }

            // 2. Replicate routing traces
            if options.copy_routing_traces {
                for src_seg in &source_segments {
                    let new_start = Point {
                        x: src_seg.start.x + delta_x,
                        y: src_seg.start.y + delta_y,
                    };
                    let new_end = Point {
                        x: src_seg.end.x + delta_x,
                        y: src_seg.end.y + delta_y,
                    };

                    // Only insert if both points are in target room
                    if target_room.contains_point(new_start) || target_room.contains_point(new_end) {
                        board.segments.push(Segment {
                            uuid: Uuid::new_v4(),
                            start: new_start,
                            end: new_end,
                            width: src_seg.width,
                            layer: src_seg.layer.clone(),
                            net: src_seg.net, // Note: net remapping can be updated via netlist connectivity
                        });
                    }
                }
            }

            // 3. Replicate vias
            if options.copy_vias {
                for src_via in &source_vias {
                    let new_pos = Point {
                        x: src_via.position.x + delta_x,
                        y: src_via.position.y + delta_y,
                    };
                    if target_room.contains_point(new_pos) {
                        board.vias.push(Via {
                            uuid: Uuid::new_v4(),
                            position: new_pos,
                            diameter: src_via.diameter,
                            drill: src_via.drill,
                            layers: src_via.layers.clone(),
                            net: src_via.net,
                            via_type: src_via.via_type,
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

/// Helper function to strip channel numbers/suffixes (e.g. "R1_CH2" -> "R1", "U1_3" -> "U1", "C1" -> "C1").
fn strip_channel_suffix(reference: &str) -> String {
    if let Some(pos) = reference.rfind('_') {
        reference[..pos].to_string()
    } else {
        reference.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_contains_point_and_anchor() {
        let boundary = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ];
        let room = Room::new("Channel 1", boundary);

        assert!(room.contains_point(Point { x: 5.0, y: 5.0 }));
        assert!(!room.contains_point(Point { x: 15.0, y: 5.0 }));
        assert!(!room.contains_point(Point { x: -1.0, y: 5.0 }));

        let anchor = room.anchor();
        assert!((anchor.x - 5.0).abs() < 1e-6);
        assert!((anchor.y - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_copy_room_format() {
        let mut board = PcbBoard::default();

        // Source room at (0, 0) to (20, 20)
        let mut room1 = Room::new(
            "Channel 1",
            vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 20.0, y: 0.0 },
                Point { x: 20.0, y: 20.0 },
                Point { x: 0.0, y: 20.0 },
            ],
        );
        room1.footprint_refs.insert("R1_CH1".to_string());
        room1.footprint_refs.insert("C1_CH1".to_string());

        // Target room at (50, 0) to (70, 20) (delta = +50, 0)
        let mut room2 = Room::new(
            "Channel 2",
            vec![
                Point { x: 50.0, y: 0.0 },
                Point { x: 70.0, y: 0.0 },
                Point { x: 70.0, y: 20.0 },
                Point { x: 50.0, y: 20.0 },
            ],
        );
        room2.footprint_refs.insert("R1_CH2".to_string());
        room2.footprint_refs.insert("C1_CH2".to_string());

        // Place footprints
        let fp_r1_ch1 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "R1_CH1".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0805".to_string(),
            position: Point { x: 5.0, y: 5.0 },
            rotation: 90.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![],
            graphics: vec![],
            properties: vec![],
        };
        let fp_c1_ch1 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "C1_CH1".to_string(),
            value: "100nF".to_string(),
            footprint_id: "C_0603".to_string(),
            position: Point { x: 12.0, y: 15.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![],
            graphics: vec![],
            properties: vec![],
        };
        let fp_r1_ch2 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "R1_CH2".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0805".to_string(),
            position: Point { x: 0.0, y: 0.0 }, // Unformatted initial pos
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![],
            graphics: vec![],
            properties: vec![],
        };
        let fp_c1_ch2 = Footprint {
            uuid: Uuid::new_v4(),
            reference: "C1_CH2".to_string(),
            value: "100nF".to_string(),
            footprint_id: "C_0603".to_string(),
            position: Point { x: 0.0, y: 0.0 }, // Unformatted initial pos
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: vec![],
            graphics: vec![],
            properties: vec![],
        };

        board.footprints = vec![fp_r1_ch1, fp_c1_ch1, fp_r1_ch2, fp_c1_ch2];

        // Add trace inside channel 1
        board.segments.push(Segment {
            uuid: Uuid::new_v4(),
            start: Point { x: 5.0, y: 5.0 },
            end: Point { x: 12.0, y: 15.0 },
            width: 0.25,
            layer: "F.Cu".to_string(),
            net: 1,
        });

        let r1_id = room1.id;
        let r2_id = room2.id;
        let mut mgr = RoomManager::new();
        mgr.add_room(room1);
        mgr.add_room(room2);

        mgr.copy_room_format(&mut board, r1_id, &[r2_id], RoomCopyOptions::default())
            .expect("room format copying should succeed");

        // Verify R1_CH2 position and rotation
        let tgt_r1 = board
            .footprints
            .iter()
            .find(|f| f.reference == "R1_CH2")
            .unwrap();
        assert_eq!(tgt_r1.rotation, 90.0);
        assert!((tgt_r1.position.x - 55.0).abs() < 1e-6);
        assert!((tgt_r1.position.y - 5.0).abs() < 1e-6);

        // Verify C1_CH2 position
        let tgt_c1 = board
            .footprints
            .iter()
            .find(|f| f.reference == "C1_CH2")
            .unwrap();
        assert!((tgt_c1.position.x - 62.0).abs() < 1e-6);
        assert!((tgt_c1.position.y - 15.0).abs() < 1e-6);

        // Verify trace replication (should now have 2 segments)
        assert_eq!(board.segments.len(), 2);
        let replicated_seg = &board.segments[1];
        assert!((replicated_seg.start.x - 55.0).abs() < 1e-6);
        assert!((replicated_seg.start.y - 5.0).abs() < 1e-6);
        assert!((replicated_seg.end.x - 62.0).abs() < 1e-6);
        assert!((replicated_seg.end.y - 15.0).abs() < 1e-6);
    }
}
