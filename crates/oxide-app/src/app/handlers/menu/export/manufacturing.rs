use iced::Task;
use oxide_output::{
    CbrExporter, CbrOptions, DraftsmanDocument, DwgExporter, DwgOptions, DwgVersion, DxfExporter,
    DxfOptions, ExcellonExporter, GerberExporter, GerberOptions, Ipc2581Options,
    PickAndPlaceExporter, PickAndPlaceOptions, export_ipc2581,
};
use oxide_types::pcb::PcbBoard;
use std::path::PathBuf;

use super::super::super::super::*;

impl Oxide {
    /// Helper to resolve the active or project PCB board.
    fn resolve_pcb_board(&self) -> Option<PcbBoard> {
        if let Some(engine) = self.active_pcb_engine() {
            return Some(engine.board().clone());
        }
        if let Some((_, engine)) = self.document_state.pcb_engines.iter().next() {
            return Some(engine.board().clone());
        }
        None
    }

    /// Export Gerber RS-274X / X2 layer archive.
    pub(crate) fn handle_export_gerber(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning(
                "Export Gerber: No active PCB layout found. Please open a .snxpcb layout or run 'Update PCB from Schematic' first.",
            );
            return Task::none();
        };

        Task::perform(
            async move {
                let dir = rfd::AsyncFileDialog::new()
                    .set_title("Select Folder for Gerber X2 Output")
                    .pick_folder()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(dir) = dir else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = GerberExporter::new(GerberOptions::default());
                let layers = exporter.export_board(&board).map_err(|e| e.to_string())?;

                let mut written = 0;
                for layer in &layers {
                    let out_path = dir.join(&layer.filename);
                    std::fs::write(&out_path, &layer.content).map_err(|e| {
                        format!("Failed to write layer '{}': {}", layer.filename, e)
                    })?;
                    written += 1;
                }

                Ok((dir, written))
            },
            |res| match res {
                Ok((dir, count)) => {
                    crate::diagnostics::log_info(format!(
                        "Export Gerber: Successfully wrote {} Gerber X2 layer files to {}",
                        count,
                        dir.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export Gerber failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export Excellon NC Drill file.
    pub(crate) fn handle_export_drill(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export Drill: No PCB layout or schematic found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let dir = rfd::AsyncFileDialog::new()
                    .set_title("Select Folder for NC Drill Files")
                    .pick_folder()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(dir) = dir else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = ExcellonExporter::default();
                let outputs = exporter.export_board(&board).map_err(|e| e.to_string())?;

                let mut total_holes = 0;
                let mut total_tools = 0;
                for out in &outputs {
                    let path = dir.join(&out.filename);
                    std::fs::write(&path, &out.content).map_err(|e| e.to_string())?;
                    total_holes += out.hole_count;
                    total_tools += out.tool_count;
                }
                Ok((dir, total_holes, total_tools, outputs.len()))
            },
            |res| match res {
                Ok((dir, holes, tools, files)) => {
                    crate::diagnostics::log_info(format!(
                        "Export Drill: Wrote {} drill file(s) with {} holes across {} tools to {}",
                        files,
                        holes,
                        tools,
                        dir.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export Drill failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export Pick-and-Place (Centroid CPL) CSV file.
    pub(crate) fn handle_export_pnp(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export Pick and Place: No PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Save Pick and Place (CPL) CSV")
                    .set_file_name("pick_and_place.csv")
                    .add_filter("CSV File", &["csv"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
                let content = exporter.export(&board).map_err(|e| e.to_string())?;

                std::fs::write(&path, content).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export Pick and Place: Wrote placement file to {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!(
                            "Export Pick and Place failed: {e}"
                        ));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export IPC-2581 XML file.
    pub(crate) fn handle_export_ipc2581(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export IPC-2581: No PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Save IPC-2581 XML File")
                    .set_file_name("design.xml")
                    .add_filter("IPC-2581 XML", &["xml", "cvg"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let output = export_ipc2581(&board, &Ipc2581Options::default())
                    .map_err(|e| e.to_string())?;

                std::fs::write(&path, output.xml_content).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export IPC-2581: Successfully exported IPC-2581 Rev C XML to {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export IPC-2581 failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export Draftsman Engineering Drawing SVG sheet.
    pub(crate) fn handle_export_draftsman(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export Draftsman: No PCB layout found.");
            return Task::none();
        };

        let project_title = self
            .document_state
            .active_document_project()
            .map(|p| p.data.name.clone())
            .unwrap_or_else(|| "Oxide Project".to_string());

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Save Draftsman Engineering Drawing")
                    .set_file_name("draftsman_drawing.svg")
                    .add_filter("Scalable Vector Graphics", &["svg"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let mut draftsman = DraftsmanDocument::new(&project_title);
                draftsman.sync_with_board(&board);
                let svg_content = draftsman.generate_sheet_svg(0, &board)?;

                std::fs::write(&path, svg_content).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export Draftsman: Successfully generated ASME Y14.5 engineering drawing at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export Draftsman failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export AutoCAD Release 12 (AC1009) ASCII DXF file.
    pub(crate) fn handle_export_dxf(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export DXF: No active PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Export AutoCAD R12 DXF")
                    .set_file_name("board_layout.dxf")
                    .add_filter("AutoCAD R12 DXF (*.dxf)", &["dxf"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = DxfExporter::new(DxfOptions::default());
                let dxf_content = exporter.export_board(&board).map_err(|e| e.to_string())?;

                std::fs::write(&path, dxf_content).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export DXF: Successfully generated AutoCAD R12 DXF drawing at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export DXF failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export binary AutoCAD Release 12 DWG (`.dwg`) file.
    pub(crate) fn handle_export_dwg(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export DWG: No active PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Export AutoCAD DWG")
                    .set_file_name("board_layout.dwg")
                    .add_filter("AutoCAD Drawing (*.dwg)", &["dwg"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = DwgExporter::new(DwgOptions {
                    version: DwgVersion::R12Ac1009,
                    ..Default::default()
                });
                let dwg_bytes = exporter.export_dwg(&board).map_err(|e| e.to_string())?;

                std::fs::write(&path, dwg_bytes).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export DWG: Successfully generated AutoCAD DWG binary drawing at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export DWG failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export binary AutoCAD Drawing Template (`.dwt`) file with fabrication title block.
    pub(crate) fn handle_export_dwt(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export DWT: No active PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Export AutoCAD Drawing Template (DWT)")
                    .set_file_name("board_template.dwt")
                    .add_filter("AutoCAD Drawing Template (*.dwt)", &["dwt"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = DwgExporter::new(DwgOptions {
                    version: DwgVersion::R12Ac1009,
                    is_template: true,
                    include_border: true,
                    ..Default::default()
                });
                let dwt_bytes = exporter.export_dwt(&board).map_err(|e| e.to_string())?;

                std::fs::write(&path, dwt_bytes).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export DWT: Successfully generated AutoCAD Drawing Template at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export DWT failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Export Copper Bottom Routing (`.cbr`) legacy CAM file.
    pub(crate) fn handle_export_cbr(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export CBR: No active PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Export Copper Bottom Routing (.cbr)")
                    .set_file_name("board_bottom.cbr")
                    .add_filter("Copper Bottom Routing (*.cbr)", &["cbr"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                let exporter = CbrExporter::new(CbrOptions::default());
                let cbr_content = exporter.export_bottom_copper(&board).map_err(|e| e.to_string())?;

                std::fs::write(&path, cbr_content).map_err(|e| e.to_string())?;
                Ok(path)
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export CBR: Successfully generated Copper Bottom Routing file at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export CBR failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }

    /// Handle Forward ECO: Update PCB from Schematic Netlist & Components.
    pub(crate) fn handle_update_pcb_from_schematic(&mut self) -> Task<Message> {
        let (ctx, issues) = match super::build_export_scope(&self.document_state) {
            Some(c) => c,
            None => {
                crate::diagnostics::log_warning(
                    "ECO: No active schematic or project loaded to update PCB from.",
                );
                return Task::none();
            }
        };
        super::log_stitch_issues(&self.document_state, &ctx, &issues);

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
