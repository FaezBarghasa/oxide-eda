use std::sync::Arc;

use iced::event::Event;
use iced::mouse;
use iced::widget::canvas;
use iced::{Color, Rectangle, Renderer, Theme};
use oxide_gfx::primitive::circle::Circle as GfxCircle;
use oxide_gfx::primitive::line::LineSegment;
use oxide_gfx::primitive::polygon::GpuPolygon;
use oxide_gfx::scene::{CPU_PCB_DRAW_ORDER, DirtyFlags, Scene, SceneBucket};
use oxide_renderer::pcb::{PcbRenderer, PcbSnapshot};
use oxide_renderer::schematic::ViewRenderer;
use oxide_renderer::theme::ResolvedTheme;
use oxide_router::interactive::InteractiveRouter;
use oxide_router::{CornerMode, RouteSegment, ViaPlacement};

use crate::app::Message;
use crate::canvas::{Camera, CanvasEvent};

#[derive(Debug, Default)]
pub struct PcbCanvasState {
    panning: bool,
    last_pan_pos: Option<iced::Point>,
    pub pending_fit: Option<Rectangle>,
    pub click_start: Option<(f64, f64)>,
    pub move_origin: Option<(f64, f64)>,
    pub move_dragging: bool,
    pub last_click_time: Option<std::time::Instant>,
    pub last_click_world: Option<(f64, f64)>,
    pub ctrl_held: bool,
    pub shift_held: bool,
    // --- Interactive Push-and-Shove Routing Session ---
    pub active_router: Option<InteractiveRouter>,
    pub async_solver: Option<oxide_router::interactive::AsyncRoutingSolver>,
    pub routing_epoch: u64,
    pub live_route_preview: Vec<RouteSegment>,
    pub preview_vias: Vec<ViaPlacement>,
    pub routing_layer: u8,
    pub routing_net: u32,
    pub corner_mode: CornerMode,
    // --- Git-Native Visual Diff-Merge Viewport ---
    pub visual_diff: Option<crate::diff::PcbVisualDiff>,
    pub diff_mode: crate::diff::VisualDiffMode,
    pub split_swipe_x: f32,
    pub split_swipe_dragging: bool,
    // --- Precision Crosshair Alignment ---
    pub show_large_crosshair: bool,
    pub crosshair_pos: Option<iced::Point>,
    // --- Altium-Style Layer Visibilities ---
    pub single_layer_mode: bool,
    pub active_layer: Option<String>,
}

pub struct PcbCanvas {
    pub bg_cache: canvas::Cache,
    pub content_cache: canvas::Cache,
    pub content_cache_camera: std::cell::Cell<(f32, f32, f32)>,
    /// Cached GPU scene for the `gpu_render` path. The tessellated geometry is
    /// camera-independent — pan/zoom is applied by the shader's ortho
    /// projection at draw time, not baked into the instance data — so the scene
    /// survives across pan/zoom frames and is rebuilt only when the board
    /// content or theme changes. Invalidated by [`Self::clear_content_cache`]
    /// (the single content-changed signal every mutation site already calls)
    /// and [`Self::set_renderer_snapshot`]. `RefCell` because `gpu_scene` is
    /// called from `view()`, which holds `&self`. Stored behind an `Arc` so the
    /// cache hand-off and the shader primitive share the geometry by refcount
    /// bump — no per-frame deep copy.
    scene_cache: std::cell::RefCell<Option<Arc<Scene>>>,
    /// Monotonic id of the geometry in `scene_cache`, bumped on every
    /// invalidation (`clear_content_cache` / `set_renderer_snapshot`). Handed to
    /// the shader via [`Self::scene_generation`] so the GPU pipeline skips
    /// re-uploading unchanged geometry on pan/zoom. `Cell` for the same
    /// `&self`-in-`view()` reason as `scene_cache`.
    scene_generation: std::cell::Cell<u64>,
    /// The **sole** home of the PCB pan/zoom camera (ADR-0001 §A1: no shadow
    /// copies of widget state). `RefCell` gives interior mutability so
    /// `canvas::Program::update` — which only borrows `&self` — can mutate it,
    /// while `draw` and `view()` read it through the same cell. With no second
    /// copy in the widget `State`, the CPU background/grid, the CPU content
    /// path and the GPU shader program are read off one value and can never
    /// drift out of sync.
    camera: std::cell::RefCell<Camera>,
    pub pending_fit: std::cell::Cell<Option<Rectangle>>,
    pub grid_visible: bool,
    /// Effective (live) PCB GPU-render flag. When true, `draw` skips the CPU
    /// content tessellation (a `shader` stacked above draws it on the GPU) and
    /// `view()` mounts the shader stack. Synced from the Preferences toggle —
    /// updated immediately on draft change for live preview, committed/reverted
    /// on Save/Discard. Mirrors `UiState::pcb_gpu_render` (the saved value).
    pub gpu_render: bool,
    pub theme_bg: Color,
    pub theme_grid: Color,
    pub canvas_colors: oxide_types::theme::CanvasColors,
    pub renderer_snapshot: Option<PcbSnapshot>,
    pub visible_grid_mm: f64,
    pub show_pdn_heatmap: bool,
    pub pdn_mesh_cache: std::cell::RefCell<Option<oxide_gfx::pdn_heatmap::PdnMeshGrid>>,
}

impl Default for PcbCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl PcbCanvas {
    pub fn new() -> Self {
        let colors = oxide_types::theme::canvas_colors(oxide_types::theme::ThemeId::Oxide);
        Self {
            bg_cache: canvas::Cache::default(),
            content_cache: canvas::Cache::default(),
            content_cache_camera: std::cell::Cell::new((0.0, 0.0, 1.0)),
            scene_cache: std::cell::RefCell::new(None),
            scene_generation: std::cell::Cell::new(0),
            camera: std::cell::RefCell::new(Camera::default()),
            pending_fit: std::cell::Cell::new(None),
            grid_visible: true,
            gpu_render: false,
            theme_bg: crate::render_config::to_iced(&colors.background),
            theme_grid: crate::render_config::to_iced(&colors.grid),
            canvas_colors: colors,
            renderer_snapshot: None,
            visible_grid_mm: 2.54,
            show_pdn_heatmap: false,
            pdn_mesh_cache: std::cell::RefCell::new(None),
        }
    }

    pub fn active_renderer_snapshot(&self) -> Option<&PcbSnapshot> {
        self.renderer_snapshot.as_ref()
    }

    pub fn set_renderer_snapshot(&mut self, renderer_snapshot: Option<PcbSnapshot>) {
        self.renderer_snapshot = renderer_snapshot;
        // The board changed — the cached GPU scene no longer matches.
        *self.scene_cache.get_mut() = None;
        *self.scene_generation.get_mut() += 1;
    }

    pub fn clear_bg_cache(&mut self) {
        self.bg_cache.clear();
    }

    pub fn clear_content_cache(&mut self) {
        self.content_cache.clear();
        // GPU scene shares this content-changed signal (theme + geometry).
        *self.scene_cache.get_mut() = None;
        *self.scene_generation.get_mut() += 1;
    }

    pub fn fit_to_board(&mut self) {
        if let Some(snapshot) = self.active_renderer_snapshot()
            && let Some(bounds) = renderer_snapshot_bounds(snapshot)
        {
            self.pending_fit.set(Some(Rectangle::new(
                iced::Point::new(bounds.x, bounds.y),
                iced::Size::new(bounds.width, bounds.height),
            )));
        }
    }

    pub fn set_theme_colors(&mut self, bg: Color, grid: Color) {
        self.theme_bg = bg;
        self.theme_grid = grid;
        self.bg_cache.clear();
    }

    /// Current pan/zoom for the GPU path as
    /// `(offset_x_px, offset_y_px, scale_px_per_mm)`, read in `view()` from the
    /// single [`Self::camera`] home to build the shader program. Reads the same
    /// cell the CPU `draw` uses, so the GPU content stays frame-coherent with
    /// the CPU background + grid.
    pub fn live_camera(&self) -> (f32, f32, f32) {
        let camera = self.camera.borrow();
        (camera.offset.x, camera.offset.y, camera.scale)
    }

    pub fn zoom_in(&self) {
        let mut cam = self.camera.borrow_mut();
        let center = iced::Point::new(cam.offset.x + 400.0, cam.offset.y + 300.0);
        cam.zoom_at(center, 1.0, iced::Rectangle::default());
    }

    pub fn zoom_out(&self) {
        let mut cam = self.camera.borrow_mut();
        let center = iced::Point::new(cam.offset.x + 400.0, cam.offset.y + 300.0);
        cam.zoom_at(center, -1.0, iced::Rectangle::default());
    }

    /// Center camera on world coordinates (in mm).
    pub fn center_on_world(&self, world_x: f32, world_y: f32) {
        let mut cam = self.camera.borrow_mut();
        cam.center_on_world(world_x, world_y);
    }

