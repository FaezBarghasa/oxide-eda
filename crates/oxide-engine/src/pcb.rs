//! Authoritative transactional PCB layout document engine.
//!
//! Provides transactional mutations, reversible undo/redo journals,
//! and generation-tracked snapshots for PCB primitives (footprints, pads,
//! tracks, arcs, vias, and copper zones).

use std::path::PathBuf;
use oxide_types::pcb::{Footprint, PcbBoard, Point, Segment, Via, Zone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::EngineError;

/// PCB entity category for selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectedPcbKind {
    Footprint,
    Pad,
    Segment,
    Via,
    Zone,
    Graphic,
}

/// A selected PCB primitive item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedPcbItem {
    pub uuid: Uuid,
    pub kind: SelectedPcbKind,
    pub name: Option<String>,
}

/// Individual invertible PCB layout command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PcbCommand {
    ReplaceBoard {
        board: PcbBoard,
    },
    PlaceFootprint {
        footprint: Footprint,
    },
    MoveFootprint {
        uuid: Uuid,
        dx: f64,
        dy: f64,
    },
    RotateFootprint {
        uuid: Uuid,
        delta_deg: f64,
    },
    DeleteFootprint {
        uuid: Uuid,
    },
    AddSegment {
        segment: Segment,
    },
    MoveSegment {
        uuid: Uuid,
        new_start: Point,
        new_end: Point,
    },
    DeleteSegment {
        uuid: Uuid,
    },
    AddVia {
        via: Via,
    },
    MoveVia {
        uuid: Uuid,
        new_position: Point,
    },
    DeleteVia {
        uuid: Uuid,
    },
    AddZone {
        zone: Zone,
    },
    DeleteZone {
        uuid: Uuid,
    },
    MoveSelection {
        items: Vec<SelectedPcbItem>,
        dx: f64,
        dy: f64,
    },
}

#[derive(Debug, Clone)]
struct PcbHistoryEntry {
    inverse_command: PcbCommand,
}

/// Transactional PCB layout engine.
#[derive(Debug, Clone)]
pub struct PcbEngine {
    board: PcbBoard,
    path: Option<PathBuf>,
    history: Vec<PcbHistoryEntry>,
    redo_stack: Vec<PcbHistoryEntry>,
    generation: u64,
}

impl PcbEngine {
    pub fn new(board: PcbBoard) -> Self {
        Self {
            board,
            path: None,
            history: Vec::new(),
            redo_stack: Vec::new(),
            generation: 1,
        }
    }

