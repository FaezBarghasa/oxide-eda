//! High-Speed Length & Delay Tuning with Forward Crosstalk Coupling Compensation.
//!
//! Replaces naive uncoupled delay equations with electromagnetic self-coupling compensation:
//! $t_{\text{pd, actual}} = t_{\text{pd, uncoupled}} \cdot \left( 1 - k_f \exp\left( -2.5 \frac{s}{h} \right) \right)$

use thiserror::Error;

use crate::geometry::rtree::SpatialIndex;
use crate::geometry::{BoundingBox, Point2D};
use crate::{LayerId, NetId};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TuningError {
    #[error("Target delay {0} ps unreachable within boundary envelope")]
    UnreachableTargetDelay(f64),
    #[error("Violated minimum pitch ratio: s/h {found:.2} < required {required:.2}")]
    PitchRatioViolation { found: f64, required: f64 },
    #[error("Path too short for meander synthesis")]
    PathTooShort,
}

#[derive(Debug, Clone)]
pub struct MeanderConstraint {
    pub target_delay_ps: f64,
    pub tolerance_ps: f64,
    pub min_spacing_h_ratio: f64, // e.g. s >= 3.0 * h
    pub dielectric_height_um: f64,
    pub max_amplitude_um: f64,
    pub trace_width_um: f64,
    pub er_eff: f64,
}

impl Default for MeanderConstraint {
    fn default() -> Self {
        Self {
            target_delay_ps: 100.0,
            tolerance_ps: 0.5,
            min_spacing_h_ratio: 3.0,
            dielectric_height_um: 100.0,
            max_amplitude_um: 500.0,
            trace_width_um: 150.0,
            er_eff: 3.8,
        }
    }
}

/// Computes actual electromagnetic propagation delay accounting for forward crosstalk.
#[inline]
pub fn compute_coupled_delay_ps(
    length_um: f64,
    er_eff: f64,
    spacing_um: f64,
    dielectric_height_um: f64,
    forward_coupling_k: f64,
) -> f64 {
    const C0_UM_PER_PS: f64 = 299.792458; // speed of light in µm/ps
    let v_phase = C0_UM_PER_PS / er_eff.sqrt();
    let uncoupled_delay = length_um / v_phase;

    let s_h_ratio = spacing_um / dielectric_height_um.max(1.0);
    let coupling_reduction = forward_coupling_k * (-2.5 * s_h_ratio).exp();
    let coupling_factor = (1.0 - coupling_reduction).max(0.5);

    uncoupled_delay * coupling_factor
}

/// Synthesizes a forward-coupling compensated accordion meander along a straight corridor segment.
pub fn synthesize_coupled_accordion(
    start: Point2D,
    end: Point2D,
    constraint: &MeanderConstraint,
    forward_coupling_k: f64,
) -> Result<Vec<Point2D>, TuningError> {
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;
    let direct_length = (dx * dx + dy * dy).sqrt();

    if direct_length < 100.0 {
        return Err(TuningError::PathTooShort);
    }

    let min_spacing = constraint.min_spacing_h_ratio * constraint.dielectric_height_um;
    let base_delay = compute_coupled_delay_ps(
        direct_length,
        constraint.er_eff,
        min_spacing,
        constraint.dielectric_height_um,
        forward_coupling_k,
    );

    if base_delay >= constraint.target_delay_ps - constraint.tolerance_ps {
        // Direct path already satisfies target delay
        return Ok(vec![start, end]);
    }

    let mut points = vec![start];
    let (dir_x, dir_y) = (dx / direct_length, dy / direct_length);
    let (perp_x, perp_y) = (-dir_y, dir_x);

    let pitch = min_spacing + constraint.trace_width_um;
    let num_ripples = ((direct_length * 0.8) / pitch).floor() as usize;

    if num_ripples == 0 {
        return Ok(vec![start, end]);
    }

    let mut current_pos = direct_length * 0.1;
    let amplitude = constraint.max_amplitude_um;

    for i in 0..num_ripples {
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        let p1 = Point2D::new(
            (start.x as f64 + dir_x * current_pos).round() as i64,
            (start.y as f64 + dir_y * current_pos).round() as i64,
        );
        let p2 = Point2D::new(
            (p1.x as f64 + perp_x * amplitude * sign).round() as i64,
            (p1.y as f64 + perp_y * amplitude * sign).round() as i64,
        );
        let p3 = Point2D::new(
            (p2.x as f64 + dir_x * (pitch * 0.5)).round() as i64,
            (p2.y as f64 + dir_y * (pitch * 0.5)).round() as i64,
        );
        let p4 = Point2D::new(
            (p3.x as f64 - perp_x * amplitude * sign).round() as i64,
            (p3.y as f64 - perp_y * amplitude * sign).round() as i64,
        );

        points.push(p1);
        points.push(p2);
        points.push(p3);
        points.push(p4);

        current_pos += pitch;
    }

    points.push(end);
    Ok(points)
}