    /// Build the `oxide_gfx` scene for a board snapshot. Shared by the CPU
    /// `draw` path and the GPU [`Self::gpu_scene`] path so both tessellate from
    /// identical instance data.
    pub fn build_scene(&self, snapshot: &PcbSnapshot) -> Scene {
        let mut scene = Scene::default();
        let theme = ResolvedTheme::from_canvas_colors(self.canvas_colors);
        PcbRenderer::build_scene(
            snapshot,
            &theme,
            DirtyFlags::LINES | DirtyFlags::CIRCLES | DirtyFlags::POLYGONS | DirtyFlags::OVERLAY,
            &mut scene,
        );

        // Component 5: GPU Viewport-Composited SI/PI Heatmaps
        if self.show_pdn_heatmap {
            let bounds = renderer_snapshot_bounds(snapshot).unwrap_or(Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            });

            let w = (bounds.width as f64).max(10.0);
            let h = (bounds.height as f64).max(10.0);
            let step = (w.min(h) / 20.0).max(1.0);

            let mut grid = oxide_gfx::pdn_heatmap::PdnMeshGrid::new(w, h, step, 3.3, 35.0);

            // Synthesize VRM source and load sinks from actual pads belonging to power nets
            let mut vrm_placed = false;
            let mut sink_count = 0;
            let min_x_f64 = bounds.x as f64;
            let min_y_f64 = bounds.y as f64;

            for pad in &snapshot.pads {
                if let Some(net_name) = &pad.net_name {
                    let net_upper = net_name.to_ascii_uppercase();
                    let is_pwr = net_upper.contains("VCC")
                        || net_upper.contains("VDD")
                        || net_upper.contains("3V3")
                        || net_upper.contains("5V")
                        || net_upper.contains("1V8")
                        || net_upper.contains("1V2")
                        || net_upper.contains("0V85")
                        || net_upper.contains("PWR")
                        || net_upper.contains("CORE");

                    if is_pwr {
                        let col = (((pad.center[0] as f64 - min_x_f64) / step).round() as usize)
                            .min(grid.cols.saturating_sub(1));
                        let row = (((pad.center[1] as f64 - min_y_f64) / step).round() as usize)
                            .min(grid.rows.saturating_sub(1));

                        if !vrm_placed {
                            // First detected power pin acts as VRM supply terminal
                            grid.set_voltage_source(col, row, 3.3);
                            vrm_placed = true;
                        } else if sink_count < 16 {
                            // Subsequent power pins act as distributed IC/PHY DC current sinks
                            grid.add_current_sink(col, row, 0.45);
                            sink_count += 1;
                        }
                    }
                }
            }

            // Fallback if no annotated power net pads are present
            if !vrm_placed {
                grid.set_voltage_source(0, 0, 3.3);
                if grid.cols > 1 && grid.rows > 1 {
                    grid.add_current_sink(grid.cols / 2, grid.rows / 2, 2.5);
                    grid.add_current_sink(grid.cols - 1, grid.rows - 1, 1.2);
                }
            }
            grid.solve_dc_drop(100);

            // Synthesize quad cells into overlay_polygons with semi-transparent screen blend
            let min_x = bounds.x;
            let min_y = bounds.y;
            for r in 0..(grid.rows - 1) {
                for c in 0..(grid.cols - 1) {
                    let idx = r * grid.cols + c;
                    let node = &grid.nodes[idx];
                    let drop_mv = ((grid.nominal_voltage_v - node.voltage_v) * 1e3).max(0.0);

                    // Colormap mapping with 0.45 alpha for copper plane blending
                    let (r_val, g_val, b_val) = if drop_mv < 5.0 {
                        (0.18f32, 0.80f32, 0.44f32) // Green
                    } else if drop_mv < 25.0 {
                        let factor = ((drop_mv - 5.0) / 20.0) as f32;
                        (
                            0.18 + factor * (0.95 - 0.18),
                            0.80 + factor * (0.77 - 0.80),
                            0.44 + factor * (0.06 - 0.44),
                        )
                    } else {
                        let factor = (((drop_mv - 25.0) / 25.0).clamp(0.0, 1.0)) as f32;
                        (
                            0.95 + factor * (0.91 - 0.95),
                            0.77 - factor * (0.77 - 0.30),
                            0.06 + factor * (0.24 - 0.06),
                        )
                    };

                    let x0 = min_x + (c as f64 * step) as f32;
                    let y0 = min_y + (r as f64 * step) as f32;
                    let x1 = x0 + step as f32;
                    let y1 = y0 + step as f32;

                    scene.overlay_polygons.push(GpuPolygon {
                        vertices: vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]],
                        fill_color: [r_val, g_val, b_val, 0.45],
                        stroke_color: Some([r_val, g_val, b_val, 0.60]),
                        stroke_width: 0.1,
                    });
                }
            }

            *self.pdn_mesh_cache.borrow_mut() = Some(grid);
        }

        scene
    }

    /// Build the scene for GPU rendering: the same geometry as the CPU path.
    /// Overlay primitives (active-layer zone highlight, selection highlight)
    /// stay in their own `Scene::overlay_*` fields rather than folding into
    /// the base buckets — [`crate::scene_shader::ScenePrimitive::draw`]
    /// composites them in a dedicated pass *after* every base bucket, so they
    /// always render on top, matching the CPU `draw_scene` overlay pass.
    /// Returns `None` when no board snapshot is loaded.
    pub fn gpu_scene(&self) -> Option<Arc<Scene>> {
        // Fast path: hand back the shared cached scene without re-tessellating.
        // `borrow().clone()` is an `Arc` refcount bump (on a miss it clones
        // `None`, which is free); the `borrow()` temporary is released before
        // the rebuild below takes a `borrow_mut()`. See [`Self::scene_cache`].
        if let Some(scene) = self.scene_cache.borrow().clone() {
            return Some(scene);
        }

        let snapshot = self.active_renderer_snapshot()?;
        let scene = Arc::new(self.build_scene(snapshot));
        *self.scene_cache.borrow_mut() = Some(Arc::clone(&scene));
        Some(scene)
    }

    /// Generation id of the scene [`Self::gpu_scene`] currently returns. Passed
    /// to the shader so the GPU pipeline can skip re-uploading unchanged
    /// geometry on pan/zoom. Read in `view()` right after `gpu_scene()`, so it
    /// reflects the same geometry that call returned (a rebuild bumps this at
    /// its invalidation site, never mid-`gpu_scene`).
    pub fn scene_generation(&self) -> u64 {
        self.scene_generation.get()
    }
}

fn color_from_rgba(rgba: [f32; 4]) -> Color {
    Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3])
}

fn world_to_screen(camera: &Camera, bounds: Rectangle, point: [f32; 2]) -> iced::Point {
    camera.world_to_screen(iced::Point::new(point[0], point[1]), bounds)
}

fn include_world_point(bounds: &mut Option<(f32, f32, f32, f32)>, x: f32, y: f32) {
    if let Some((min_x, min_y, max_x, max_y)) = bounds.as_mut() {
        *min_x = (*min_x).min(x);
        *min_y = (*min_y).min(y);
        *max_x = (*max_x).max(x);
        *max_y = (*max_y).max(y);
    } else {
        *bounds = Some((x, y, x, y));
    }
}

fn include_world_span(bounds: &mut Option<(f32, f32, f32, f32)>, x: f32, y: f32, radius: f32) {
    let r = radius.max(0.0);
    include_world_point(bounds, x - r, y - r);
    include_world_point(bounds, x + r, y + r);
}

fn renderer_snapshot_bounds(snapshot: &PcbSnapshot) -> Option<Rectangle> {
    let mut bounds: Option<(f32, f32, f32, f32)> = None;

    for trace in &snapshot.traces {
        let half_w = (trace.width_mm * 0.5).max(0.02);
        include_world_span(&mut bounds, trace.p0[0], trace.p0[1], half_w);
        include_world_span(&mut bounds, trace.p1[0], trace.p1[1], half_w);
    }

    for via in &snapshot.vias {
        include_world_span(
            &mut bounds,
            via.center[0],
            via.center[1],
            (via.diameter_mm * 0.5).max(0.02),
        );
    }

    for pad in &snapshot.pads {
        include_world_span(
            &mut bounds,
            pad.center[0],
            pad.center[1],
            (pad.size_mm[0].max(pad.size_mm[1]) * 0.5).max(0.02),
        );
    }

    for zone in &snapshot.zones {
        for vertex in &zone.vertices {
            include_world_point(&mut bounds, vertex[0], vertex[1]);
        }
    }

    for rule_area in &snapshot.rule_areas {
        for vertex in &rule_area.vertices {
            include_world_point(&mut bounds, vertex[0], vertex[1]);
        }
    }

    for line in &snapshot.ratsnest_lines {
        include_world_point(&mut bounds, line.p0[0], line.p0[1]);
        include_world_point(&mut bounds, line.p1[0], line.p1[1]);
    }

    for marker in &snapshot.drc_markers {
        include_world_span(
            &mut bounds,
            marker.center[0],
            marker.center[1],
            marker.radius_mm.max(0.02),
        );
    }

    bounds.map(|(min_x, min_y, max_x, max_y)| {
        Rectangle::new(
            iced::Point::new(min_x, min_y),
            iced::Size::new((max_x - min_x).max(0.1), (max_y - min_y).max(0.1)),
        )
    })
}

fn draw_dashed_line(
    frame: &mut canvas::Frame,
    p0: iced::Point,
    p1: iced::Point,
    width: f32,
    color: Color,
) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let length = (dx * dx + dy * dy).sqrt();
    if length <= 0.0001 {
        return;
    }

    let dash = 8.0;
    let gap = 5.0;
    let ux = dx / length;
    let uy = dy / length;
    let mut dist = 0.0;

    while dist < length {
        let seg_end = (dist + dash).min(length);
        let sp = iced::Point::new(p0.x + ux * dist, p0.y + uy * dist);
        let ep = iced::Point::new(p0.x + ux * seg_end, p0.y + uy * seg_end);
        let path = canvas::Path::line(sp, ep);
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_width(width)
                .with_color(color),
        );
        dist += dash + gap;
    }
}

