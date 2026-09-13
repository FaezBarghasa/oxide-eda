use std::collections::HashMap;
use crate::geom::Point2D;

#[derive(Debug, Clone)]
pub struct PlacementSuggestion {
    pub component_id: u32,
    pub suggested_position: Point2D,
    pub confidence: f32,
}

pub struct PlacementAdvisor;

impl Default for PlacementAdvisor {
    fn default() -> Self {
        Self::new()
    }
}

impl PlacementAdvisor {
    pub fn new() -> Self {
        Self
    }

    /// Suggests component placement regions using connectivity force layout
    pub fn suggest_placement(
        &self,
        component_ids: &[u32],
        nets: &[(u32, Vec<u32>)], // (net_id, component_ids)
        current_positions: &HashMap<u32, Point2D>,
    ) -> Vec<PlacementSuggestion> {
        let mut suggestions = Vec::with_capacity(component_ids.len());

        for &comp_id in component_ids {
            let mut target_x = 0i64;
            let mut target_y = 0i64;
            let mut connected_count = 0i64;

            for (_net_id, connected_comps) in nets {
                if connected_comps.contains(&comp_id) {
                    for &other in connected_comps {
                        if other != comp_id {
                            if let Some(pos) = current_positions.get(&other) {
                                target_x += pos.x;
                                target_y += pos.y;
                                connected_count += 1;
                            }
                        }
                    }
                }
            }

            let suggested_pos = if connected_count > 0 {
                Point2D::new(target_x / connected_count, target_y / connected_count)
            } else {
                current_positions
                    .get(&comp_id)
                    .copied()
                    .unwrap_or(Point2D::ZERO)
            };

            suggestions.push(PlacementSuggestion {
                component_id: comp_id,
                suggested_position: suggested_pos,
                confidence: if connected_count > 0 { 0.85 } else { 0.5 },
            });
        }

        suggestions
    }
}