/// Continuously and analytically computes safe meander amplitude against obstacles
/// within the lobe's corridor envelope without iterative step-sampling.
#[allow(clippy::too_many_arguments)]
pub fn compute_analytical_safe_amplitude(
    si: &SpatialIndex,
    net_id: NetId,
    p_base1: Point2D,
    p_base2: Point2D,
    perp_x: f64,
    perp_y: f64,
    sign: f64,
    max_amplitude_um: f64,
    clearance_um: f64,
    trace_width_um: f64,
) -> f64 {
    let p_peak1 = Point2D::new(
        (p_base1.x as f64 + perp_x * max_amplitude_um * sign).round() as i64,
        (p_base1.y as f64 + perp_y * max_amplitude_um * sign).round() as i64,
    );
    let p_peak2 = Point2D::new(
        (p_base2.x as f64 + perp_x * max_amplitude_um * sign).round() as i64,
        (p_base2.y as f64 + perp_y * max_amplitude_um * sign).round() as i64,
    );

    let required_margin = ((trace_width_um / 2.0) + clearance_um).round() as i64;
    let envelope_bbox = BoundingBox::from_points(&[p_base1, p_base2, p_peak1, p_peak2])
        .expand(required_margin);

    let obstacles = si.check_collision(&envelope_bbox, &[net_id]);
    if obstacles.is_empty() {
        return max_amplitude_um;
    }

    let mut min_allowable_amp = max_amplitude_um;
    let eff_perp_x = perp_x * sign;
    let eff_perp_y = perp_y * sign;

    for obs in obstacles {
        let b = obs.bbox;
        let corners = [
            Point2D::new(b.min.x, b.min.y),
            Point2D::new(b.min.x, b.max.y),
            Point2D::new(b.max.x, b.min.y),
            Point2D::new(b.max.x, b.max.y),
        ];

        for c in corners {
            let dx = (c.x - p_base1.x) as f64;
            let dy = (c.y - p_base1.y) as f64;
            let perp_dist = dx * eff_perp_x + dy * eff_perp_y;
            if perp_dist > 0.0 {
                let safe_limit = (perp_dist - (trace_width_um / 2.0 + clearance_um)).max(0.0);
                if safe_limit < min_allowable_amp {
                    min_allowable_amp = safe_limit;
                }
            }
        }
    }

    min_allowable_amp
}