fn draw_lines(
    frame: &mut canvas::Frame,
    lines: &[LineSegment],
    camera: &Camera,
    bounds: Rectangle,
) {
    for line in lines {
        let p0 = world_to_screen(camera, bounds, line.p0);
        let p1 = world_to_screen(camera, bounds, line.p1);
        let width = (line.width * camera.scale).max(0.5);
        let color = color_from_rgba(line.color);

        if line.is_dashed() {
            draw_dashed_line(frame, p0, p1, width, color);
        } else {
            let path = canvas::Path::line(p0, p1);
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_width(width)
                    .with_color(color),
            );
        }
    }
}

fn draw_circles(
    frame: &mut canvas::Frame,
    circles: &[GfxCircle],
    camera: &Camera,
    bounds: Rectangle,
) {
    for circle in circles {
        let center = world_to_screen(camera, bounds, circle.center);
        let radius = (circle.radius * camera.scale).max(0.5);
        let stroke_width = (circle.stroke_width * camera.scale).max(0.5);
        let color = color_from_rgba(circle.color);
        let path = canvas::Path::circle(center, radius);

        if circle.is_filled() {
            frame.fill(&path, color);
        } else {
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_width(stroke_width)
                    .with_color(color),
            );
        }
    }
}

fn draw_polygons(
    frame: &mut canvas::Frame,
    polygons: &[GpuPolygon],
    camera: &Camera,
    bounds: Rectangle,
) {
    for polygon in polygons {
        if polygon.vertices.len() < 3 {
            continue;
        }

        let points: Vec<iced::Point> = polygon
            .vertices
            .iter()
            .map(|vertex| world_to_screen(camera, bounds, *vertex))
            .collect();

        let path = canvas::Path::new(|builder| {
            builder.move_to(points[0]);
            for point in &points[1..] {
                builder.line_to(*point);
            }
            builder.close();
        });

        frame.fill(&path, color_from_rgba(polygon.fill_color));

        if let Some(stroke_color) = polygon.stroke_color {
            frame.stroke(
                &path,
                canvas::Stroke::default()
                    .with_width((polygon.stroke_width * camera.scale).max(0.5))
                    .with_color(color_from_rgba(stroke_color)),
            );
        }
    }
}

fn draw_scene(frame: &mut canvas::Frame, scene: &Scene, camera: &Camera, bounds: Rectangle) {
    // Walk the shared `CPU_PCB_DRAW_ORDER` so this CPU path and the GPU
    // `scene_shader` cannot silently drift apart; the `scene::order` parity test
    // diffs the two orders. Main geometry first, then the overlay pass.
    for &bucket in CPU_PCB_DRAW_ORDER {
        match bucket {
            SceneBucket::Lines => draw_lines(frame, &scene.lines, camera, bounds),
            SceneBucket::Circles => draw_circles(frame, &scene.circles, camera, bounds),
            SceneBucket::Polygons => draw_polygons(frame, &scene.polygons, camera, bounds),
            SceneBucket::OverlayLines => draw_lines(frame, &scene.overlay_lines, camera, bounds),
            SceneBucket::OverlayCircles => {
                draw_circles(frame, &scene.overlay_circles, camera, bounds)
            }
            SceneBucket::OverlayPolygons => {
                draw_polygons(frame, &scene.overlay_polygons, camera, bounds)
            }
            // The PCB CPU path emits no arc, text, or ERC buckets. Handled for
            // exhaustiveness so adding a Scene bucket forces a decision here.
            SceneBucket::Arcs
            | SceneBucket::Texts
            | SceneBucket::ErcMarkerLines
            | SceneBucket::ErcMarkerCircles
            | SceneBucket::ErcMarkerPolygons => {}
        }
    }
}

impl canvas::Program<Message> for PcbCanvas {
    type State = PcbCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        if let Some(target) = self.pending_fit.take() {
            state.pending_fit = Some(target);
        }

