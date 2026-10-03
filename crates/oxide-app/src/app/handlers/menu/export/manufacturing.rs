use iced::Task;
use oxide_output::{
    CbrExporter, CbrOptions, CdrExporter, CdrOptions, CdrVersion, DraftsmanDocument, DwgExporter,
    DwgOptions, DwgVersion, DxfExporter, DxfOptions, ExcellonExporter, GerberExporter,
    GerberOptions, Ipc2581Options, PickAndPlaceExporter, PickAndPlaceOptions, export_ipc2581,
};
use oxide_types::atomic_io::{atomic_write, atomic_write_stream};
use oxide_types::pcb::PcbBoard;

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

                tokio::task::spawn_blocking(move || {
                    let exporter = GerberExporter::new(GerberOptions::default());
                    let layers = exporter.export_board(&board).map_err(|e| e.to_string())?;

                    let mut written = 0;
                    for layer in &layers {
                        let out_path = dir.join(&layer.filename);
                        atomic_write(&out_path, layer.content.as_bytes()).map_err(|e| {
                            format!("Failed to write layer '{}': {}", layer.filename, e)
                        })?;
                        written += 1;
                    }

                    Ok((dir, written))
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = ExcellonExporter::default();
                    let outputs = exporter.export_board(&board).map_err(|e| e.to_string())?;

                    let mut total_holes = 0;
                    let mut total_tools = 0;
                    for out in &outputs {
                        let path = dir.join(&out.filename);
                        atomic_write(&path, out.content.as_bytes()).map_err(|e| e.to_string())?;
                        total_holes += out.hole_count;
                        total_tools += out.tool_count;
                    }
                    Ok((dir, total_holes, total_tools, outputs.len()))
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = PickAndPlaceExporter::new(PickAndPlaceOptions::default());
                    let content = exporter.export(&board).map_err(|e| e.to_string())?;
                    atomic_write(&path, content.as_bytes()).map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let output = export_ipc2581(&board, &Ipc2581Options::default())
                        .map_err(|e| e.to_string())?;
                    atomic_write(&path, output.xml_content.as_bytes()).map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let mut draftsman = DraftsmanDocument::new(&project_title);
                    draftsman.sync_with_board(&board);
                    let svg_content = draftsman
                        .generate_sheet_svg(0, &board)
                        .map_err(|e| e.to_string())?;
                    atomic_write(&path, svg_content.as_bytes()).map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = DxfExporter::new(DxfOptions::default());
                    atomic_write_stream(&path, |writer| {
                        exporter
                            .export_board_to_writer(&board, writer)
                            .map_err(|e| std::io::Error::other(e.to_string()))
                    })
                    .map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = DwgExporter::new(DwgOptions {
                        version: DwgVersion::R12Ac1009,
                        ..Default::default()
                    });
                    atomic_write_stream(&path, |writer| {
                        exporter
                            .export_dwg_to_writer(&board, writer)
                            .map_err(|e| std::io::Error::other(e.to_string()))
                    })
                    .map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = DwgExporter::new(DwgOptions {
                        version: DwgVersion::R12Ac1009,
                        is_template: true,
                        include_border: true,
                        ..Default::default()
                    });
                    atomic_write_stream(&path, |writer| {
                        exporter
                            .export_dwt_to_writer(&board, writer)
                            .map_err(|e| std::io::Error::other(e.to_string()))
                    })
                    .map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

                tokio::task::spawn_blocking(move || {
                    let exporter = CbrExporter::new(CbrOptions::default());
                    atomic_write_stream(&path, |writer| {
                        exporter
                            .export_bottom_copper_to_writer(&board, writer)
                            .map_err(|e| std::io::Error::other(e.to_string()))
                    })
                    .map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
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

    /// Export CorelDRAW Vector Drawing (`.cdr`) file.
    pub(crate) fn handle_export_cdr(&mut self) -> Task<Message> {
        let Some(board) = self.resolve_pcb_board() else {
            crate::diagnostics::log_warning("Export CDR: No active PCB layout found.");
            return Task::none();
        };

        Task::perform(
            async move {
                let file = rfd::AsyncFileDialog::new()
                    .set_title("Export CorelDRAW Vector Drawing (.cdr)")
                    .set_file_name("board.cdr")
                    .add_filter("CorelDRAW Drawing (*.cdr)", &["cdr"])
                    .save_file()
                    .await
                    .map(|f| f.path().to_path_buf());

                let Some(path) = file else {
                    return Err("Cancelled by user".to_string());
                };

                tokio::task::spawn_blocking(move || {
                    let exporter = CdrExporter::new(CdrOptions {
                        version: CdrVersion::V3_0,
                        ..Default::default()
                    });
                    atomic_write_stream(&path, |writer| {
                        exporter
                            .export_board_to_writer(&board, writer)
                            .map_err(|e| std::io::Error::other(e.to_string()))
                    })
                    .map_err(|e| e.to_string())?;
                    Ok(path)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |res| match res {
                Ok(path) => {
                    crate::diagnostics::log_info(format!(
                        "Export CDR: Successfully generated CorelDRAW .cdr file at {}",
                        path.display()
                    ));
                    Message::Noop
                }
                Err(e) => {
                    if e != "Cancelled by user" {
                        crate::diagnostics::log_warning(format!("Export CDR failed: {e}"));
                    }
                    Message::Noop
                }
            },
        )
    }
}

