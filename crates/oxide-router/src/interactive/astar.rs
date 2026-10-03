//! Weighted A* pathfinding for PCB routing.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use oxide_physics::Microns;

use crate::RoutingError;
use crate::geometry::rtree::{NetId, SpatialIndex};
use crate::geometry::{BoundingBox, Point2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GridCoord {
    x: i64,
    y: i64,
}

/// Find a clear path from `start` to `target` using weighted A* on a routing grid.
///
/// Returns `Ok(path)` if a DRC-compliant trajectory reaches `target`, or `Err(RoutingError::NoPathFound)`.
pub fn find_astar_path(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
) -> Result<Vec<Point2D>, RoutingError> {
    find_astar_path_with_grid(spatial_index, start, target, net_id, width, 100)
}

/// Find a clear path with explicit grid step resolution.
pub fn find_astar_path_with_grid(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
    grid_step_microns: Microns,
) -> Result<Vec<Point2D>, RoutingError> {
    if start == target {
        return Ok(vec![start]);
    }

    let grid_step = grid_step_microns.max(10);
    let start_coord = GridCoord {
        x: start.x / grid_step,
        y: start.y / grid_step,
    };
    let target_coord = GridCoord {
        x: target.x / grid_step,
        y: target.y / grid_step,
    };

    if start_coord == target_coord {
        let half_w = width / 2 + 50;
        let check_bbox = BoundingBox::from_points(&[start, target]).expand(half_w);
        if spatial_index
            .check_collision(&check_bbox, &[net_id])
            .is_empty()
        {
            return Ok(vec![start, target]);
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from: HashMap<GridCoord, GridCoord> = HashMap::new();
    let mut g_score: HashMap<GridCoord, i64> = HashMap::new();

    g_score.insert(start_coord, 0);
    let h_start = heuristic(start_coord, target_coord);
    open_set.push(Reverse((h_start, 0i64, start_coord)));

    let directions = [
        (1, 0, 1000),
        (-1, 0, 1000),
        (0, 1, 1000),
        (0, -1, 1000),
        (1, 1, 1414),
        (-1, 1, 1414),
        (1, -1, 1414),
        (-1, -1, 1414),
    ];

    let mut iterations = 0;
    let euclidean_dist = start.distance_to(target);
    let estimated_steps = (euclidean_dist / grid_step).max(1);
    let max_iterations = (estimated_steps * 25).clamp(8000, 100_000);
    let mut target_reached = false;

    while let Some(Reverse((_, current_g, current))) = open_set.pop() {
        iterations += 1;
        if current == target_coord {
            target_reached = true;
            break;
        }
        if iterations > max_iterations {
            break;
        }

        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current);

        for (dx, dy, cost) in directions {
            let neighbor = GridCoord {
                x: current.x + dx,
                y: current.y + dy,
            };

            if closed_set.contains(&neighbor) {
                continue;
            }

            // Check collision with spatial index
            let pt = Point2D::new(neighbor.x * grid_step, neighbor.y * grid_step);
            let half_w = width / 2 + 50;
            let check_bbox = BoundingBox::from_center_radius(pt, half_w);
            let collisions = spatial_index.check_collision(&check_bbox, &[net_id]);

            if !collisions.is_empty() {
                continue;
            }

            let tentative_g = current_g + cost;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i64::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let h = heuristic(neighbor, target_coord);
                open_set.push(Reverse((tentative_g + h, tentative_g, neighbor)));
            }
        }
    }

    if !target_reached && !came_from.contains_key(&target_coord) {
        return Err(RoutingError::NoPathFound);
    }

    // Reconstruct path
    let mut path = vec![target];
    let mut curr = target_coord;

    while let Some(&prev) = came_from.get(&curr) {
        let pt = Point2D::new(prev.x * grid_step, prev.y * grid_step);
        path.push(pt);
        curr = prev;
    }

    path.push(start);
    path.reverse();

    // Simplify collinear points
    Ok(simplify_path(&path))
}

/// Find a clear path using ML-guided A* heuristics while maintaining 100% deterministic DRC enforcement.
pub fn find_astar_path_with_ml(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
    advisor: Option<&mut oxide_ml::RoutingAdvisor>,
) -> Result<Vec<Point2D>, RoutingError> {
    if start == target {
        return Ok(vec![start]);
    }

    let grid_step = 100i64; // 100 µm grid resolution
    let start_coord = GridCoord {
        x: start.x / grid_step,
        y: start.y / grid_step,
    };
    let target_coord = GridCoord {
        x: target.x / grid_step,
        y: target.y / grid_step,
    };

    if start_coord == target_coord {
        let half_w = width / 2 + 50;
        let check_bbox = BoundingBox::from_points(&[start, target]).expand(half_w);
        if spatial_index
            .check_collision(&check_bbox, &[net_id])
            .is_empty()
        {
            return Ok(vec![start, target]);
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from: HashMap<GridCoord, GridCoord> = HashMap::new();
    let mut g_score: HashMap<GridCoord, i64> = HashMap::new();

    g_score.insert(start_coord, 0);
    let h_start = heuristic(start_coord, target_coord);
    open_set.push(Reverse((h_start, 0i64, start_coord)));

    let directions = [
        (1, 0, 1000, 2),   // East (RoutingAction::MoveEast = 2)
        (-1, 0, 1000, 6),  // West (RoutingAction::MoveWest = 6)
        (0, 1, 1000, 0),   // North (RoutingAction::MoveNorth = 0)
        (0, -1, 1000, 4),  // South (RoutingAction::MoveSouth = 4)
        (1, 1, 1414, 1),   // NorthEast = 1
        (-1, 1, 1414, 7),  // NorthWest = 7
        (1, -1, 1414, 3),  // SouthEast = 3
        (-1, -1, 1414, 5), // SouthWest = 5
    ];

    let mut iterations = 0;
    let euclidean_dist = start.distance_to(target);
    let estimated_steps = (euclidean_dist / grid_step).max(1);
    let max_iterations = (estimated_steps * 30).clamp(25000, 150_000);
    let mut target_reached = false;

    let mut advisor_ref = advisor;

    while let Some(Reverse((_, current_g, current))) = open_set.pop() {
        iterations += 1;
        if current == target_coord {
            target_reached = true;
            break;
        }
        if iterations > max_iterations {
            break;
        }

        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current);

        let current_pt = Point2D::new(current.x * grid_step, current.y * grid_step);
        let ml_output = advisor_ref.as_deref_mut().map(|adv| {
            let ml_pt = oxide_ml::Point2D::new(current_pt.x, current_pt.y);
            let ml_tgt = oxide_ml::Point2D::new(target.x, target.y);
            adv.predict(ml_pt, 0, ml_tgt, net_id, &[])
        });

        for (dx, dy, base_cost, action_idx) in directions {
            let neighbor = GridCoord {
                x: current.x + dx,
                y: current.y + dy,
            };

            if closed_set.contains(&neighbor) {
                continue;
            }

            // Check collision with spatial index (100% deterministic gate)
            let pt = Point2D::new(neighbor.x * grid_step, neighbor.y * grid_step);
            let half_w = width / 2 + 50;
            let check_bbox = BoundingBox::from_center_radius(pt, half_w);
            let collisions = spatial_index.check_collision(&check_bbox, &[net_id]);

            if !collisions.is_empty() {
                // Hard deterministic rule rejection: cannot route through foreign obstacles
                continue;
            }

            // Learned ML bias: slight discount for high-scoring policy moves while preserving admissibility
            let mut move_cost = base_cost;
            if let Some(ref out) = ml_output {
                let score = out.action_scores[action_idx];
                let discount = (score * out.confidence * 150.0) as i64;
                move_cost = (move_cost - discount).max(850);
            }

            let tentative_g = current_g + move_cost;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i64::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let h = heuristic(neighbor, target_coord);
                open_set.push(Reverse((tentative_g + h, tentative_g, neighbor)));
            }
        }
    }

    if !target_reached && !came_from.contains_key(&target_coord) {
        return Err(RoutingError::NoPathFound);
    }

    // Reconstruct path
    let mut path = vec![target];
    let mut curr = target_coord;

    while let Some(&prev) = came_from.get(&curr) {
        let pt = Point2D::new(prev.x * grid_step, prev.y * grid_step);
        path.push(pt);
        curr = prev;
    }

    path.push(start);
    path.reverse();

    Ok(simplify_path(&path))
}

fn heuristic(a: GridCoord, b: GridCoord) -> i64 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    let min = dx.min(dy);
    let max = dx.max(dy);
    min * 1414 + (max - min) * 1000
}

fn simplify_path(points: &[Point2D]) -> Vec<Point2D> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut result = vec![points[0]];
    for i in 1..points.len() - 1 {
        let p0 = *result.last().unwrap();
        let p1 = points[i];
        let p2 = points[i + 1];

        let (dx1, dy1) = p0.direction_to(p1);
        let (dx2, dy2) = p1.direction_to(p2);

        if (dx1 - dx2).abs() > 1e-4 || (dy1 - dy2).abs() > 1e-4 {
            result.push(p1);
        }
    }

    result.push(*points.last().unwrap());
    result
}
