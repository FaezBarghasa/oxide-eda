//! Canvas implementations for RF & Telecom panels (Smith Chart, Eye Diagram, Constellation).

use iced::mouse::Cursor;
use iced::widget::canvas::{self, Geometry, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};
use oxide_rf::constellation::ConstellationDataset;
use oxide_rf::eye_diagram::EyeDiagramDataset;
use oxide_rf::s_param::SParameterDataset;
use oxide_types::theme::ThemeTokens;

#[derive(Default)]
pub struct CanvasState {
    pub _cache: canvas::Cache,
}

// -----------------------------------------------------------------------------
// 1. Smith Chart Canvas
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct SmithChartCanvas<'a> {
    pub s_params: Option<&'a SParameterDataset>,
    pub tokens: &'a ThemeTokens,
}

impl<'a, Message> Program<Message, Theme, Renderer> for SmithChartCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Background
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgba(0.06, 0.07, 0.08, 1.0),
        );

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = (bounds.width.min(bounds.height) / 2.0 - 20.0).max(10.0);

        // Outer unity circle
        let outer_circle = Path::circle(center, radius);
        frame.stroke(
            &outer_circle,
            Stroke::default()
                .with_color(Color::from_rgba(0.4, 0.5, 0.6, 0.9))
                .with_width(1.5),
        );

        // Real axis horizontal line
        let real_axis = Path::line(
            Point::new(center.x - radius, center.y),
            Point::new(center.x + radius, center.y),
        );
        frame.stroke(
            &real_axis,
            Stroke::default()
                .with_color(Color::from_rgba(0.3, 0.4, 0.5, 0.6))
                .with_width(1.0),
        );

        // Constant Resistance Circles (r = 0.5, 1.0, 2.0)
        let r_values = [0.2, 0.5, 1.0, 2.0, 5.0];
        for &r in &r_values {
            let cx = center.x + radius * (r / (1.0 + r)) as f32;
            let cr = radius * (1.0 / (1.0 + r)) as f32;
            let c_path = Path::circle(Point::new(cx, center.y), cr);
            frame.stroke(
                &c_path,
                Stroke::default()
                    .with_color(Color::from_rgba(0.25, 0.3, 0.35, 0.5))
                    .with_width(0.75),
            );
        }

        // Plot S11 trace if available
        if let Some(net) = self.s_params {
            let mut builder = canvas::path::Builder::new();
            let mut started = false;

            for pt in &net.points {
                let gamma = pt.s11;
                let re = gamma.re as f32;
                let im = gamma.im as f32;

                let px = center.x + re * radius;
                let py = center.y - im * radius;

                let p = Point::new(px, py);
                if !started {
                    builder.move_to(p);
                    started = true;
                } else {
                    builder.line_to(p);
                }
            }

            let path = builder.build();
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(Color::from_rgb(0.2, 0.8, 1.0))
                    .with_width(2.0),
            );
        } else {
            frame.fill_text(canvas::Text {
                content: "Smith Chart: Run RF Simulation (F10) for S-Parameters".to_string(),
                position: Point::new(center.x, center.y),
                color: Color::from_rgba(0.6, 0.6, 0.6, 0.8),
                size: iced::Pixels(11.0),
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

// -----------------------------------------------------------------------------
// 2. Eye Diagram Canvas
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct EyeDiagramCanvas<'a> {
    pub eye_diagram: Option<&'a EyeDiagramDataset>,
    pub tokens: &'a ThemeTokens,
}

impl<'a, Message> Program<Message, Theme, Renderer> for EyeDiagramCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgba(0.04, 0.05, 0.06, 1.0),
        );

        let pad = 30.0;
        let plot_w = bounds.width - 2.0 * pad;
        let plot_h = bounds.height - 2.0 * pad;

        if let Some(eye) = self.eye_diagram {
            let trace_color = Color::from_rgba(0.1, 0.9, 0.4, 0.35); // Phosphor persistence green

            for trace in &eye.traces {
                if trace.voltage.len() < 2 {
                    continue;
                }
                let mut builder = canvas::path::Builder::new();
                let mut started = false;

                for (idx, &v) in trace.voltage.iter().enumerate() {
                    let frac_x = idx as f32 / (trace.voltage.len() - 1) as f32;
                    let px = pad + frac_x * plot_w;
                    // Normalized amplitude between -1.5 and 1.5
                    let norm_v = ((v + 1.5) / 3.0).clamp(0.0, 1.0) as f32;
                    let py = pad + plot_h - norm_v * plot_h;

                    let p = Point::new(px, py);
                    if !started {
                        builder.move_to(p);
                        started = true;
                    } else {
                        builder.line_to(p);
                    }
                }
                let path = builder.build();
                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(trace_color)
                        .with_width(1.0),
                );
            }
        } else {
            frame.fill_text(canvas::Text {
                content: "Eye Diagram: No traces folded. Run RF simulation to render.".to_string(),
                position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                color: Color::from_rgba(0.6, 0.6, 0.6, 0.8),
                size: iced::Pixels(11.0),
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

// -----------------------------------------------------------------------------
// 3. Constellation Canvas
// -----------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct ConstellationCanvas<'a> {
    pub constellation: Option<&'a ConstellationDataset>,
    pub tokens: &'a ThemeTokens,
}

impl<'a, Message> Program<Message, Theme, Renderer> for ConstellationCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color::from_rgba(0.05, 0.06, 0.07, 1.0),
        );

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let scale = (bounds.width.min(bounds.height) / 3.2).max(10.0);

        // I & Q axes
        let i_axis = Path::line(
            Point::new(10.0, center.y),
            Point::new(bounds.width - 10.0, center.y),
        );
        let q_axis = Path::line(
            Point::new(center.x, 10.0),
            Point::new(center.x, bounds.height - 10.0),
        );

        frame.stroke(
            &i_axis,
            Stroke::default()
                .with_color(Color::from_rgba(0.3, 0.35, 0.4, 0.6))
                .with_width(1.0),
        );
        frame.stroke(
            &q_axis,
            Stroke::default()
                .with_color(Color::from_rgba(0.3, 0.35, 0.4, 0.6))
                .with_width(1.0),
        );

        if let Some(cons) = self.constellation {
            for pt in &cons.points {
                // Render cloud points (received)
                let px = center.x + (pt.received.i as f32) * scale;
                let py = center.y - (pt.received.q as f32) * scale;

                let dot = Path::circle(Point::new(px, py), 1.5);
                frame.fill(
                    &dot,
                    Color::from_rgba(0.9, 0.7, 0.1, 0.6),
                );

                // Render ideal reference targets
                if let Some(ideal) = pt.ideal {
                    let ix = center.x + (ideal.i as f32) * scale;
                    let iy = center.y - (ideal.q as f32) * scale;

                    let target = Path::circle(Point::new(ix, iy), 4.0);
                    frame.stroke(
                        &target,
                        Stroke::default()
                            .with_color(Color::from_rgb(0.2, 0.8, 1.0))
                            .with_width(1.5),
                    );
                }
            }
        } else {
            frame.fill_text(canvas::Text {
                content: "Constellation (I/Q): Run RF simulation to render.".to_string(),
                position: Point::new(center.x, center.y),
                color: Color::from_rgba(0.6, 0.6, 0.6, 0.8),
                size: iced::Pixels(11.0),
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
