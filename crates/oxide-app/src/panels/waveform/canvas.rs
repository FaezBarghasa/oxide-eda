//! Waveform Viewer Panel canvas implementation for Oxide EDA.
//! Renders analog voltages, digital signals, time/frequency axes, graticule, and dual measurement cursors.

use iced::mouse::Cursor;
use iced::widget::canvas::{self, Geometry, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use oxide_types::sim::{WaveformDataset, WaveformTrace};
use oxide_types::theme::ThemeTokens;

/// Canvas program for rendering interactive simulation waveforms.
#[derive(Debug, Clone)]
pub struct WaveformCanvas<'a> {
    pub dataset: Option<&'a WaveformDataset>,
    pub selected_traces: &'a [String],
    pub cursor_a: Option<f64>,
    pub cursor_b: Option<f64>,
    pub tokens: &'a ThemeTokens,
}

#[derive(Default)]
pub struct WaveformState {
    cache: canvas::Cache,
}

impl WaveformState {
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

// Distinct trace colors for multi-signal overlay
const TRACE_COLORS: [Color; 6] = [
    Color::from_rgb(0.2, 0.8, 0.2), // Bright green
    Color::from_rgb(0.9, 0.7, 0.1), // Amber / Yellow
    Color::from_rgb(0.2, 0.6, 1.0), // Sky Blue
    Color::from_rgb(1.0, 0.3, 0.3), // Coral / Red
    Color::from_rgb(0.8, 0.3, 0.9), // Magenta
    Color::from_rgb(0.1, 0.9, 0.9), // Cyan
];

impl<'a, Message> Program<Message, Theme, Renderer> for WaveformCanvas<'a> {
    type State = WaveformState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let pad_left = 60.0;
        let pad_right = 20.0;
        let pad_top = 20.0;
        let pad_bottom = 30.0;

        let plot_rect = Rectangle {
            x: pad_left,
            y: pad_top,
            width: (bounds.width - pad_left - pad_right).max(10.0),
            height: (bounds.height - pad_top - pad_bottom).max(10.0),
        };

        // 1. Background
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgba(0.08, 0.09, 0.11, 1.0),
        );

        frame.fill_rectangle(
            Point::new(plot_rect.x, plot_rect.y),
            Size::new(plot_rect.width, plot_rect.height),
            Color::from_rgba(0.04, 0.05, 0.06, 1.0),
        );

        // 2. Graticule / Grid lines
        let grid_divisions_x = 8;
        let grid_divisions_y = 6;
        let grid_color = Color::from_rgba(0.2, 0.24, 0.28, 0.6);