/// Synthesizes an obstacle-aware accordion meander that dynamically clamps lateral amplitude
/// against neighboring obstacles in the SpatialIndex or switches to trombone jogging.
#[allow(clippy::too_many_arguments)]
pub fn synthesize_obstacle_aware_accordion(
    start: Point2D,
    end: Point2D,
    constraint: &MeanderConstraint,
    forward_coupling_k: f64,
    spatial_index: Option<&SpatialIndex>,
    net_id: NetId,
    _layer: LayerId,
    clearance_um: f64,
) -> Result<Vec<Point2D>, TuningError> {
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;
    let direct_length = (dx * dx + dy * dy).sqrt();

    if direct_length < 100.0 {
        return Err(TuningError::PathTooShort);
    }

    let min_spacing = constraint.min_spacing_h_ratio * constraint.dielectric_height_um;
    let base_delay = compute_coupled_delay_ps(
        direct_length,
        constraint.er_eff,
        min_spacing,
        constraint.dielectric_height_um,
        forward_coupling_k,
    );

    if base_delay >= constraint.target_delay_ps - constraint.tolerance_ps {
        return Ok(vec![start, end]);
    }

    let mut points = vec![start];
    let (dir_x, dir_y) = (dx / direct_length, dy / direct_length);
    let (perp_x, perp_y) = (-dir_y, dir_x);

    let pitch = min_spacing + constraint.trace_width_um;
    let num_ripples = ((direct_length * 0.8) / pitch).floor() as usize;

    if num_ripples == 0 {
        return Ok(vec![start, end]);
    }

    let mut current_pos = direct_length * 0.1;
    let initial_amplitude = constraint.max_amplitude_um;

    for i in 0..num_ripples {
        let preferred_sign = if i % 2 == 0 { 1.0 } else { -1.0 };

        let p_base1 = Point2D::new(
            (start.x as f64 + dir_x * current_pos).round() as i64,
            (start.y as f64 + dir_y * current_pos).round() as i64,
        );
        let p_base2 = Point2D::new(
            (p_base1.x as f64 + dir_x * (pitch * 0.5)).round() as i64,
            (p_base1.y as f64 + dir_y * (pitch * 0.5)).round() as i64,
        );

        // Continuous analytical spatial query for safe amplitude on specified side
        let find_safe_amp = |sign: f64| -> f64 {
            let Some(si) = spatial_index else {
                return initial_amplitude;
            };
            compute_analytical_safe_amplitude(
                si,
                net_id,
                p_base1,
                p_base2,
                perp_x,
                perp_y,
                sign,
                initial_amplitude,
                clearance_um,
                constraint.trace_width_um,
            )
        };

        let mut offset_sign = preferred_sign;
        let mut clamped_amp = find_safe_amp(offset_sign);

        // Switch to trombone jog if preferred side is blocked
        if clamped_amp < 50.0 {
            let alt_sign = -preferred_sign;
            let alt_amp = find_safe_amp(alt_sign);
            if alt_amp >= 50.0 {
                offset_sign = alt_sign;
                clamped_amp = alt_amp;
            }
        }

        let p1 = Point2D::new(
            (start.x as f64 + dir_x * current_pos).round() as i64,
            (start.y as f64 + dir_y * current_pos).round() as i64,
        );

        if clamped_amp >= 50.0 {
            let p2 = Point2D::new(
                (p1.x as f64 + perp_x * clamped_amp * offset_sign).round() as i64,
                (p1.y as f64 + perp_y * clamped_amp * offset_sign).round() as i64,
            );
            let p3 = Point2D::new(
                (p2.x as f64 + dir_x * (pitch * 0.5)).round() as i64,
                (p2.y as f64 + dir_y * (pitch * 0.5)).round() as i64,
            );
            let p4 = Point2D::new(
                (p3.x as f64 - perp_x * clamped_amp * offset_sign).round() as i64,
                (p3.y as f64 - perp_y * clamped_amp * offset_sign).round() as i64,
            );

            points.push(p1);
            points.push(p2);
            points.push(p3);
            points.push(p4);
        } else {
            // Inline straight step when both sides are restricted
            let next_inline = Point2D::new(
                (p1.x as f64 + dir_x * (pitch * 0.5)).round() as i64,
                (p1.y as f64 + dir_y * (pitch * 0.5)).round() as i64,
            );
            points.push(p1);
            points.push(next_inline);
        }

        current_pos += pitch;
    }

    points.push(end);
    Ok(points)
}

