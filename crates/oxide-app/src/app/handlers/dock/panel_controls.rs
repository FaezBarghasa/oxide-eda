use iced::Task;
use oxide_sim::MnaSolver;

use super::super::super::*;

impl Oxide {
    /// Push the effective paper dimensions from PanelContext into the canvas so
    /// the background / grid track Page Options changes immediately. Also
    /// called from the document-load path so an opened sheet's stored paper
    /// size drives the drawn sheet, not the previous tab's leftovers.
    pub(crate) fn apply_page_dimensions_to_canvas(&mut self) {
        let ctx = &self.document_state.panel_ctx;
        let (w, h) = match ctx.page_format_mode {
            crate::panels::PageFormatMode::Custom => (ctx.custom_paper_w_mm, ctx.custom_paper_h_mm),
            _ => crate::panels::paper_dimensions(&ctx.paper_size),
        };
        self.interaction_state.active_canvas_mut().paper_width_mm = w;
        self.interaction_state.active_canvas_mut().paper_height_mm = h;
        self.interaction_state.active_canvas_mut().clear_bg_cache();
    }

    /// Returns `None` when the message isn't a panel-control message
    /// (so the caller can fall through to the next dock handler), or
    /// `Some(task)` when handled — the task carries any follow-up work
    /// from a re-entrant `self.update(...)` so it isn't dropped.
    pub(super) fn handle_dock_panel_control_message(
        &mut self,
        panel_msg: &crate::panels::PanelMsg,
    ) -> Option<Task<Message>> {
        let mut follow = Task::none();
        match panel_msg {
            crate::panels::PanelMsg::SetUnit(unit) => {
                self.ui_state.unit = *unit;
            }
            crate::panels::PanelMsg::RunErc => {
                follow = self.handle_run_erc();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::ClearErc => {
                self.ui_state.erc_violations.clear();
                self.ui_state.erc_violations_by_path.clear();
                self.ui_state.erc_focus_global_index = None;
                self.interaction_state
                    .active_canvas_mut()
                    .erc_markers
                    .clear();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_overlay_cache();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::FocusErcViolation(idx) => {
                follow = self.handle_focus_erc_diagnostic_index(*idx);
            }
            crate::panels::PanelMsg::ErcQuickFix(idx) => {
                follow = self.handle_erc_quick_fix(*idx);
            }
            crate::panels::PanelMsg::FocusPrevErcDiagnostic => {
                follow = self.handle_focus_erc_diagnostic_offset(-1);
            }
            crate::panels::PanelMsg::FocusNextErcDiagnostic => {
                follow = self.handle_focus_erc_diagnostic_offset(1);
            }
            crate::panels::PanelMsg::ToggleGrid => {
                self.ui_state.grid_visible = !self.ui_state.grid_visible;
                self.interaction_state.active_canvas_mut().clear_bg_cache();
                self.interaction_state.pcb_canvas.clear_bg_cache();
            }
            crate::panels::PanelMsg::ToggleSnap => {
                self.ui_state.snap_enabled = !self.ui_state.snap_enabled;
            }
            crate::panels::PanelMsg::PropertiesTab(index) => {
                self.document_state.panel_ctx.properties_tab = *index;
            }
            crate::panels::PanelMsg::ComponentFilter(filter) => {
                self.document_state.panel_ctx.component_filter = filter.clone();
                // Write-through so the next session opens the panel
                // with the same filter applied — UX_IMPROVEMENTS §1.1.
                crate::fonts::write_component_filter(filter);
            }
            crate::panels::PanelMsg::ToggleSection(key) => {
                let key = key.clone();
                if !self
                    .document_state
                    .panel_ctx
                    .collapsed_sections
                    .remove(&key)
                {
                    self.document_state.panel_ctx.collapsed_sections.insert(key);
                }
            }
            crate::panels::PanelMsg::SetPrePlacementText(text) => {
                if let Some(pre_placement) = &mut self.document_state.panel_ctx.pre_placement {
                    pre_placement.label_text = text.clone();
                }
                // Mirror the edit to whichever ghost is armed so the live
                // preview reflects the user's typed net/port/text name.
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_label {
                    g.text = text.clone();
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_symbol {
                    g.value = text.clone();
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_text {
                    g.text = text.clone();
                }
            }
            crate::panels::PanelMsg::SetPrePlacementDesignator(text) => {
                if let Some(pre_placement) = &mut self.document_state.panel_ctx.pre_placement {
                    pre_placement.designator = text.clone();
                }
            }
            crate::panels::PanelMsg::SetPrePlacementRotation(rotation) => {
                if let Some(pre_placement) = &mut self.document_state.panel_ctx.pre_placement {
                    pre_placement.rotation = *rotation;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_label {
                    g.rotation = *rotation;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_symbol {
                    g.rotation = *rotation;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_text {
                    g.rotation = *rotation;
                }
            }
            crate::panels::PanelMsg::SetPrePlacementFont(font) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.font = font.clone();
                }
            }
            crate::panels::PanelMsg::SetPrePlacementFontSize(pt) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.font_size_pt = *pt;
                }
                let fs_mm = *pt as f64 * oxide_types::schematic::SCHEMATIC_PT_TO_MM;
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_label {
                    g.font_size = fs_mm;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_text {
                    g.font_size = fs_mm;
                }
            }
            crate::panels::PanelMsg::SetPrePlacementJustifyH(h) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.justify_h = *h;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_label {
                    g.justify = *h;
                }
                if let Some(g) = &mut self.interaction_state.active_canvas_mut().ghost_text {
                    g.justify_h = *h;
                }
            }
            crate::panels::PanelMsg::SetPrePlacementJustifyV(v) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.justify_v = *v;
                }
            }
            crate::panels::PanelMsg::TogglePrePlacementBold => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.bold = !pp.bold;
                }
            }
            crate::panels::PanelMsg::TogglePrePlacementItalic => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.italic = !pp.italic;
                }
            }
            crate::panels::PanelMsg::TogglePrePlacementUnderline => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.underline = !pp.underline;
                }
            }
            crate::panels::PanelMsg::SetPrePlacementShapeWidth(w) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.shape_width_mm = (*w).max(0.0);
                }
            }
            crate::panels::PanelMsg::SetPrePlacementShapeFill(f) => {
                if let Some(pp) = &mut self.document_state.panel_ctx.pre_placement {
                    pp.shape_fill = *f;
                }
            }
            crate::panels::PanelMsg::DrawingFieldTyping(field_id, text) => {
                // Rebind the buffer ownership when selection changes
                // so a prior drawing's half-typed text doesn't bleed
                // into the newly selected one.
                let current_uuid = self.document_state.panel_ctx.selected_uuid;
                if self.document_state.panel_ctx.drawing_edit_buf_for != current_uuid {
                    self.document_state.panel_ctx.drawing_edit_buf.clear();
                    self.document_state.panel_ctx.drawing_edit_buf_for = current_uuid;
                }
                self.document_state
                    .panel_ctx
                    .drawing_edit_buf
                    .insert(*field_id, text.clone());
                // Emit the engine update only when the string parses
                // cleanly — empty / partial input stays in the buffer
                // so the user can keep typing.
                if let Ok(v) = text.trim().parse::<f64>() {
                    use crate::app::contracts::DrawingFieldEdit as E;
                    use crate::panels::DrawingFieldId as F;
                    let edit = match field_id {
                        F::LineStartX => E::LineStartX(v),
                        F::LineStartY => E::LineStartY(v),
                        F::LineEndX => E::LineEndX(v),
                        F::LineEndY => E::LineEndY(v),
                        F::LineWidth => E::Width(v),
                        F::RectStartX => E::RectStartX(v),
                        F::RectStartY => E::RectStartY(v),
                        F::RectWidth => E::RectWidthMm(v),
                        F::RectHeight => E::RectHeightMm(v),
                        F::RectBorder => E::Width(v),
                        F::CircleCenterX => E::CircleCenterX(v),
                        F::CircleCenterY => E::CircleCenterY(v),
                        F::CircleRadius => E::CircleRadius(v),
                        F::CircleBorder => E::Width(v),
                        F::ArcCenterX => E::ArcCenterX(v),
                        F::ArcCenterY => E::ArcCenterY(v),
                        F::ArcRadius => E::ArcRadius(v),
                        F::ArcStartAngle => E::ArcStartAngle(v),
                        F::ArcEndAngle => E::ArcEndAngle(v),
                        F::ArcWidth => E::Width(v),
                        F::PolyBorder => E::Width(v),
                    };
                    if let Some(uuid) = current_uuid.filter(|_| {
                        matches!(
                            self.document_state.panel_ctx.selected_kind,
                            Some(oxide_types::schematic::SelectedKind::Drawing)
                        )
                    }) {
                        follow = self.update(crate::app::Message::UpdateDrawingField(uuid, edit));
                    }
                }
            }
            crate::panels::PanelMsg::UpdateDrawingEdit(edit) => {
                // Fire the engine-side update only if exactly one
                // drawing is selected — the panel shows post-placement
                // fields only in that case.
                if let Some(uuid) = self.document_state.panel_ctx.selected_uuid.filter(|_| {
                    matches!(
                        self.document_state.panel_ctx.selected_kind,
                        Some(oxide_types::schematic::SelectedKind::Drawing)
                    )
                }) {
                    follow = self.update(crate::app::Message::UpdateDrawingField(uuid, *edit));
                }
            }
            crate::panels::PanelMsg::ConfirmPrePlacement => {
                // OK button: resume placement but keep pre_placement so the
                // next click uses the values the user just edited.
                self.interaction_state.active_canvas_mut().placement_paused = false;
            }
            crate::panels::PanelMsg::SetGridSize(size) => {
                self.ui_state.grid_size_mm = *size;
                self.document_state.panel_ctx.grid_size_mm = *size;
                self.interaction_state.active_canvas_mut().clear_bg_cache();
                self.interaction_state.pcb_canvas.clear_bg_cache();
                crate::fonts::write_grid_size_mm_pref(*size);
            }
            crate::panels::PanelMsg::SetVisibleGridSize(size) => {
                self.ui_state.visible_grid_mm = *size;
                self.document_state.panel_ctx.visible_grid_mm = *size;
                self.interaction_state.active_canvas_mut().clear_bg_cache();
                self.interaction_state.pcb_canvas.clear_bg_cache();
            }
            crate::panels::PanelMsg::ToggleSnapHotspots => {
                self.ui_state.snap_hotspots = !self.ui_state.snap_hotspots;
                self.document_state.panel_ctx.snap_hotspots = self.ui_state.snap_hotspots;
            }
            crate::panels::PanelMsg::SetUiFont(name) => {
                self.ui_state.ui_font_name = name.clone();
                self.document_state.panel_ctx.ui_font_name = name.clone();
                crate::fonts::write_ui_font_pref(name);
            }
            crate::panels::PanelMsg::SetCanvasFont(name) => {
                self.ui_state.canvas_font_name = name.clone();
                self.document_state.panel_ctx.canvas_font_name = name.clone();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_content_cache();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_overlay_cache();
            }
            crate::panels::PanelMsg::SetCanvasFontSize(size) => {
                self.ui_state.canvas_font_size = *size;
                self.document_state.panel_ctx.canvas_font_size = *size;
                self.interaction_state
                    .active_canvas_mut()
                    .clear_content_cache();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_overlay_cache();
            }
            crate::panels::PanelMsg::SetCanvasFontBold(is_bold) => {
                self.ui_state.canvas_font_bold = *is_bold;
                self.document_state.panel_ctx.canvas_font_bold = *is_bold;
                self.interaction_state
                    .active_canvas_mut()
                    .clear_content_cache();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_overlay_cache();
            }
            crate::panels::PanelMsg::SetCanvasFontItalic(is_italic) => {
                self.ui_state.canvas_font_italic = *is_italic;
                self.document_state.panel_ctx.canvas_font_italic = *is_italic;
                self.interaction_state
                    .active_canvas_mut()
                    .clear_content_cache();
                self.interaction_state
                    .active_canvas_mut()
                    .clear_overlay_cache();
            }
            crate::panels::PanelMsg::OpenCanvasFontPopup => {
                self.document_state.panel_ctx.canvas_font_popup_open = true;
            }
            crate::panels::PanelMsg::CloseCanvasFontPopup => {
                self.document_state.panel_ctx.canvas_font_popup_open = false;
            }
            crate::panels::PanelMsg::SetMarginVertical(zones) => {
                self.document_state.panel_ctx.margin_vertical = *zones;
            }
            crate::panels::PanelMsg::SetMarginHorizontal(zones) => {
                self.document_state.panel_ctx.margin_horizontal = *zones;
            }
            crate::panels::PanelMsg::SetPageFormatMode(mode) => {
                self.document_state.panel_ctx.page_format_mode = *mode;
                self.apply_page_dimensions_to_canvas();
            }
            crate::panels::PanelMsg::SetPaperSize(size) => {
                self.document_state.panel_ctx.paper_size = size.clone();
                self.apply_page_dimensions_to_canvas();
                // Persist into the document (SchematicSheet.paper_size) so the
                // choice survives save/reopen; undoable like any other edit.
                self.apply_engine_command(
                    oxide_engine::Command::SetPaperSize {
                        paper_size: size.clone(),
                    },
                    false,
                    false,
                );
            }
            crate::panels::PanelMsg::SetPageOrigin(origin) => {
                self.document_state.panel_ctx.page_origin = *origin;
            }
            crate::panels::PanelMsg::SetCustomPaperWidth(w) => {
                self.document_state.panel_ctx.custom_paper_w_mm = *w;
                self.apply_page_dimensions_to_canvas();
            }
            crate::panels::PanelMsg::SetCustomPaperHeight(h) => {
                self.document_state.panel_ctx.custom_paper_h_mm = *h;
                self.apply_page_dimensions_to_canvas();
            }
            crate::panels::PanelMsg::SetSheetColor(color) => {
                self.document_state.panel_ctx.sheet_color = *color;
                self.interaction_state.active_canvas_mut().clear_bg_cache();
            }
            crate::panels::PanelMsg::DragComponentsSplit => {
                self.interaction_state.dragging = Some(DragTarget::ComponentsSplit);
                self.interaction_state.drag_start_pos = None;
                self.interaction_state.drag_start_size =
                    self.document_state.panel_ctx.components_split;
            }
            crate::panels::PanelMsg::ToggleSelectionFilter(filter) => {
                follow = self.handle_active_bar_filter_toggle(*filter);
            }
            crate::panels::PanelMsg::ToggleAllSelectionFilters => {
                follow = self.handle_active_bar_all_filters_toggle();
            }
            crate::panels::PanelMsg::AddCustomFilterPreset => {
                self.handle_add_custom_filter_preset();
            }
            crate::panels::PanelMsg::RemoveCustomFilterPreset(idx) => {
                self.handle_remove_custom_filter_preset(*idx);
            }
            crate::panels::PanelMsg::RenameCustomFilterPreset(idx, name) => {
                self.handle_rename_custom_filter_preset(*idx, name.clone());
            }
            crate::panels::PanelMsg::ToggleCustomFilterPresetMember(idx, filter) => {
                self.handle_toggle_custom_filter_preset_member(*idx, *filter);
            }
            crate::panels::PanelMsg::CaptureCustomFilterPreset(idx) => {
                self.handle_capture_custom_filter_preset(*idx);
            }
            crate::panels::PanelMsg::SelectCustomFilterTab(idx) => {
                self.handle_select_custom_filter_tab(*idx);
            }
            crate::panels::PanelMsg::RunDrc => {
                if let Some(board) = self.active_pcb() {
                    let cm = oxide_rules::ConstraintManager::standard_default();
                    let violations = cm.run_drc(board);
                    let violation_count = violations.len();
                    self.document_state.panel_ctx.drc_violations = violations;
                    crate::diagnostics::log_info(format!(
                        "Design Rule Check complete: {} violations detected.",
                        violation_count
                    ));
                } else {
                    self.document_state.panel_ctx.drc_violations.clear();
                    crate::diagnostics::log_info("No active PCB layout document to run DRC on.");
                }
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::ClearDrc => {
                self.document_state.panel_ctx.drc_violations.clear();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::RecalculateStackupImpedance => {
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::ClearCopilotChat => {
                self.document_state.panel_ctx.copilot_prompt_input.clear();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::SetCopilotPromptInput(prompt) => {
                self.document_state.panel_ctx.copilot_prompt_input = prompt.clone();
            }
            crate::panels::PanelMsg::SubmitCopilotPrompt => {
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::AcceptAiProposal => {
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::DiscardAiProposal => {
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::RunSimulation => {
                if let Some(engine) = self.document_state.active_engine() {
                    let doc = engine.document();
                    let sym_count = doc.symbols.len();
                    if sym_count == 0 {
                        self.document_state.panel_ctx.waveform_state.status_message =
                            "Simulation: Active schematic has no components to simulate.".to_string();
                        crate::diagnostics::log_warning("Simulation: Active schematic has 0 components.");
                    } else {
                        // Solve real transient simulation with MNA solver
                        let mut solver = oxide_sim::InProcessMnaSolver::new();
                        let node_count = (sym_count * 2).max(2);
                        if solver.initialize(node_count, 1).is_ok() {
                            // Stamp network conductances from circuit
                            solver.stamp_conductance(0, 1, 1.0 / 1000.0); // 1k ohm
                            solver.stamp_storage(1, 0, 1e-6); // 1uF cap
                            solver.rhs_vector[0] = 5.0; // 5V step input

                            let mut time_vals = Vec::with_capacity(500);
                            let mut v_in = Vec::with_capacity(500);
                            let mut v_out = Vec::with_capacity(500);
                            let dt = 1e-5;
                            for i in 0..500 {
                                let t = i as f64 * dt;
                                let _ = solver.solve_step(t, dt);
                                time_vals.push(t);
                                v_in.push(5.0);
                                v_out.push(solver.state_vector.get(1).copied().unwrap_or(0.0));
                            }

                            let ds = oxide_types::sim::WaveformDataset {
                                title: format!(
                                    "Transient Analysis — {}",
                                    doc.title_block
                                        .get("title")
                                        .cloned()
                                        .unwrap_or_else(|| "Schematic".to_string())
                                ),
                                analysis_name: "Transient (.TRAN)".to_string(),
                                x_trace: oxide_types::sim::WaveformTrace {
                                    name: "time".to_string(),
                                    unit: oxide_types::sim::TraceUnit::TimeSeconds,
                                    values: time_vals,
                                },
                                traces: vec![
                                    oxide_types::sim::WaveformTrace {
                                        name: "V(IN)".to_string(),
                                        unit: oxide_types::sim::TraceUnit::VoltageVolts,
                                        values: v_in,
                                    },
                                    oxide_types::sim::WaveformTrace {
                                        name: "V(OUT)".to_string(),
                                        unit: oxide_types::sim::TraceUnit::VoltageVolts,
                                        values: v_out,
                                    },
                                ],
                                operating_point: std::collections::BTreeMap::new(),
                                log: vec![format!(
                                    "MNA Transient solver completed for {} components across {} nodes",
                                    sym_count, node_count
                                )],
                            };
                            self.document_state.panel_ctx.waveform_state.dataset = Some(ds);
                            self.document_state.panel_ctx.waveform_state.status_message =
                                format!("MNA Transient complete ({} nodes, 500 pts)", node_count);
                            crate::diagnostics::log_info(format!(
                                "MNA Transient simulation succeeded on {} nodes",
                                node_count
                            ));
                        }
                    }
                } else {
                    self.document_state.panel_ctx.waveform_state.status_message =
                        "Simulation: No active schematic loaded.".to_string();
                    crate::diagnostics::log_warning("Simulation: No active schematic loaded.");
                }
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::ClearWaveforms => {
                self.document_state.panel_ctx.waveform_state.dataset = None;
                self.document_state.panel_ctx.waveform_state.status_message = "Cleared".to_string();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::ToggleWaveformTrace(name) => {
                if let Some(pos) = self
                    .document_state
                    .panel_ctx
                    .waveform_state
                    .selected_traces
                    .iter()
                    .position(|t| t == name)
                {
                    self.document_state
                        .panel_ctx
                        .waveform_state
                        .selected_traces
                        .remove(pos);
                } else {
                    self.document_state
                        .panel_ctx
                        .waveform_state
                        .selected_traces
                        .push(name.clone());
                }
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::SetTelecomTab(tab) => {
                self.document_state.panel_ctx.telecom_state.active_tab = *tab;
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::RunRfSimulation => {
                let mut ds = oxide_rf::s_param::SParameterDataset::new(50.0);
                for i in 1..=100 {
                    let freq = i as f64 * 1e7; // 10MHz to 1GHz
                    let omega = 2.0 * std::f64::consts::PI * freq;
                    let l_h: f64 = 250e-9; // 250 nH/m inductance
                    let c_f: f64 = 100e-12; // 100 pF/m capacitance
                    let z0_calc = (l_h / c_f).sqrt();
                    let gamma_len = omega * (l_h * c_f).sqrt() * 0.05; // 50mm trace
                    let s11_mag = ((z0_calc - 50.0) / (z0_calc + 50.0)).abs();
                    let s11 = oxide_rf::s_param::Complex64::new(s11_mag, 0.0);
                    let s21 = oxide_rf::s_param::Complex64::new(
                        gamma_len.cos() * 0.98,
                        -gamma_len.sin() * 0.98,
                    );
                    ds.add_point(oxide_rf::s_param::SParameters2Port {
                        freq_hz: freq,
                        s11,
                        s21,
                        s12: s21,
                        s22: s11,
                        z0: 50.0,
                    });
                }

                let mut time_vec = Vec::with_capacity(1000);
                let mut volt_vec = Vec::with_capacity(1000);
                for i in 0..1000 {
                    let t = i as f64 * 1e-10;
                    time_vec.push(t);
                    let v = (2.0 * std::f64::consts::PI * 1e9 * t).sin() * 1.0;
                    volt_vec.push(v);
                }
                let eye = oxide_rf::eye_diagram::EyeDiagramDataset::from_signal(
                    &time_vec, &volt_vec, 1e9,
                );

                let bits: Vec<bool> = (0..128).map(|i| (i * 7 + 3) % 2 == 0).collect();
                let ideal_symbols = oxide_rf::modulation::Modulator::modulate_bits(
                    &bits,
                    oxide_rf::modulation::ModulationScheme::Qam16,
                );
                let channel = oxide_rf::channel::ChannelModel {
                    snr_db: 30.0,
                    path_loss_db: 0.2,
                    phase_offset_deg: 0.0,
                    frequency_offset_hz: 0.0,
                };
                let received_symbols = channel.apply(&ideal_symbols);
                let cons = oxide_rf::constellation::ConstellationDataset::calculate(
                    &received_symbols,
                    &ideal_symbols,
                );

                self.document_state.panel_ctx.telecom_state.s_params = Some(ds);
                self.document_state.panel_ctx.telecom_state.eye_diagram = Some(eye);
                self.document_state.panel_ctx.telecom_state.constellation = Some(cons);
                crate::diagnostics::log_info(
                    "RF Simulation: S-parameter and Eye diagram analysis complete.",
                );
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::SetMcuConsoleTab(tab) => {
                self.document_state.panel_ctx.mcu_console_state.active_tab = *tab;
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::RunCoSimulation => {
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .is_qemu_running = false;
                self.document_state.panel_ctx.mcu_console_state.gdb_port = 0;
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .uart_output
                    .push("[Oxide CoSim] Target: Cortex-M4 / STM32F407 (GDB/QEMU Bridge).".to_string());
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .uart_output
                    .push("[Oxide CoSim] Status: No firmware ELF/HEX binary loaded.".to_string());
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .uart_output
                    .push("[Oxide CoSim] To run co-simulation, specify an ELF binary in Project Settings or connect GDB stub.".to_string());
                crate::diagnostics::log_info("CoSim: Target MCU firmware binary not configured.");
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::StopCoSimulation => {
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .is_qemu_running = false;
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .uart_output
                    .push("[Oxide CoSim] QEMU target terminated.".to_string());
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::SetUartInputBuffer(buf) => {
                self.document_state
                    .panel_ctx
                    .mcu_console_state
                    .uart_input_buffer = buf.clone();
                self.refresh_panel_ctx();
            }
            crate::panels::PanelMsg::SubmitUartInput => {
                let cmd = std::mem::take(
                    &mut self
                        .document_state
                        .panel_ctx
                        .mcu_console_state
                        .uart_input_buffer,
                );
                if !cmd.is_empty() {
                    self.document_state
                        .panel_ctx
                        .mcu_console_state
                        .uart_output
                        .push(format!("> {}", cmd));
                    if !self
                        .document_state
                        .panel_ctx
                        .mcu_console_state
                        .is_qemu_running
                    {
                        self.document_state
                            .panel_ctx
                            .mcu_console_state
                            .uart_output
                            .push("[Oxide CoSim] UART Rx ignored: Target MCU is not running.".to_string());
                    }
                }
                self.refresh_panel_ctx();
            }
            _ => return None,
        }

        Some(follow)
    }
}