        for i in 0..=grid_divisions_x {
            let x = plot_rect.x + (i as f32 / grid_divisions_x as f32) * plot_rect.width;
            let path = Path::line(
                Point::new(x, plot_rect.y),
                Point::new(x, plot_rect.y + plot_rect.height),
            );
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(grid_color)
                    .with_width(1.0),
            );
        }

        for i in 0..=grid_divisions_y {
            let y = plot_rect.y + (i as f32 / grid_divisions_y as f32) * plot_rect.height;
            let path = Path::line(
                Point::new(plot_rect.x, y),
                Point::new(plot_rect.x + plot_rect.width, y),
            );
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(grid_color)
                    .with_width(1.0),
            );
        }

        let dataset = match self.dataset {
            Some(ds) if !ds.x_trace.values.is_empty() => ds,
            _ => {
                // Empty state label
                frame.fill_text(canvas::Text {
                    content: "No simulation dataset loaded. Run simulation (F9) to view waveforms.".to_string(),
                    position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                    color: Color::from_rgba(0.6, 0.6, 0.6, 1.0),
                    size: iced::Pixels(12.0),
                    align_x: iced::alignment::Horizontal::Center.into(),
                    align_y: iced::alignment::Vertical::Center.into(),
                    ..Default::default()
                });
                return vec![frame.into_geometry()];
            }
        };

        // Determine X range
        let x_min = dataset.x_trace.values.first().copied().unwrap_or(0.0);
        let x_max = dataset.x_trace.values.last().copied().unwrap_or(1.0);
        let x_span = if (x_max - x_min).abs() > 1e-15 {
            x_max - x_min
        } else {
            1.0
        };

        // Filter active traces
        let active_traces: Vec<&WaveformTrace> = if self.selected_traces.is_empty() {
            dataset.traces.iter().collect()
        } else {
            dataset
                .traces
                .iter()
                .filter(|t| self.selected_traces.contains(&t.name))
                .collect()
        };

        // Determine global Y range across active traces
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for trace in &active_traces {
            for &val in &trace.values {
                if !val.is_nan() && !val.is_infinite() {
                    if val < y_min {
                        y_min = val;
                    }
                    if val > y_max {
                        y_max = val;
                    }
                }
            }
        }

        if y_min == f64::MAX || y_max == f64::MIN || (y_max - y_min).abs() < 1e-12 {
            y_min -= 1.0;
            y_max += 1.0;
        } else {
            // Add 10% vertical padding
            let pad = (y_max - y_min) * 0.1;
            y_min -= pad;
            y_max += pad;
        }
        let y_span = y_max - y_min;

        // Render traces
        for (idx, trace) in active_traces.iter().enumerate() {
            let color = TRACE_COLORS[idx % TRACE_COLORS.len()];
            let n_pts = dataset.x_trace.values.len().min(trace.values.len());
            if n_pts < 2 {
                continue;
            }

            let mut builder = canvas::path::Builder::new();
            let mut started = false;

            for i in 0..n_pts {
                let x_val = dataset.x_trace.values[i];
                let y_val = trace.values[i];

                if y_val.is_nan() || y_val.is_infinite() {
                    continue;
                }

                let px = plot_rect.x + ((x_val - x_min) / x_span) as f32 * plot_rect.width;
                let py = plot_rect.y + plot_rect.height - ((y_val - y_min) / y_span) as f32 * plot_rect.height;

                let pt = Point::new(px, py);
                if !started {
                    builder.move_to(pt);
                    started = true;
                } else {
                    builder.line_to(pt);
                }
            }

            let path = builder.build();
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(color)
                    .with_width(1.5),
            );
        }

        // 3. Render Axis Labels
        let axis_label_color = Color::from_rgba(0.7, 0.75, 0.8, 1.0);
        let x_unit_suffix = dataset.x_trace.unit.suffix();

        // X Axis labels (bottom)
        for i in 0..=grid_divisions_x {
            let frac = i as f64 / grid_divisions_x as f64;
            let val = x_min + frac * x_span;
            let x = plot_rect.x + (frac as f32) * plot_rect.width;
            let label = format_axis_value(val, x_unit_suffix);

            frame.fill_text(canvas::Text {
                content: label,
                position: Point::new(x, plot_rect.y + plot_rect.height + 6.0),
                color: axis_label_color,
                size: iced::Pixels(9.0),
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Top.into(),
                ..Default::default()
            });
        }

        // Y Axis labels (left)
        for i in 0..=grid_divisions_y {
            let frac = i as f64 / grid_divisions_y as f64;
            let val = y_max - frac * y_span;
            let y = plot_rect.y + (frac as f32) * plot_rect.height;
            let label = format_axis_value(val, "V/A");

            frame.fill_text(canvas::Text {
                content: label,
                position: Point::new(plot_rect.x - 6.0, y),
                color: axis_label_color,
                size: iced::Pixels(9.0),
                align_x: iced::alignment::Horizontal::Right.into(),
                align_y: iced::alignment::Vertical::Center.into(),
                ..Default::default()
            });
        }

        // 4. Render Dual Measurement Cursors (Cursor A & Cursor B)
        if let Some(ca) = self.cursor_a {
            if ca >= x_min && ca <= x_max {
                let px = plot_rect.x + ((ca - x_min) / x_span) as f32 * plot_rect.width;
                let path = Path::line(
                    Point::new(px, plot_rect.y),
                    Point::new(px, plot_rect.y + plot_rect.height),
                );
                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(Color::from_rgb(0.9, 0.4, 0.1))
                        .with_width(1.0),
                );
                frame.fill_text(canvas::Text {
                    content: format!("A: {}", format_axis_value(ca, x_unit_suffix)),
                    position: Point::new(px + 4.0, plot_rect.y + 4.0),
                    color: Color::from_rgb(0.9, 0.4, 0.1),
                    size: iced::Pixels(9.0),
                    ..Default::default()
                });
            }
        }

        if let Some(cb) = self.cursor_b {
            if cb >= x_min && cb <= x_max {
                let px = plot_rect.x + ((cb - x_min) / x_span) as f32 * plot_rect.width;
                let path = Path::line(
                    Point::new(px, plot_rect.y),
                    Point::new(px, plot_rect.y + plot_rect.height),
                );
                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(Color::from_rgb(0.1, 0.7, 0.9))
                        .with_width(1.0),
                );
                frame.fill_text(canvas::Text {
                    content: format!("B: {}", format_axis_value(cb, x_unit_suffix)),
                    position: Point::new(px + 4.0, plot_rect.y + 16.0),
                    color: Color::from_rgb(0.1, 0.7, 0.9),
                    size: iced::Pixels(9.0),
                    ..Default::default()
                });
            }
        }

        vec![frame.into_geometry()]
    }
}

fn format_axis_value(val: f64, unit: &str) -> String {
    let abs = val.abs();
    if abs == 0.0 {
        return format!("0 {}", unit);
    }
    if abs >= 1e6 {
        format!("{:.2} M{}", val / 1e6, unit)
    } else if abs >= 1e3 {
        format!("{:.2} k{}", val / 1e3, unit)
    } else if abs >= 1.0 {
        format!("{:.2} {}", val, unit)
    } else if abs >= 1e-3 {
        format!("{:.2} m{}", val * 1e3, unit)
    } else if abs >= 1e-6 {
        format!("{:.2} µ{}", val * 1e6, unit)
    } else if abs >= 1e-9 {
        format!("{:.2} n{}", val * 1e9, unit)
    } else if abs >= 1e-12 {
        format!("{:.2} p{}", val * 1e12, unit)
    } else {
        format!("{:.2e} {}", val, unit)
    }
}