    pub fn new_with_path(board: PcbBoard, path: Option<PathBuf>) -> Self {
        Self {
            board,
            path,
            history: Vec::new(),
            redo_stack: Vec::new(),
            generation: 1,
        }
    }

    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    pub fn board(&self) -> &PcbBoard {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut PcbBoard {
        self.generation = self.generation.wrapping_add(1);
        &mut self.board
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Execute a PCB command and push its inverse to the undo stack.
    pub fn execute(&mut self, command: PcbCommand) -> Result<(), EngineError> {
        let inverse = self.apply_command(command)?;
        if let Some(inv) = inverse {
            self.history.push(PcbHistoryEntry {
                inverse_command: inv,
            });
            self.redo_stack.clear();
        }
        self.generation = self.generation.wrapping_add(1);
        Ok(())
    }

    /// Undo the last PCB layout transaction.
    pub fn undo(&mut self) -> Result<bool, EngineError> {
        let Some(entry) = self.history.pop() else {
            return Ok(false);
        };

        let inverse_for_redo = self.apply_command(entry.inverse_command)?;
        if let Some(redo_cmd) = inverse_for_redo {
            self.redo_stack.push(PcbHistoryEntry {
                inverse_command: redo_cmd,
            });
        }
        self.generation = self.generation.wrapping_add(1);
        Ok(true)
    }

    /// Redo the last undone PCB layout transaction.
    pub fn redo(&mut self) -> Result<bool, EngineError> {
        let Some(entry) = self.redo_stack.pop() else {
            return Ok(false);
        };

        let inverse_for_undo = self.apply_command(entry.inverse_command)?;
        if let Some(undo_cmd) = inverse_for_undo {
            self.history.push(PcbHistoryEntry {
                inverse_command: undo_cmd,
            });
        }
        self.generation = self.generation.wrapping_add(1);
        Ok(true)
    }

    /// Internal command applier, returns the inverse command for undo/redo.
    fn apply_command(&mut self, command: PcbCommand) -> Result<Option<PcbCommand>, EngineError> {
        match command {
            PcbCommand::ReplaceBoard { board } => {
                let old_board = std::mem::replace(&mut self.board, board);
                Ok(Some(PcbCommand::ReplaceBoard { board: old_board }))
            }
            PcbCommand::PlaceFootprint { footprint } => {
                let uuid = footprint.uuid;
                self.board.footprints.push(footprint);
                Ok(Some(PcbCommand::DeleteFootprint { uuid }))
            }
            PcbCommand::MoveFootprint { uuid, dx, dy } => {
                if let Some(fp) = self.board.footprints.iter_mut().find(|f| f.uuid == uuid) {
                    fp.position.x += dx;
                    fp.position.y += dy;
                    for pad in &mut fp.pads {
                        pad.position.x += dx;
                        pad.position.y += dy;
                    }
                    Ok(Some(PcbCommand::MoveFootprint {
                        uuid,
                        dx: -dx,
                        dy: -dy,
                    }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::RotateFootprint { uuid, delta_deg } => {
                if let Some(fp) = self.board.footprints.iter_mut().find(|f| f.uuid == uuid) {
                    fp.rotation = (fp.rotation + delta_deg).rem_euclid(360.0);
                    Ok(Some(PcbCommand::RotateFootprint {
                        uuid,
                        delta_deg: -delta_deg,
                    }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::DeleteFootprint { uuid } => {
                if let Some(idx) = self.board.footprints.iter().position(|f| f.uuid == uuid) {
                    let old_fp = self.board.footprints.remove(idx);
                    Ok(Some(PcbCommand::PlaceFootprint { footprint: old_fp }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::AddSegment { segment } => {
                let uuid = segment.uuid;
                self.board.segments.push(segment);
                Ok(Some(PcbCommand::DeleteSegment { uuid }))
            }
            PcbCommand::MoveSegment {
                uuid,
                new_start,
                new_end,
            } => {
                if let Some(seg) = self.board.segments.iter_mut().find(|s| s.uuid == uuid) {
                    let old_start = seg.start;
                    let old_end = seg.end;
                    seg.start = new_start;
                    seg.end = new_end;
                    Ok(Some(PcbCommand::MoveSegment {
                        uuid,
                        new_start: old_start,
                        new_end: old_end,
                    }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::DeleteSegment { uuid } => {
                if let Some(idx) = self.board.segments.iter().position(|s| s.uuid == uuid) {
                    let old_seg = self.board.segments.remove(idx);
                    Ok(Some(PcbCommand::AddSegment { segment: old_seg }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::AddVia { via } => {
                let uuid = via.uuid;
                self.board.vias.push(via);
                Ok(Some(PcbCommand::DeleteVia { uuid }))
            }
            PcbCommand::MoveVia { uuid, new_position } => {
                if let Some(via) = self.board.vias.iter_mut().find(|v| v.uuid == uuid) {
                    let old_pos = via.position;
                    via.position = new_position;
                    Ok(Some(PcbCommand::MoveVia {
                        uuid,
                        new_position: old_pos,
                    }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::DeleteVia { uuid } => {
                if let Some(idx) = self.board.vias.iter().position(|v| v.uuid == uuid) {
                    let old_via = self.board.vias.remove(idx);
                    Ok(Some(PcbCommand::AddVia { via: old_via }))
                } else {
                    Ok(None)
                }
            }
            PcbCommand::AddZone { zone } => {
                let uuid = zone.uuid;
                self.board.zones.push(zone);
                Ok(Some(PcbCommand::DeleteZone { uuid }))
            }
            PcbCommand::DeleteZone { uuid } => {
                if let Some(idx) = self.board.zones.iter().position(|z| z.uuid == uuid) {
                    let old_zone = self.board.zones.remove(idx);
                    Ok(Some(PcbCommand::AddZone { zone: old_zone }))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