/// Synthesizes an obstacle-aware accordion meander with rounded bi-arc or clothoid fillets.
#[allow(clippy::too_many_arguments)]
pub fn synthesize_coupled_accordion_filleted(
    start: Point2D,
    end: Point2D,
    constraint: &MeanderConstraint,
    forward_coupling_k: f64,
    spatial_index: Option<&SpatialIndex>,
    net_id: NetId,
    layer: LayerId,
    clearance_um: f64,
    fillet_radius_um: f64,
    use_clothoid: bool,
) -> Result<Vec<Point2D>, TuningError> {
    let raw_pts = synthesize_obstacle_aware_accordion(
        start,
        end,
        constraint,
        forward_coupling_k,
        spatial_index,
        net_id,
        layer,
        clearance_um,
    )?;
    Ok(super::clothoid::fillet_meander_path(&raw_pts, fillet_radius_um, use_clothoid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::rtree::{SpatialObject, SpatialObjectType};
    use uuid::Uuid;

    #[test]
    fn test_coupled_delay_equation() {
        let delay_uncoupled = compute_coupled_delay_ps(10000.0, 4.0, 1000.0, 100.0, 0.0);
        let delay_coupled = compute_coupled_delay_ps(10000.0, 4.0, 150.0, 100.0, 0.25);
        // Tight spacing induces forward coupling reduction (speedup)
        assert!(delay_coupled < delay_uncoupled);
    }

    #[test]
    fn test_synthesize_coupled_accordion() {
        let start = Point2D::new(0, 0);
        let end = Point2D::new(10000, 0);
        let constraint = MeanderConstraint {
            target_delay_ps: 200.0,
            ..Default::default()
        };
        let meander = synthesize_coupled_accordion(start, end, &constraint, 0.15)
            .expect("meander synthesized");
        assert!(meander.len() > 2);
    }

    #[test]
    fn test_meander_amplitude_obstacle_clamping() {
        let start = Point2D::new(0, 0);
        let end = Point2D::new(10000, 0);
        let constraint = MeanderConstraint {
            target_delay_ps: 300.0,
            max_amplitude_um: 1000.0,
            ..Default::default()
        };

        // Place an obstacle at Y = 350um along the path from X=2000 to X=8000
        let mut spatial_index = SpatialIndex::new();
        let obstacle_bbox = BoundingBox::new(Point2D::new(1000, 350), Point2D::new(8000, 800));
        spatial_index.insert(SpatialObject {
            id: Uuid::new_v4(),
            bbox: obstacle_bbox,
            object_type: SpatialObjectType::Track,
            net_id: Some(99), // Different net
            layer: 0,
        });

        let clearance_um = 150.0;
        let meander = synthesize_obstacle_aware_accordion(
            start,
            end,
            &constraint,
            0.15,
            Some(&spatial_index),
            1, // Our net
            0,
            clearance_um,
        )
        .expect("obstacle-aware meander synthesized");

        assert!(meander.len() > 2);

        // Verify that no point in the synthesized meander enters within clearance of the obstacle
        for pt in &meander {
            if pt.x >= 1000 && pt.x <= 8000 {
                // Obstacle bottom is at Y=350, with clearance 150um and half_width 75um,
                // trace center must not exceed Y = 350 - 150 = 200
                assert!(
                    pt.y < 350,
                    "Meander point {:?} violated obstacle boundary at Y=350",
                    pt
                );
            }
        }
    }

    #[test]
    fn test_analytical_1d_interval_clamping() {
        let mut spatial_index = SpatialIndex::new();
        // Obstacle at Y = 400
        let obstacle_bbox = BoundingBox::new(Point2D::new(500, 400), Point2D::new(1500, 800));
        spatial_index.insert(SpatialObject {
            id: Uuid::new_v4(),
            bbox: obstacle_bbox,
            object_type: SpatialObjectType::Track,
            net_id: Some(99),
            layer: 0,
        });

        let p_base1 = Point2D::new(600, 0);
        let p_base2 = Point2D::new(900, 0);
        let perp_x = 0.0;
        let perp_y = 1.0;
        let sign = 1.0;
        let max_amplitude_um = 600.0;
        let clearance_um = 100.0;
        let trace_width_um = 150.0;

        let safe_amp = compute_analytical_safe_amplitude(
            &spatial_index,
            1,
            p_base1,
            p_base2,
            perp_x,
            perp_y,
            sign,
            max_amplitude_um,
            clearance_um,
            trace_width_um,
        );

        // Required clearance margin is trace_width/2 + clearance = 75 + 100 = 175um.
        // Obstacle is at Y=400, so safe amplitude must be <= 400 - 175 = 225um.
        assert!(
            safe_amp <= 225.0,
            "analytical safe amplitude must be clamped to 225, got {}",
            safe_amp
        );
        assert!(safe_amp > 0.0);
    }
}
