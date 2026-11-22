use iced::Task;

use super::super::super::menu_bar::MenuMessage;
use super::super::super::*;

impl Oxide {
    pub(super) fn handle_menu_route_command(&mut self, msg: &MenuMessage) -> Option<Task<Message>> {
        match msg {
            MenuMessage::RouteInteractive => {
                self.interaction_state.current_tool = Tool::Wire;
                self.interaction_state
                    .pcb_canvas
                    .set_routing_mode(oxide_router::interactive::RoutingMode::PushAndShove);
                crate::diagnostics::log_info("Interactive Routing: Push-and-Shove enabled");
                Some(Task::none())
            }
            MenuMessage::RouteDiffPair => {
                self.interaction_state.current_tool = Tool::Wire;
                self.interaction_state
                    .pcb_canvas
                    .set_routing_mode(oxide_router::interactive::RoutingMode::DifferentialPair);
                crate::diagnostics::log_info("Interactive Routing: Differential Pair mode enabled");
                Some(Task::none())
            }
            MenuMessage::RouteMulti => {
                self.interaction_state.current_tool = Tool::Wire;
                self.interaction_state
                    .pcb_canvas
                    .set_routing_mode(oxide_router::interactive::RoutingMode::WalkAround);
                crate::diagnostics::log_info("Interactive Routing: Walkaround / Multi-route enabled");
                Some(Task::none())
            }
            MenuMessage::RouteLengthTuning => {
                self.interaction_state.current_tool = Tool::Wire;
                self.interaction_state
                    .pcb_canvas
                    .enable_length_tuning(15_000, 500);
                crate::diagnostics::log_info(
                    "Interactive Length Tuning enabled (Target: 15.00mm, Tol: ±0.50mm)",
                );
                Some(Task::none())
            }
            MenuMessage::RouteAutoRoute => {
                if let Some(mut board) = self.active_pcb().cloned() {
                    let rules =
                        std::sync::Arc::new(oxide_rules::ConstraintManager::standard_default());
                    let spatial =
                        std::sync::Arc::new(oxide_router::SpatialIndex::build(&board));
                    let mut autorouter =
                        oxide_router::topology::TopologicalAutorouter::new(rules, spatial);
                    let _ = autorouter.build_topological_map(&board);
                    let net_ids: Vec<u32> = board.nets.iter().map(|n| n.number).collect();
                    let results = autorouter.route_board(&mut board, &net_ids);
                    let count = results.len();
                    crate::diagnostics::log_info(format!(
                        "Topological autorouting complete: {count} routes established."
                    ));
                    if let Some(engine) = self.active_pcb_engine_mut() {
                        *engine.board_mut() = board;
                    }
                    self.sync_pcb_canvas_from_visible_board();
                    self.interaction_state.pcb_canvas.clear_content_cache();
                }
                Some(Task::none())
            }
            MenuMessage::RouteFanoutBga => {
                if let Some(mut board) = self.active_pcb().cloned() {
                    let mut fanouts_added = 0;
                    for fp in &board.footprints {
                        if fp.pads.len() >= 8 {
                            for pad in &fp.pads {
                                if let Some(pad_net) = &pad.net {
                                    if pad_net.number > 0 {
                                        let px = fp.position.x + pad.position.x;
                                        let py = fp.position.y + pad.position.y;
                                        let via_x = px + 0.35;
                                        let via_y = py + 0.35;
                                        board.segments.push(oxide_types::pcb::Segment {
                                            uuid: uuid::Uuid::new_v4(),
                                            start: oxide_types::pcb::Point::new(px, py),
                                            end: oxide_types::pcb::Point::new(via_x, via_y),
                                            width: 0.15,
                                            layer: "F.Cu".to_string(),
                                            net: pad_net.number,
                                        });
                                        board.vias.push(oxide_types::pcb::Via {
                                            uuid: uuid::Uuid::new_v4(),
                                            position: oxide_types::pcb::Point::new(via_x, via_y),
                                            diameter: 0.45,
                                            drill: 0.2,
                                            layers: vec!["F.Cu".to_string(), "B.Cu".to_string()],
                                            net: pad_net.number,
                                            via_type: oxide_types::pcb::ViaType::Through,
                                            via_span: None,
                                        });
                                        fanouts_added += 1;
                                    }
                                }
                            }
                        }
                    }
                    crate::diagnostics::log_info(format!(
                        "BGA escape fanout complete: {fanouts_added} escape vias generated."
                    ));
                    if let Some(engine) = self.active_pcb_engine_mut() {
                        *engine.board_mut() = board;
                    }
                    self.sync_pcb_canvas_from_visible_board();
                    self.interaction_state.pcb_canvas.clear_content_cache();
                }
                Some(Task::none())
            }
            MenuMessage::RouteTeardrops => {
                if let Some(mut board) = self.active_pcb().cloned() {
                    let added =
                        oxide_router::copper_pour::TeardropGenerator::apply_teardrops_to_board(
                            &mut board,
                        );
                    crate::diagnostics::log_info(format!(
                        "Applied {added} teardrop fillets across board."
                    ));
                    if let Some(engine) = self.active_pcb_engine_mut() {
                        *engine.board_mut() = board;
                    }
                    self.sync_pcb_canvas_from_visible_board();
                    self.interaction_state.pcb_canvas.clear_content_cache();
                }
                Some(Task::none())
            }
            MenuMessage::RouteUnrouteAll => {
                if let Some(engine) = self.active_pcb_engine_mut() {
                    engine.board_mut().segments.clear();
                    engine.board_mut().vias.clear();
                    crate::diagnostics::log_info("Unrouted all board tracks and vias.");
                }
                self.sync_pcb_canvas_from_visible_board();
                self.interaction_state.pcb_canvas.clear_content_cache();
                Some(Task::none())
            }
            MenuMessage::RouteUnrouteNet => {
                let net_to_unroute = self.interaction_state.pcb_canvas.routing_net;
                if let Some(engine) = self.active_pcb_engine_mut() {
                    if net_to_unroute > 0 {
                        engine
                            .board_mut()
                            .segments
                            .retain(|s| s.net != net_to_unroute);
                        engine
                            .board_mut()
                            .vias
                            .retain(|v| v.net != net_to_unroute);
                        crate::diagnostics::log_info(format!("Unrouted net {net_to_unroute}."));
                    }
                }
                self.sync_pcb_canvas_from_visible_board();
                self.interaction_state.pcb_canvas.clear_content_cache();
                Some(Task::none())
            }
            MenuMessage::RouteCorner45 => {
                self.interaction_state
                    .pcb_canvas
                    .set_corner_mode(oxide_router::CornerMode::FortyFiveDeg);
                crate::diagnostics::log_info("Corner mode set to 45° Mitred");
                Some(Task::none())
            }
            MenuMessage::RouteCornerRounded => {
                self.interaction_state
                    .pcb_canvas
                    .set_corner_mode(oxide_router::CornerMode::FortyFiveArc);
                crate::diagnostics::log_info("Corner mode set to Rounded (Arc)");
                Some(Task::none())
            }
            _ => None,
        }
    }
}