        if let Some(target) = state.pending_fit.take() {
            self.camera.borrow_mut().fit_rect(target, bounds);
            return Some(canvas::Action::publish(Message::CanvasEvent(
                CanvasEvent::CursorMoved,
            )));
        }

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let scroll_y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => *y,
                    mouse::ScrollDelta::Pixels { y, .. } => *y / 50.0,
                };
                if let Some(cursor_pos) = cursor.position_in(bounds)
                    && self
                        .camera
                        .borrow_mut()
                        .zoom_at(cursor_pos, scroll_y, bounds)
                {
                    return Some(
                        canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                            .and_capture(),
                    );
                }
            }
            Event::Keyboard(iced::keyboard::Event::ModifiersChanged(mods)) => {
                state.ctrl_held = mods.control();
                state.shift_held = mods.shift();
            }
            Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if let iced::keyboard::Key::Character(c) = key
                    && (c.as_str() == "`" || c.as_str() == "~")
                {
                    state.show_large_crosshair = !state.show_large_crosshair;
                    return Some(
                        canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                            .and_capture(),
                    );
                }
                if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) = key {
                    state.diff_mode = match state.diff_mode {
                        crate::diff::VisualDiffMode::UnifiedOverlay => {
                            crate::diff::VisualDiffMode::SplitSwipe
                        }
                        crate::diff::VisualDiffMode::SplitSwipe => {
                            crate::diff::VisualDiffMode::SideBySide
                        }
                        crate::diff::VisualDiffMode::SideBySide => {
                            crate::diff::VisualDiffMode::UnifiedOverlay
                        }
                    };
                    if state.split_swipe_x <= 0.0 {
                        state.split_swipe_x = bounds.width / 2.0;
                    }
                    return Some(
                        canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                            .and_capture(),
                    );
                }
                // Shift+S: Toggle Altium-Style Single-Layer Mode
                if let iced::keyboard::Key::Character(c) = key
                    && c.as_str().eq_ignore_ascii_case("s")
                    && modifiers.shift()
                {
                    state.single_layer_mode = !state.single_layer_mode;
                    return Some(
                        canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                            .and_capture(),
                    );
                }
                if let Some(router) = &mut state.active_router {
                    match key {
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                            state.routing_epoch += 1;
                            if modifiers.shift() {
                                state.corner_mode = router.hotkey_cycle_corner_mode();
                            } else {
                                router.hotkey_cycle_corner_style();
                            }
                            if let Some(cursor_pos) = cursor.position_in(bounds) {
                                let world =
                                    self.camera.borrow().screen_to_world(cursor_pos, bounds);
                                state.live_route_preview =
                                    router.on_mouse_move(oxide_router::geometry::Point2D::from_mm(
                                        world.x as f64,
                                        world.y as f64,
                                    ));
                            }
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c)
                            if c.as_str() == "*" || c.as_str().eq_ignore_ascii_case("v") =>
                        {
                            state.routing_epoch += 1;
                            let next_layer = if state.routing_layer == 0 { 1 } else { 0 };
                            if let Some(via) =
                                router.hotkey_drop_via_and_switch_layer(next_layer, 600, 300)
                            {
                                state.preview_vias.push(via);
                                state.routing_layer = next_layer;
                            }
                            if let Some(cursor_pos) = cursor.position_in(bounds) {
                                let world =
                                    self.camera.borrow().screen_to_world(cursor_pos, bounds);
                                state.live_route_preview =
                                    router.on_mouse_move(oxide_router::geometry::Point2D::from_mm(
                                        world.x as f64,
                                        world.y as f64,
                                    ));
                            }
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c)
                            if c.as_str().eq_ignore_ascii_case("a") && modifiers.shift() =>
                        {
                            state.routing_epoch += 1;
                            let cur_len = state
                                .live_route_preview
                                .iter()
                                .map(|s| s.length())
                                .sum::<i64>();
                            let target_len = (cur_len + 5000).max(10_000);
                            router.enable_length_tuning(target_len, 0);
                            state.async_solver = None;
                            if let Some(cursor_pos) = cursor.position_in(bounds) {
                                let world =
                                    self.camera.borrow().screen_to_world(cursor_pos, bounds);
                                state.live_route_preview =
                                    router.on_mouse_move(oxide_router::geometry::Point2D::from_mm(
                                        world.x as f64,
                                        world.y as f64,
                                    ));
                            }
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c)
                            if c.as_str().eq_ignore_ascii_case("b") && modifiers.shift() =>
                        {
                            state.routing_epoch += 1;
                            router.mode = oxide_router::interactive::RoutingMode::DifferentialPair;
                            state.async_solver = None;
                            if let Some(cursor_pos) = cursor.position_in(bounds) {
                                let world =
                                    self.camera.borrow().screen_to_world(cursor_pos, bounds);
                                state.live_route_preview =
                                    router.on_mouse_move(oxide_router::geometry::Point2D::from_mm(
                                        world.x as f64,
                                        world.y as f64,
                                    ));
                            }
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c)
                            if c.as_str() == "1" && modifiers.control() =>
                        {
                            let style = router.hotkey_cycle_corner_style();
                            state.corner_mode = match style {
                                oxide_router::interactive::session::CornerStyle::Arc45
                                | oxide_router::interactive::session::CornerStyle::FullArc => {
                                    oxide_router::CornerMode::FortyFiveArc
                                }
                                oxide_router::interactive::session::CornerStyle::RightAngle90 => {
                                    oxide_router::CornerMode::NinetyDeg
                                }
                                _ => oxide_router::CornerMode::FortyFiveDeg,
                            };
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c)
                            if c.as_str() == "2" && modifiers.control() =>
                        {
                            state.corner_mode = router.hotkey_cycle_corner_mode();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c) if c.as_str() == "1" => {
                            router.hotkey_increase_amplitude();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c) if c.as_str() == "2" => {
                            router.hotkey_decrease_amplitude();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c) if c.as_str() == "3" => {
                            router.hotkey_increase_pitch();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Character(c) if c.as_str() == "4" => {
                            router.hotkey_decrease_pitch();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                            state.active_router = None;
                            state.async_solver = None;
                            state.live_route_preview.clear();
                            state.preview_vias.clear();
                            return Some(
                                canvas::Action::publish(Message::CanvasEvent(
                                    CanvasEvent::CursorMoved,
                                ))
                                .and_capture(),
                            );
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe
                        && (cursor_pos.x - state.split_swipe_x).abs() <= 16.0
                    {
                        state.split_swipe_dragging = true;
                        return Some(
                            canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                                .and_capture(),
                        );
                    }

                    let world = self.camera.borrow().screen_to_world(cursor_pos, bounds);
                    let wx = world.x as f64;
                    let wy = world.y as f64;

                    // Double-click detection (300ms threshold)
                    let now = std::time::Instant::now();
                    if let (Some(last_time), Some(last_pos)) =
                        (state.last_click_time, state.last_click_world)
                    {
                        let dt = now.duration_since(last_time);
                        let dist = ((wx - last_pos.0).powi(2) + (wy - last_pos.1).powi(2)).sqrt();
                        if dt.as_millis() < 300 && dist < 3.0 {
                            state.last_click_time = None;
                            state.last_click_world = None;
                            state.click_start = None;
                            state.move_origin = None;
                            state.move_dragging = false;
                            return Some(canvas::Action::publish(Message::CanvasEvent(
                                CanvasEvent::DoubleClicked {
                                    world_x: wx,
                                    world_y: wy,
                                    screen_x: cursor_pos.x,
                                    screen_y: cursor_pos.y,
                                },
                            )));
                        }
                    }
                    state.last_click_time = Some(now);
                    state.last_click_world = Some((wx, wy));
                    state.click_start = Some((wx, wy));
                    state.move_origin = Some((wx, wy));
                    state.move_dragging = false;

                    if state.active_router.is_some() {
                        state.routing_epoch += 1;
                    }

                    let event = if state.ctrl_held {
                        CanvasEvent::CtrlClicked {
                            world_x: wx,
                            world_y: wy,
                        }
                    } else {
                        CanvasEvent::Clicked {
                            world_x: wx,
                            world_y: wy,
                        }
                    };

                    return Some(
                        canvas::Action::publish(Message::CanvasEvent(event)).and_capture(),
                    );
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.split_swipe_dragging = false;
                if state.move_dragging
                    && let Some((ox, oy)) = state.move_origin.take()
                    && let Some(cursor_pos) = cursor.position_in(bounds)
                {
                    let world = self.camera.borrow().screen_to_world(cursor_pos, bounds);
                    let dx = world.x as f64 - ox;
                    let dy = world.y as f64 - oy;
                    state.move_dragging = false;
                    state.click_start = None;
                    if dx.abs() > 0.01 || dy.abs() > 0.01 {
                        return Some(canvas::Action::publish(Message::CanvasEvent(
                            CanvasEvent::MoveSelected { dx, dy },
                        )));
                    }
                } else {
                    state.move_dragging = false;
                    state.click_start = None;
                    state.move_origin = None;
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                if matches!(button, mouse::Button::Right | mouse::Button::Middle) {
                    state.panning = true;
                    state.last_pan_pos = cursor.position_in(bounds);
                    return Some(canvas::Action::capture());
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                if matches!(button, mouse::Button::Right | mouse::Button::Middle) {
                    state.panning = false;
                    state.last_pan_pos = None;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    if state.panning
                        && let Some(last_pan_pos) = state.last_pan_pos
                    {
                        self.camera
                            .borrow_mut()
                            .pan(cursor_pos.x - last_pan_pos.x, cursor_pos.y - last_pan_pos.y);
                        state.last_pan_pos = Some(cursor_pos);
                        return Some(
                            canvas::Action::publish(Message::CanvasEvent(CanvasEvent::CursorMoved))
                                .and_capture(),
                        );
                    }

                    let (world, zoom_pct) = {
                        let camera = self.camera.borrow();
                        (
                            camera.screen_to_world(cursor_pos, bounds),
                            camera.zoom_percent(),
                        )
                    };

                    state.crosshair_pos = Some(cursor_pos);

                    if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                        if state.split_swipe_dragging {
                            state.split_swipe_x = cursor_pos.x.clamp(30.0, bounds.width - 30.0);
                        } else if state.split_swipe_x <= 0.0 {
                            state.split_swipe_x = bounds.width / 2.0;
                        }
                    }

                    if let Some((sx, sy)) = state.click_start {
                        let dx = world.x as f64 - sx;
                        let dy = world.y as f64 - sy;
                        if (dx * dx + dy * dy).sqrt() > 0.5 {
                            state.move_dragging = true;
                        }
                    }

                    if let Some(router) = &mut state.active_router {
                        let target = oxide_router::geometry::Point2D::from_mm(
                            world.x as f64,
                            world.y as f64,
                        );
                        let session_mode = router
                            .session
                            .as_ref()
                            .map(|s| s.mode)
                            .unwrap_or(router.mode);
                        let session_corner = router
                            .session
                            .as_ref()
                            .map(|s| s.corner_mode)
                            .unwrap_or(state.corner_mode);

                        // Component 6: Decoupled Multi-Threaded Async Routing Solver
                        if state.async_solver.is_none() {
                            state.async_solver =
                                Some(oxide_router::interactive::AsyncRoutingSolver::spawn(
                                    router.clone(),
                                ));
                        }

                        if let Some(solver) = &state.async_solver {
                            solver.submit_target(
                                target,
                                session_mode,
                                session_corner,
                                state.routing_net,
                                state.routing_layer,
                                router
                                    .session
                                    .as_ref()
                                    .map(|s| s.track_width)
                                    .unwrap_or(200),
                                state.routing_epoch,
                            );
                            if let Some(resp) = solver.poll_latest(state.routing_epoch) {
                                state.live_route_preview = resp.segments;
                            }
                        } else {
                            state.live_route_preview = router.on_mouse_move(target);
                        }
                    }

                    return Some(canvas::Action::publish(Message::CanvasEvent(
                        CanvasEvent::CursorAt {
                            x: world.x,
                            y: world.y,
                            zoom_pct,
                        },
                    )));
                }
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Single read of the sole camera home; both the background/grid cache
        // and the content cache below project from this one value.
        let camera = self.camera.borrow();
        let bg = self.bg_cache.draw(renderer, bounds.size(), |frame| {
            frame.fill_rectangle(iced::Point::ORIGIN, bounds.size(), self.theme_bg);

            if self.grid_visible {
                let step = (self.visible_grid_mm as f32 * camera.scale).max(8.0);
                let mut x = camera.offset.x.rem_euclid(step) - step;
                while x <= bounds.width + step {
                    let path = canvas::Path::line(
                        iced::Point::new(x, 0.0),
                        iced::Point::new(x, bounds.height),
                    );
                    frame.stroke(
                        &path,
                        canvas::Stroke::default()
                            .with_color(Color {
                                a: 0.18,
                                ..self.theme_grid
                            })
                            .with_width(0.5),
                    );
                    x += step;
                }

                let mut y = camera.offset.y.rem_euclid(step) - step;
                while y <= bounds.height + step {
                    let path = canvas::Path::line(
                        iced::Point::new(0.0, y),
                        iced::Point::new(bounds.width, y),
                    );
                    frame.stroke(
                        &path,
                        canvas::Stroke::default()
                            .with_color(Color {
                                a: 0.18,
                                ..self.theme_grid
                            })
                            .with_width(0.5),
                    );
                    y += step;
                }
            }
        });

        // Dynamic routing overlay frame (always rendered un-cached for 60 FPS feedback)
        let mut overlay_frame = canvas::Frame::new(renderer, bounds.size());
        if !state.live_route_preview.is_empty() || !state.preview_vias.is_empty() {
            // Draw clearance halos first (under preview tracks)
            for seg in &state.live_route_preview {
                let p0 = world_to_screen(
                    &camera,
                    bounds,
                    [
                        seg.start_point.to_mm().0 as f32,
                        seg.start_point.to_mm().1 as f32,
                    ],
                );
                let p1 = world_to_screen(
                    &camera,
                    bounds,
                    [
                        seg.end_point.to_mm().0 as f32,
                        seg.end_point.to_mm().1 as f32,
                    ],
                );
                let halo_width = ((seg.width as f32 / 1000.0 + 0.4) * camera.scale).max(1.0);
                let path = canvas::Path::line(p0, p1);
                overlay_frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_width(halo_width)
                        .with_color(Color::from_rgba(0.0, 0.9, 1.0, 0.15)),
                );
            }

            // Draw preview track segments
            for seg in &state.live_route_preview {
                let p0 = world_to_screen(
                    &camera,
                    bounds,
                    [
                        seg.start_point.to_mm().0 as f32,
                        seg.start_point.to_mm().1 as f32,
                    ],
                );
                let p1 = world_to_screen(
                    &camera,
                    bounds,
                    [
                        seg.end_point.to_mm().0 as f32,
                        seg.end_point.to_mm().1 as f32,
                    ],
                );
                let track_width = ((seg.width as f32 / 1000.0) * camera.scale).max(1.0);
                let path = canvas::Path::line(p0, p1);
                let seg_color = if seg.layer == 0 {
                    Color::from_rgba(0.9, 0.2, 0.2, 0.85) // F.Cu
                } else {
                    Color::from_rgba(0.2, 0.5, 0.9, 0.85) // B.Cu
                };
                overlay_frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_width(track_width)
                        .with_color(seg_color),
                );
            }

            // Draw preview vias
            for via in &state.preview_vias {
                let center = world_to_screen(
                    &camera,
                    bounds,
                    [via.position.to_mm().0 as f32, via.position.to_mm().1 as f32],
                );
                let pad_r = ((via.pad_diameter as f32 / 2000.0) * camera.scale).max(1.0);
                let drill_r = ((via.drill_size as f32 / 2000.0) * camera.scale).max(0.5);

                let pad_path = canvas::Path::circle(center, pad_r);
                overlay_frame.fill(&pad_path, Color::from_rgba(0.85, 0.85, 0.2, 0.9));

                let drill_path = canvas::Path::circle(center, drill_r);
                overlay_frame.fill(&drill_path, Color::from_rgba(0.1, 0.1, 0.1, 1.0));
            }

            // Draw Length / Tuning HUD badge if tuning hud is active
            if let Some(router) = &state.active_router
                && let Some(session) = &router.session
                && let Some(hud) = &session.tuning_hud
            {
                let badge_pos = iced::Point::new(bounds.width - 260.0, 20.0);
                let badge_size = iced::Size::new(240.0, 78.0);
                overlay_frame.fill_rectangle(
                    badge_pos,
                    badge_size,
                    Color::from_rgba(0.06, 0.08, 0.12, 0.94),
                );
                overlay_frame.stroke(
                    &canvas::Path::rectangle(badge_pos, badge_size),
                    canvas::Stroke::default()
                        .with_width(1.0)
                        .with_color(Color::from_rgba(0.0, 0.85, 1.0, 0.8)),
                );

                let cur_len_um = state
                    .live_route_preview
                    .iter()
                    .map(|s| s.length())
                    .sum::<i64>();
                let target_mm = hud.target_length_microns as f64 / 1000.0;
                let cur_mm = cur_len_um as f64 / 1000.0;
                let diff_um = (cur_len_um - hud.target_length_microns).abs();
                let in_tol = diff_um <= hud.tolerance_microns;

                let title = format!("TUNING: {:.2}mm / {:.2}mm", cur_mm, target_mm);
                overlay_frame.fill_text(canvas::Text {
                    content: title,
                    position: iced::Point::new(badge_pos.x + 8.0, badge_pos.y + 6.0),
                    color: if in_tol {
                        Color::from_rgb(0.2, 0.9, 0.3)
                    } else {
                        Color::from_rgb(1.0, 0.8, 0.2)
                    },
                    size: 13.0.into(),
                    ..Default::default()
                });

                let status = if in_tol {
                    format!("MATCH (±{:.0}µm)", diff_um)
                } else if cur_len_um < hud.target_length_microns {
                    format!(
                        "SHORT by {:.1}µm",
                        (hud.target_length_microns - cur_len_um) as f64
                    )
                } else {
                    format!(
                        "LONG by {:.1}µm",
                        (cur_len_um - hud.target_length_microns) as f64
                    )
                };
                overlay_frame.fill_text(canvas::Text {
                    content: status,
                    position: iced::Point::new(badge_pos.x + 8.0, badge_pos.y + 26.0),
                    color: Color::from_rgb(0.9, 0.9, 0.9),
                    size: 11.0.into(),
                    ..Default::default()
                });

                let controls = format!(
                    "Amp: {}µm  Pitch: {}µm (1-4)",
                    hud.amplitude_microns, hud.pitch_microns
                );
                overlay_frame.fill_text(canvas::Text {
                    content: controls,
                    position: iced::Point::new(badge_pos.x + 8.0, badge_pos.y + 44.0),
                    color: Color::from_rgba(0.7, 0.7, 0.7, 0.8),
                    size: 10.0.into(),
                    ..Default::default()
                });

                // Tolerance bar indicator
                let bar_rect = canvas::Path::rectangle(
                    iced::Point::new(badge_pos.x + 8.0, badge_pos.y + 62.0),
                    iced::Size::new(224.0, 6.0),
                );
                overlay_frame.fill(&bar_rect, Color::from_rgba(0.2, 0.2, 0.2, 1.0));
                let progress = ((cur_mm / target_mm.max(0.01)) as f32).clamp(0.0, 1.0);
                let fill_rect = canvas::Path::rectangle(
                    iced::Point::new(badge_pos.x + 8.0, badge_pos.y + 62.0),
                    iced::Size::new(224.0 * progress, 6.0),
                );
                overlay_frame.fill(
                    &fill_rect,
                    if in_tol {
                        Color::from_rgb(0.2, 0.9, 0.3)
                    } else {
                        Color::from_rgb(0.9, 0.3, 0.2)
                    },
                );
            }

            // Floating On-Cursor Length & Routing Telemetry HUD badge (D10 Parity)
            if (state.active_router.is_some() || !state.live_route_preview.is_empty())
                && let Some(cursor_pos) = cursor.position_in(bounds).or(state.crosshair_pos)
            {
                let total_len_um: i64 = state.live_route_preview.iter().map(|s| s.length()).sum();
                let last_seg_len_um: i64 = state
                    .live_route_preview
                    .last()
                    .map(|s| s.length())
                    .unwrap_or(0);
                let total_mm = total_len_um as f64 / 1000.0;
                let seg_mm = last_seg_len_um as f64 / 1000.0;

                let net_name = if state.routing_net > 0 {
                    format!("Net_{}", state.routing_net)
                } else {
                    "Trace".to_string()
                };

                let layer_name = match state.routing_layer {
                    0 => "Top Layer",
                    31 => "Bottom Layer",
                    _ => "Copper",
                };

                let hud_x = (cursor_pos.x + 18.0).min(bounds.width - 180.0).max(10.0);
                let hud_y = (cursor_pos.y + 18.0).min(bounds.height - 65.0).max(10.0);
                let hud_pos = iced::Point::new(hud_x, hud_y);
                let hud_size = iced::Size::new(172.0, 50.0);

                overlay_frame.fill_rectangle(
                    hud_pos,
                    hud_size,
                    Color::from_rgba(0.04, 0.06, 0.10, 0.92),
                );
                overlay_frame.stroke(
                    &canvas::Path::rectangle(hud_pos, hud_size),
                    canvas::Stroke::default()
                        .with_width(1.0)
                        .with_color(Color::from_rgba(0.0, 0.85, 1.0, 0.8)),
                );

                // Line 1: Net & Layer
                overlay_frame.fill_text(canvas::Text {
                    content: format!("{net_name} [{layer_name}]"),
                    position: iced::Point::new(hud_pos.x + 8.0, hud_pos.y + 5.0),
                    color: Color::from_rgb(0.0, 0.9, 1.0),
                    size: 11.0.into(),
                    ..Default::default()
                });

                // Line 2: Length and Segment
                overlay_frame.fill_text(canvas::Text {
                    content: format!("L: {:.2}mm  (Seg: {:.2})", total_mm, seg_mm),
                    position: iced::Point::new(hud_pos.x + 8.0, hud_pos.y + 20.0),
                    color: Color::from_rgb(0.95, 0.95, 0.95),
                    size: 12.0.into(),
                    ..Default::default()
                });

                // Line 3: Corner Mode
                overlay_frame.fill_text(canvas::Text {
                    content: format!("Mode: {:?}", state.corner_mode),
                    position: iced::Point::new(hud_pos.x + 8.0, hud_pos.y + 36.0),
                    color: Color::from_rgb(1.0, 0.8, 0.2),
                    size: 9.0.into(),
                    ..Default::default()
                });
            }
        }

        // Live PDN Heatmap Tooltip readout when cursor hovers over mesh
        if self.show_pdn_heatmap
            && let Some(cursor_pos) = cursor.position_in(bounds)
        {
            let world_x = (cursor_pos.x - camera.offset.x) / camera.scale;
            let world_y = (cursor_pos.y - camera.offset.y) / camera.scale;

            if let Some(grid) = &*self.pdn_mesh_cache.borrow() {
                let c = (world_x as f64 / grid.step_mm).floor() as isize;
                let r = (world_y as f64 / grid.step_mm).floor() as isize;
                if c >= 0 && c < grid.cols as isize && r >= 0 && r < grid.rows as isize {
                    let idx = (r as usize) * grid.cols + (c as usize);
                    if let Some(node) = grid.nodes.get(idx) {
                        let drop_mv = (grid.nominal_voltage_v - node.voltage_v) * 1e3;
                        let tip_text = format!("PDN: {:.3}V (-{:.1}mV)", node.voltage_v, drop_mv);

                        let tip_pos = iced::Point::new(cursor_pos.x + 14.0, cursor_pos.y + 14.0);
                        let tip_size = iced::Size::new(160.0, 26.0);
                        overlay_frame.fill_rectangle(
                            tip_pos,
                            tip_size,
                            Color::from_rgba(0.06, 0.08, 0.12, 0.92),
                        );
                        overlay_frame.stroke(
                            &canvas::Path::rectangle(tip_pos, tip_size),
                            canvas::Stroke::default()
                                .with_width(1.0)
                                .with_color(Color::from_rgba(0.9, 0.7, 0.1, 0.8)),
                        );
                        overlay_frame.fill_text(canvas::Text {
                            content: tip_text,
                            position: iced::Point::new(tip_pos.x + 8.0, tip_pos.y + 6.0),
                            color: Color::from_rgb(0.95, 0.95, 0.95),
                            size: 12.0.into(),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // Render Git-Native Visual Diff Overlay
        if let Some(diff) = &state.visual_diff {
            let mode_str = match state.diff_mode {
                crate::diff::VisualDiffMode::UnifiedOverlay => "Unified Overlay",
                crate::diff::VisualDiffMode::SplitSwipe => "Split-Swipe Divider",
                crate::diff::VisualDiffMode::SideBySide => "Side-by-Side Viewport",
            };

            // Top Diff Mode HUD badge
            let badge_pos = iced::Point::new((bounds.width / 2.0 - 180.0).max(10.0), 16.0);
            let badge_size = iced::Size::new(360.0, 32.0);
            overlay_frame.fill_rectangle(
                badge_pos,
                badge_size,
                Color::from_rgba(0.05, 0.07, 0.11, 0.92),
            );
            overlay_frame.stroke(
                &canvas::Path::rectangle(badge_pos, badge_size),
                canvas::Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgba(0.3, 0.7, 1.0, 0.8)),
            );
            let diff_summary = format!(
                "DIFF: {} | +{} Added  -{} Removed  ~{} Mod (Tab to Cycle)",
                mode_str, diff.added_count, diff.removed_count, diff.modified_count
            );
            overlay_frame.fill_text(canvas::Text {
                content: diff_summary,
                position: iced::Point::new(badge_pos.x + 12.0, badge_pos.y + 10.0),
                color: Color::from_rgb(0.95, 0.95, 0.95),
                size: 11.0.into(),
                ..Default::default()
            });

            // Split-Swipe vertical line
            if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                let swipe_x = if state.split_swipe_x > 0.0 {
                    state.split_swipe_x
                } else {
                    bounds.width / 2.0
                };
                let corridor_rect = canvas::Path::rectangle(
                    iced::Point::new(swipe_x - 12.0, 0.0),
                    iced::Size::new(24.0, bounds.height),
                );
                overlay_frame.fill(&corridor_rect, Color::from_rgba(0.0, 0.9, 1.0, 0.04));

                let swipe_line = canvas::Path::line(
                    iced::Point::new(swipe_x, 0.0),
                    iced::Point::new(swipe_x, bounds.height),
                );
                overlay_frame.stroke(
                    &swipe_line,
                    canvas::Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgba(0.0, 0.9, 1.0, 0.95)),
                );
                // Handle badge at middle
                let handle_rect = canvas::Path::rectangle(
                    iced::Point::new(swipe_x - 18.0, bounds.height / 2.0 - 12.0),
                    iced::Size::new(36.0, 24.0),
                );
                overlay_frame.fill(&handle_rect, Color::from_rgba(0.0, 0.8, 0.9, 0.9));
                overlay_frame.fill_text(canvas::Text {
                    content: "◄ ►".to_string(),
                    position: iced::Point::new(swipe_x - 10.0, bounds.height / 2.0 - 6.0),
                    color: Color::BLACK,
                    size: 10.0.into(),
                    ..Default::default()
                });
            } else if state.diff_mode == crate::diff::VisualDiffMode::SideBySide {
                let mid_x = bounds.width / 2.0;
                let divider = canvas::Path::line(
                    iced::Point::new(mid_x, 0.0),
                    iced::Point::new(mid_x, bounds.height),
                );
                overlay_frame.stroke(
                    &divider,
                    canvas::Stroke::default()
                        .with_width(1.5)
                        .with_color(Color::from_rgba(0.6, 0.6, 0.7, 0.6)),
                );
                overlay_frame.fill_text(canvas::Text {
                    content: "BASELINE (HEAD~1)".to_string(),
                    position: iced::Point::new((mid_x - 140.0).max(10.0), 56.0),
                    color: Color::from_rgba(0.9, 0.3, 0.3, 0.8),
                    size: 11.0.into(),
                    ..Default::default()
                });
                overlay_frame.fill_text(canvas::Text {
                    content: "COMPARISON (WORKTREE)".to_string(),
                    position: iced::Point::new(mid_x + 20.0, 56.0),
                    color: Color::from_rgba(0.2, 0.9, 0.3, 0.8),
                    size: 11.0.into(),
                    ..Default::default()
                });
            }

            // Draw color-coded diff elements:
            let swipe_x = if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                if state.split_swipe_x > 0.0 {
                    state.split_swipe_x
                } else {
                    bounds.width / 2.0
                }
            } else {
                f32::MAX
            };

            // 1. Segments
            for seg_diff in &diff.segment_diffs {
                let color_arr =
                    crate::diff::PcbVisualDiff::classification_color(seg_diff.classification);
                let color =
                    Color::from_rgba(color_arr[0], color_arr[1], color_arr[2], color_arr[3]);

                let (p0_world, p1_world) = match seg_diff.classification {
                    crate::diff::DiffClassification::Removed => {
                        if let Some(s) = &seg_diff.before {
                            (
                                [s.start.x as f32, s.start.y as f32],
                                [s.end.x as f32, s.end.y as f32],
                            )
                        } else {
                            continue;
                        }
                    }
                    crate::diff::DiffClassification::Added
                    | crate::diff::DiffClassification::Modified => {
                        if let Some(s) = &seg_diff.after {
                            (
                                [s.start.x as f32, s.start.y as f32],
                                [s.end.x as f32, s.end.y as f32],
                            )
                        } else {
                            continue;
                        }
                    }
                };

                let screen_p0 = world_to_screen(&camera, bounds, p0_world);
                let screen_p1 = world_to_screen(&camera, bounds, p1_world);

                // If in SplitSwipe mode, only show before if on left, after if on right
                if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                    let mid_screen_x = (screen_p0.x + screen_p1.x) * 0.5;
                    if seg_diff.classification == crate::diff::DiffClassification::Removed
                        && mid_screen_x > swipe_x
                    {
                        continue;
                    }
                    if seg_diff.classification == crate::diff::DiffClassification::Added
                        && mid_screen_x < swipe_x
                    {
                        continue;
                    }
                }

                let path = canvas::Path::line(screen_p0, screen_p1);
                overlay_frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_width((3.0 * camera.scale).max(1.0))
                        .with_color(color),
                );
            }

            // 2. Footprints
            for fp_diff in &diff.footprint_diffs {
                let color_arr =
                    crate::diff::PcbVisualDiff::classification_color(fp_diff.classification);
                let color =
                    Color::from_rgba(color_arr[0], color_arr[1], color_arr[2], color_arr[3]);

                let (fp_ref, pos_world) = match fp_diff.classification {
                    crate::diff::DiffClassification::Removed => {
                        if let Some(f) = &fp_diff.before {
                            (&f.reference, [f.position.x as f32, f.position.y as f32])
                        } else {
                            continue;
                        }
                    }
                    crate::diff::DiffClassification::Added
                    | crate::diff::DiffClassification::Modified => {
                        if let Some(f) = &fp_diff.after {
                            (&f.reference, [f.position.x as f32, f.position.y as f32])
                        } else {
                            continue;
                        }
                    }
                };

                let center = world_to_screen(&camera, bounds, pos_world);

                if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                    if fp_diff.classification == crate::diff::DiffClassification::Removed
                        && center.x > swipe_x
                    {
                        continue;
                    }
                    if fp_diff.classification == crate::diff::DiffClassification::Added
                        && center.x < swipe_x
                    {
                        continue;
                    }
                }

                let box_r = (12.0 * camera.scale).clamp(4.0, 30.0);
                let fp_box = canvas::Path::rectangle(
                    iced::Point::new(center.x - box_r, center.y - box_r),
                    iced::Size::new(box_r * 2.0, box_r * 2.0),
                );
                overlay_frame.stroke(
                    &fp_box,
                    canvas::Stroke::default().with_width(2.0).with_color(color),
                );
                overlay_frame.fill_text(canvas::Text {
                    content: fp_ref.clone(),
                    position: iced::Point::new(center.x - box_r, center.y - box_r - 12.0),
                    color,
                    size: 10.0.into(),
                    ..Default::default()
                });
            }

            // 3. Vias
            for via_diff in &diff.via_diffs {
                let color_arr =
                    crate::diff::PcbVisualDiff::classification_color(via_diff.classification);
                let color =
                    Color::from_rgba(color_arr[0], color_arr[1], color_arr[2], color_arr[3]);

                let pos_world = match via_diff.classification {
                    crate::diff::DiffClassification::Removed => {
                        if let Some(v) = &via_diff.before {
                            [v.position.x as f32, v.position.y as f32]
                        } else {
                            continue;
                        }
                    }
                    crate::diff::DiffClassification::Added
                    | crate::diff::DiffClassification::Modified => {
                        if let Some(v) = &via_diff.after {
                            [v.position.x as f32, v.position.y as f32]
                        } else {
                            continue;
                        }
                    }
                };

                let center = world_to_screen(&camera, bounds, pos_world);

                if state.diff_mode == crate::diff::VisualDiffMode::SplitSwipe {
                    if via_diff.classification == crate::diff::DiffClassification::Removed
                        && center.x > swipe_x
                    {
                        continue;
                    }
                    if via_diff.classification == crate::diff::DiffClassification::Added
                        && center.x < swipe_x
                    {
                        continue;
                    }
                }

                let via_circle =
                    canvas::Path::circle(center, (6.0 * camera.scale).clamp(2.0, 20.0));
                overlay_frame.stroke(
                    &via_circle,
                    canvas::Stroke::default().with_width(2.0).with_color(color),
                );
            }
        }

        // Full-Screen 90° Orthographic Alignment Crosshair (Altium Parity)
        if (state.show_large_crosshair || state.active_router.is_some())
            && let Some(pos) = state.crosshair_pos
        {
            let h_line = canvas::Path::line(
                iced::Point::new(0.0, pos.y),
                iced::Point::new(bounds.width, pos.y),
            );
            let v_line = canvas::Path::line(
                iced::Point::new(pos.x, 0.0),
                iced::Point::new(pos.x, bounds.height),
            );
            let stroke = canvas::Stroke::default()
                .with_width(0.75)
                .with_color(Color::from_rgba(0.0, 0.94, 1.0, 0.35));
            overlay_frame.stroke(&h_line, stroke);
            overlay_frame.stroke(&v_line, stroke);
        }

        let dynamic_overlay = overlay_frame.into_geometry();

        // GPU mode: a `shader` stacked above this canvas draws the board
        // content on the GPU (see `Self::gpu_render` / `Self::gpu_scene`). The
        // CPU canvas then contributes only the background + grid (`bg`), which
        // sits *behind* the shader — iced's shader primitive composites over
        // whatever is already there and never clears its own region.
        if self.gpu_render {
            return vec![bg, dynamic_overlay];
        }

        let (cached_offset_x, cached_offset_y, cached_scale) = self.content_cache_camera.get();
        let camera_matches_cache = (cached_offset_x - camera.offset.x).abs() < 0.01
            && (cached_offset_y - camera.offset.y).abs() < 0.01
            && (cached_scale - camera.scale).abs() < 0.0001;
        if !camera_matches_cache {
            self.content_cache.clear();
        }
        let content = self.content_cache.draw(renderer, bounds.size(), |frame| {
            self.content_cache_camera
                .set((camera.offset.x, camera.offset.y, camera.scale));
            if let Some(snapshot) = self.active_renderer_snapshot() {
                let scene = self.build_scene(snapshot);
                draw_scene(frame, &scene, &camera, bounds);
            }
        });

        vec![bg, content, dynamic_overlay]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.panning {
            return mouse::Interaction::Grabbing;
        }
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_renderer::pcb::{DrcMarkerInput, PcbSnapshot};
    use oxide_types::violation::Severity;

    /// Correctness — thread #4 (z-order): `gpu_scene()` must NOT fold overlay
    /// geometry into the base `polygons`/`lines`/`circles` buckets. A DRC
    /// marker (an overlay-only primitive with no base-scene counterpart in
    /// this snapshot) must stay in `overlay_*` so the shader's dedicated
    /// overlay pass — which always composites after every base bucket — is
    /// the only thing that draws it. If the fold regresses, this marker would
    /// show up in `scene.polygons`/`scene.circles` instead, which is exactly
    /// the bug that drew overlays *under* base content.
    #[test]
    fn gpu_scene_keeps_overlays_out_of_the_base_buckets() {
        let mut canvas = PcbCanvas::new();
        let snapshot = PcbSnapshot::default().with_drc_markers(vec![DrcMarkerInput {
            center: [1.0, 1.0],
            radius_mm: 0.3,
            severity: Severity::Error,
            violation_type: None,
        }]);
        canvas.set_renderer_snapshot(Some(snapshot));

        let scene = canvas.gpu_scene().expect("snapshot is set");
        assert!(
            scene.polygons.is_empty(),
            "the DRC marker's overlay polygon must not land in the base bucket"
        );
        assert!(
            scene.circles.is_empty(),
            "the DRC marker's overlay circle must not land in the base bucket"
        );
        assert_eq!(scene.overlay_polygons.len(), 1);
        assert_eq!(scene.overlay_circles.len(), 1);
    }

    #[test]
    fn gpu_scene_is_none_without_a_snapshot() {
        let canvas = PcbCanvas::new();
        assert!(canvas.gpu_scene().is_none());
        // A missing snapshot must not populate the cache.
        assert!(canvas.scene_cache.borrow().is_none());
    }

    #[test]
    fn gpu_scene_builds_once_then_serves_from_cache() {
        let mut canvas = PcbCanvas::new();
        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));

        // First call tessellates and fills the cache.
        assert!(canvas.gpu_scene().is_some());
        assert!(canvas.scene_cache.borrow().is_some());

        // Subsequent calls are served from the cache (still populated).
        assert!(canvas.gpu_scene().is_some());
        assert!(canvas.scene_cache.borrow().is_some());
    }

    #[test]
    fn clear_content_cache_invalidates_the_gpu_scene() {
        let mut canvas = PcbCanvas::new();
        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));
        let _ = canvas.gpu_scene();
        assert!(canvas.scene_cache.borrow().is_some());

        canvas.clear_content_cache();
        assert!(canvas.scene_cache.borrow().is_none());
    }

    #[test]
    fn setting_a_new_snapshot_invalidates_the_gpu_scene() {
        let mut canvas = PcbCanvas::new();
        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));
        let _ = canvas.gpu_scene();
        assert!(canvas.scene_cache.borrow().is_some());

        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));
        assert!(canvas.scene_cache.borrow().is_none());
    }

    #[test]
    fn scene_generation_bumps_on_every_invalidation() {
        let mut canvas = PcbCanvas::new();
        let g0 = canvas.scene_generation();

        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));
        let g1 = canvas.scene_generation();
        assert!(g1 > g0, "snapshot swap must bump the generation");

        canvas.clear_content_cache();
        let g2 = canvas.scene_generation();
        assert!(g2 > g1, "content/theme change must bump the generation");
    }

    #[test]
    fn gpu_scene_keeps_a_stable_generation_across_cache_hits() {
        let mut canvas = PcbCanvas::new();
        canvas.set_renderer_snapshot(Some(PcbSnapshot::default()));

        let gen_at_build = canvas.scene_generation();
        let _ = canvas.gpu_scene(); // miss: builds + caches
        // A pan/zoom re-reads gpu_scene without invalidating: same generation,
        // so the shader will skip the geometry upload.
        let _ = canvas.gpu_scene(); // hit
        assert_eq!(canvas.scene_generation(), gen_at_build);
    }

    #[test]
    fn live_camera_reflects_the_single_source_after_a_mutation() {
        // #5 regression: `view()` (GPU shader) and the CPU `draw` both read the
        // one `camera` cell. Mutating it the way `Program::update` does must be
        // visible through `live_camera()` — there is no separate shadow that
        // could go stale.
        let canvas = PcbCanvas::new();
        let (x0, y0, s0) = canvas.live_camera();

        canvas.camera.borrow_mut().pan(12.0, -7.0);

        let (x1, y1, s1) = canvas.live_camera();
        assert_eq!(x1, x0 + 12.0);
        assert_eq!(y1, y0 - 7.0);
        assert_eq!(s1, s0, "pan must not change scale");
    }

    #[test]
    fn gpu_scene_keeps_an_overlay_polygon_out_of_the_main_bucket() {
        // #4 (fixed, review #308 finding 4): `gpu_scene()` used to fold
        // `overlay_polygons` into the main `polygons` Vec *before* the GPU
        // shader ever saw the scene, so `GPU_SCENE_DRAW_ORDER` (which has no
        // overlay buckets at all) drew that geometry as an ordinary member of
        // the early `Polygons` bucket -- under traces, not on top of them --
        // while the CPU `draw_scene` path walks `CPU_PCB_DRAW_ORDER`, which
        // paints `OverlayPolygons` LAST. `gpu_scene()` now leaves overlay
        // geometry in its own field, and `scene_shader::ScenePrimitive::draw`
        // composites it in a dedicated pass strictly after every base bucket
        // -- matching the CPU path exactly. This pins that CPU==GPU parity by
        // comparing the same distinctly-coloured overlay polygon across both
        // scene builds.
        let mut canvas = PcbCanvas::new();
        let snapshot = PcbSnapshot::default().with_drc_markers(vec![DrcMarkerInput {
            center: [5.0, 5.0],
            radius_mm: 0.5,
            severity: Severity::Error,
            violation_type: None,
        }]);

        // CPU reference: a DRC marker is the one thing an otherwise-empty
        // board contributes to `overlay_polygons`, distinctly coloured by
        // severity (`ColorSlot::ErcError`) and nothing else lands in the
        // plain `polygons` bucket for an empty board.
        let cpu_scene = canvas.build_scene(&snapshot);
        assert_eq!(
            cpu_scene.overlay_polygons.len(),
            1,
            "a DRC marker must emit exactly one overlay polygon"
        );
        assert!(
            cpu_scene.polygons.is_empty(),
            "an empty board contributes no main-bucket polygons"
        );
        let overlay_fill_color = cpu_scene.overlay_polygons[0].fill_color;

        canvas.set_renderer_snapshot(Some(snapshot));
        let gpu_scene = canvas.gpu_scene().expect("snapshot is set");

        // Parity: the same distinctly-coloured polygon stays in
        // `overlay_polygons` on the GPU path too, never folded into the main
        // `polygons` bucket -- CPU and GPU agree on where it lives.
        assert!(
            gpu_scene.polygons.is_empty(),
            "the overlay polygon must not be folded into the main polygons bucket"
        );
        assert_eq!(
            gpu_scene.overlay_polygons.len(),
            1,
            "gpu_scene must keep the overlay polygon in its own bucket"
        );
        assert_eq!(gpu_scene.overlay_polygons[0].fill_color, overlay_fill_color);
    }

    #[test]
    fn test_interactive_routing_state_transitions() {
        let mut state = PcbCanvasState::default();
        assert!(state.active_router.is_none());
        assert!(state.live_route_preview.is_empty());
        assert!(state.preview_vias.is_empty());

        let rules = std::sync::Arc::new(oxide_rules::ConstraintManager::standard_default());
        let board = oxide_types::pcb::PcbBoard {
            uuid: uuid::Uuid::new_v4(),
            version: 1,
            generator: "oxide".to_string(),
            thickness: 1.6,
            outline: Vec::new(),
            layers: Vec::new(),
            setup: None,
            nets: Vec::new(),
            footprints: Vec::new(),
            segments: Vec::new(),
            vias: Vec::new(),
            zones: Vec::new(),
            graphics: Vec::new(),
            texts: Vec::new(),
        };
        let spatial = std::sync::Arc::new(oxide_router::SpatialIndex::build(&board));
        let mut router = oxide_router::interactive::InteractiveRouter::new(rules, spatial);
        let start = oxide_router::geometry::Point2D::from_mm(10.0, 10.0);
        let _ = router.start_routing(start, 1, 0);

        let preview = router.on_mouse_move(oxide_router::geometry::Point2D::from_mm(20.0, 15.0));
        assert!(!preview.is_empty());
        state.active_router = Some(router);
        state.live_route_preview = preview;
        assert!(!state.live_route_preview.is_empty());
    }

    #[test]
    fn test_pdn_heatmap_gpu_compositing_generation() {
        let mut canvas = PcbCanvas::new();
        let snapshot = PcbSnapshot::default();
        canvas.set_renderer_snapshot(Some(snapshot.clone()));

        // Without heatmap
        canvas.show_pdn_heatmap = false;
        let scene_base = canvas.build_scene(&snapshot);
        assert!(scene_base.overlay_polygons.is_empty());

        // With PDN heatmap enabled
        canvas.show_pdn_heatmap = true;
        let scene_heatmap = canvas.build_scene(&snapshot);
        assert!(!scene_heatmap.overlay_polygons.is_empty());
        assert!(canvas.pdn_mesh_cache.borrow().is_some());
    }

    #[test]
    fn test_interactive_tuning_hud_and_modes() {
        let mut state = PcbCanvasState::default();
        let rules = std::sync::Arc::new(oxide_rules::ConstraintManager::standard_default());
        let spatial_index = std::sync::Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
        let mut router = oxide_router::interactive::InteractiveRouter::new(rules, spatial_index);
        let start = oxide_router::geometry::Point2D::new(0, 0);
        router.start_routing(start, 1, 0).unwrap();

        // Enable length tuning
        router.enable_length_tuning(20_000, 0);
        assert_eq!(
            router.session.as_ref().unwrap().mode,
            oxide_router::interactive::RoutingMode::LengthTuning
        );
        assert!(router.session.as_ref().unwrap().tuning_hud.is_some());

        // Hotkeys for tuning
        router.hotkey_increase_amplitude();
        assert_eq!(
            router
                .session
                .as_ref()
                .unwrap()
                .tuning_hud
                .as_ref()
                .unwrap()
                .amplitude_microns,
            900
        );
        router.hotkey_increase_pitch();
        assert_eq!(
            router
                .session
                .as_ref()
                .unwrap()
                .tuning_hud
                .as_ref()
                .unwrap()
                .pitch_microns,
            700
        );

        state.active_router = Some(router);
        assert!(state.active_router.is_some());
    }

    #[test]
    fn test_d10_floating_on_cursor_length_hud() {
        let mut state = PcbCanvasState::default();
        let rules = std::sync::Arc::new(oxide_rules::ConstraintManager::standard_default());
        let spatial_index = std::sync::Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
        let mut router = oxide_router::interactive::InteractiveRouter::new(rules, spatial_index);
        let start = oxide_router::geometry::Point2D::new(0, 0);
        router.start_routing(start, 1, 0).unwrap();

        // Simulate interactive route preview
        let s1 = oxide_router::RouteSegment {
            start_point: oxide_router::geometry::Point2D::new(0, 0),
            end_point: oxide_router::geometry::Point2D::new(5000, 0),
            width: 250,
            layer: 0,
            net_id: 1,
            segment_type: oxide_router::SegmentType::Straight,
        };
        let s2 = oxide_router::RouteSegment {
            start_point: oxide_router::geometry::Point2D::new(5000, 0),
            end_point: oxide_router::geometry::Point2D::new(5000, 3000),
            width: 250,
            layer: 0,
            net_id: 1,
            segment_type: oxide_router::SegmentType::Straight,
        };
        state.live_route_preview = vec![s1, s2];
        state.routing_net = 1;
        state.routing_layer = 0;
        state.crosshair_pos = Some(iced::Point::new(450.0, 320.0));
        state.active_router = Some(router);

        // Verify total length calculation: 5000µm + 3000µm = 8000µm = 8.00mm
        let total_len_um: i64 = state.live_route_preview.iter().map(|s| s.length()).sum();
        let last_seg_len_um: i64 = state
            .live_route_preview
            .last()
            .map(|s| s.length())
            .unwrap_or(0);
        assert_eq!(
            total_len_um, 8000,
            "Total route length must be 8000µm (8mm)"
        );
        assert_eq!(
            last_seg_len_um, 3000,
            "Last segment length must be 3000µm (3mm)"
        );
        assert_eq!(state.crosshair_pos, Some(iced::Point::new(450.0, 320.0)));
    }

    #[test]
    fn test_canvas_split_swipe_and_overlay_rendering() {
        let mut state = PcbCanvasState::default();
        let mut board_a = oxide_types::pcb::PcbBoard::default();
        let mut board_b = oxide_types::pcb::PcbBoard::default();

        let r1 = oxide_types::pcb::Footprint {
            uuid: uuid::Uuid::new_v4(),
            reference: "R1".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0603".to_string(),
            position: oxide_types::pcb::Point { x: 10.0, y: 10.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        board_a.footprints.push(r1.clone());

        let mut r1_mod = r1;
        r1_mod.value = "20k".to_string();
        board_b.footprints.push(r1_mod);

        let seg = oxide_types::pcb::Segment {
            uuid: uuid::Uuid::new_v4(),
            start: oxide_types::pcb::Point { x: 0.0, y: 0.0 },
            end: oxide_types::pcb::Point { x: 20.0, y: 0.0 },
            width: 0.25,
            layer: "F.Cu".to_string(),
            net: 1,
        };
        board_b.segments.push(seg);

        let diff = crate::diff::PcbVisualDiff::compute_diff(&board_a, &board_b);
        assert_eq!(diff.modified_count, 1);
        assert_eq!(diff.added_count, 1);

        state.visual_diff = Some(diff);
        assert_eq!(state.diff_mode, crate::diff::VisualDiffMode::UnifiedOverlay);

        // Test cycle diff mode
        state.diff_mode = crate::diff::VisualDiffMode::SplitSwipe;
        state.split_swipe_x = 400.0;
        assert_eq!(state.diff_mode, crate::diff::VisualDiffMode::SplitSwipe);
        assert_eq!(state.split_swipe_x, 400.0);

        state.diff_mode = crate::diff::VisualDiffMode::SideBySide;
        assert_eq!(state.diff_mode, crate::diff::VisualDiffMode::SideBySide);
    }

    #[test]
    fn test_canvas_interactive_clothoid_meander_commitment() {
        let rules = std::sync::Arc::new(oxide_rules::ConstraintManager::standard_default());
        let spatial_index = std::sync::Arc::new(oxide_router::geometry::rtree::SpatialIndex::new());
        let mut router = oxide_router::interactive::InteractiveRouter::new(rules, spatial_index);
        let start = oxide_router::geometry::Point2D::new(0, 0);
        router.start_routing(start, 1, 0).unwrap();

        // Switch corner style to Arc45 for G2 Clothoid / Euler filleted transitions
        router.session.as_mut().unwrap().corner_style =
            oxide_router::interactive::session::CornerStyle::Arc45;
        router.session.as_mut().unwrap().corner_mode = oxide_router::CornerMode::FortyFiveArc;

        let end = oxide_router::geometry::Point2D::new(50_000, 0);
        let meander_segs =
            router.generate_meander_with_params(start, end, 30_000, 1, 0, 200, 1200, 800);

        // Filleted meander produces multiple sub-segments along Euler spirals
        assert!(
            meander_segs.len() > 4,
            "Filleted clothoid meander must decompose into polyline arcs"
        );

        // Verify total length increased beyond straight line
        let total_len: i64 = meander_segs.iter().map(|s| s.length()).sum();
        assert!(
            total_len > 50_000,
            "Meander total length must exceed straight path"
        );

        // Convert route segments into board segments (commitment)
        let mut committed_board = oxide_types::pcb::PcbBoard::default();
        for s in &meander_segs {
            committed_board.segments.push(oxide_types::pcb::Segment {
                uuid: uuid::Uuid::new_v4(),
                start: oxide_types::pcb::Point {
                    x: s.start_point.x as f64 / 1000.0,
                    y: s.start_point.y as f64 / 1000.0,
                },
                end: oxide_types::pcb::Point {
                    x: s.end_point.x as f64 / 1000.0,
                    y: s.end_point.y as f64 / 1000.0,
                },
                width: s.width as f64 / 1000.0,
                layer: "F.Cu".to_string(),
                net: s.net_id,
            });
        }
        assert_eq!(committed_board.segments.len(), meander_segs.len());
    }

    #[test]
    fn test_canvas_large_crosshair_and_split_swipe_corridor() {
        let mut state = PcbCanvasState::default();

        // 1. Precision crosshair state toggling
        assert!(!state.show_large_crosshair);
        state.show_large_crosshair = true;
        state.crosshair_pos = Some(iced::Point::new(320.0, 240.0));
        assert!(state.show_large_crosshair);
        assert_eq!(state.crosshair_pos, Some(iced::Point::new(320.0, 240.0)));

        // 2. Split-swipe dragging corridor
        state.diff_mode = crate::diff::VisualDiffMode::SplitSwipe;
        state.split_swipe_x = 400.0;
        state.split_swipe_dragging = false;

        // Cursor within ±16px hit corridor arms drag
        let cursor_x = 410.0;
        let in_corridor = (cursor_x - state.split_swipe_x).abs() <= 16.0;
        assert!(in_corridor, "Cursor within 16px must hit drag corridor");

        state.split_swipe_dragging = true;
        state.split_swipe_x = cursor_x;
        assert_eq!(state.split_swipe_x, 410.0);
    }
}
