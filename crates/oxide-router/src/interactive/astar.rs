//! Weighted A* pathfinding for PCB routing.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use oxide_physics::Microns;

use crate::geometry::rtree::{NetId, SpatialIndex};
use crate::geometry::{BoundingBox, Point2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridCoord {
    x: i64,
    y: i64,
}

/// Find a clear path from `start` to `target` using weighted A* on a routing grid.
pub fn find_astar_path(
    spatial_index: &SpatialIndex,
    start: Point2D,
    target: Point2D,
    net_id: NetId,
    width: Microns,
) -> Vec<Point2D> {
    if start == target {
        return vec![start];
    }

    let grid_step = 250i64; // 250 µm resolution (0.25mm)
    let start_coord = GridCoord {
        x: (start.x.0 / grid_step),
        y: (start.y.0 / grid_step),
    };
    let target_coord = GridCoord {
        x: (target.x.0 / grid_step),
        y: (target.y.0 / grid_step),
    };

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
    let max_iterations = 2000;

    while let Some(Reverse((_, current_g, current))) = open_set.pop() {
        iterations += 1;
        if iterations > max_iterations || current == target_coord {
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
            let pt = Point2D::new(
                Microns(neighbor.x * grid_step),
                Microns(neighbor.y * grid_step),
            );
            let half_w = Microns(width.0 / 2 + 100);
            let check_bbox = BoundingBox::from_center_radius(pt, half_w);
            let collisions = spatial_index.check_collision(&check_bbox, &[net_id]);

            let penalty = if !collisions.is_empty() {
                // High obstacle penalty
                100_000
            } else {
                0
            };

            let tentative_g = current_g + cost + penalty;
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i64::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let h = heuristic(neighbor, target_coord);
                open_set.push(Reverse((tentative_g + h, tentative_g, neighbor)));
            }
        }
    }

    // Reconstruct path
    let mut path = vec![target];
    let mut curr = target_coord;

    while let Some(&prev) = came_from.get(&curr) {
        let pt = Point2D::new(
            Microns(prev.x * grid_step),
            Microns(prev.y * grid_step),
        );
        path.push(pt);
        curr = prev;
    }

    path.push(start);
    path.reverse();

    // Simplify collinear points
    simplify_path(&path)
}

fn heuristic(a: GridCoord, b: GridCoord) -> i64 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    // Diagonal distance * 1000
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

        // If directions differ, keep waypoint
        if (dx1 - dx2).abs() > 1e-4 || (dy1 - dy2).abs() > 1e-4 {
            result.push(p1);
        }
    }

    result.push(*points.last().unwrap());
    result
}
