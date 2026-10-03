use iced::Task;
use oxide_types::pcb::PcbBoard;
use std::path::PathBuf;

use super::export;
use super::super::super::*;

impl Oxide {
    /// Handle Forward ECO: Update PCB from Schematic Netlist & Components.
    pub(crate) fn handle_update_pcb_from_schematic(&mut self) -> Task<Message> {
        let (ctx, issues) = match export::build_export_scope(&self.document_state) {
            Some(c) => c,
            None => {
                crate::diagnostics::log_warning(
                    "ECO: No active schematic or project loaded to update PCB from.",
                );
                return Task::none();
            }
        };
        export::log_stitch_issues(&self.document_state, &ctx, &issues);

        let Some(netlist) = &ctx.netlist else {
            crate::diagnostics::log_warning(
                "ECO: No connectivity netlist could be derived from schematic.",
            );
            return Task::none();
        };

        let mut schematic_components = Vec::new();
        for sheet in &ctx.sheets {
            for sym in &sheet.schematic.symbols {
                schematic_components.push((
                    sym.reference.clone(),
                    sym.value.clone(),
                    sym.footprint.clone(),
                ));
            }
        }

        // Target active PCB engine, or primary open PCB engine, or create a new PCB tab
        let pcb_path = self
            .document_state
            .tabs
            .get(self.document_state.active_tab)
            .and_then(|t| match t.kind {
                crate::app::TabKind::Pcb => Some(t.path.clone()),
                _ => None,
            })
            .or_else(|| self.document_state.pcb_engines.keys().next().cloned())
            .unwrap_or_else(|| {
                // Synthesize a PCB path alongside the active project or schematic
                if let Some(proj) = self.document_state.active_document_project() {
                    proj.dir().join("board.snxpcb")
                } else if let Some(active) = &self.document_state.active_path {
                    active.with_extension("snxpcb")
                } else {
                    PathBuf::from("board.snxpcb")
                }
            });

        if !self.document_state.pcb_engines.contains_key(&pcb_path) {
            let board = PcbBoard::default();
            let engine = oxide_engine::pcb::PcbEngine::new(board);
            self.document_state
                .pcb_engines
                .insert(pcb_path.clone(), engine);

            let tab_title = pcb_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("board.snxpcb")
                .to_string();
            self.document_state.tabs.push(crate::app::TabInfo {
                title: tab_title,
                path: pcb_path.clone(),
                cached_document: None,
                dirty: true,
                project_id: self.document_state.active_project,
                kind: crate::app::TabKind::Pcb,
            });
            self.document_state.active_tab = self.document_state.tabs.len() - 1;
        }

        if let Some(engine) = self.document_state.pcb_engines.get_mut(&pcb_path) {
            let report = oxide_net::EcoEngine::diff_schematic_to_pcb(
                netlist,
                engine.board(),
                &schematic_components,
            );
            let action_count = report.len();
            oxide_net::EcoEngine::apply_eco(engine.board_mut(), &report);

            crate::diagnostics::log_info(format!(
                "ECO: Applied {} engineering change orders to PCB '{}'. Synchronized {} components.",
                action_count,
                pcb_path.display(),
                schematic_components.len()
            ));

            // Focus the PCB tab
            if let Some(idx) = self
                .document_state
                .tabs
                .iter()
                .position(|t| matches!(t.kind, crate::app::TabKind::Pcb) && t.path == pcb_path)
            {
                self.document_state.active_tab = idx;
            }
        }

        Task::none()
    }
}
