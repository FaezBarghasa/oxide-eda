//! Hierarchical Constraint Manager and DRC rule evaluation engine.

use serde::{Deserialize, Serialize};

use oxide_physics::Microns;

use crate::rules::{ClearanceRule, DesignRule, HighSpeedRule, ViaStyleRule, WidthRule};
use crate::scope::RuleScope;
use crate::violation::RuleViolation;

/// Central constraint repository that resolves design rules hierarchically.
///
/// Rules are evaluated in strict order of specificity:
/// `Net` (4) > `NetClass` (3) > `Room` (2) > `Global` (1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConstraintManager {
    pub rules: Vec<DesignRule>,
}

impl ConstraintManager {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a design rule to the manager.
    pub fn add_rule(&mut self, rule: DesignRule) {
        self.rules.push(rule);
    }

    /// Standard baseline rules for typical 2-layer or 4-layer PCB fabrication
    /// (0.15mm min trace/space, 0.3mm drill / 0.6mm via pad).
    pub fn standard_default() -> Self {
        let mut cm = Self::new();
        // Global 0.15mm (150 µm) minimum trace width, 0.25mm preferred, 2.0mm max
        cm.add_rule(DesignRule::Width(WidthRule::new(
            RuleScope::Global,
            150,
            250,
            2000,
        )));
        // Global 0.15mm (150 µm) minimum electrical clearance
        cm.add_rule(DesignRule::Clearance(ClearanceRule::new(
            RuleScope::Global,
            150,
        )));
        // Global standard via style (0.3mm drill, 0.6mm diameter)
        cm.add_rule(DesignRule::ViaStyle(ViaStyleRule {
            scope: RuleScope::Global,
            min_drill: 300,
            min_diameter: 600,
            preferred_drill: 300,
            preferred_diameter: 600,
        }));
        cm
    }

    /// Resolve the most specific [`WidthRule`] that applies to a given net, net class, and room.
    pub fn resolve_width_rule(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
    ) -> Option<&WidthRule> {
        let mut candidates: Vec<&WidthRule> = self
            .rules
            .iter()
            .filter_map(|r| match r {
                DesignRule::Width(w) if w.scope.matches(net, net_class, room) => Some(w),
                _ => None,
            })
            .collect();

        // Sort descending by scope specificity (highest specificity first)
        candidates.sort_by_key(|w| std::cmp::Reverse(w.scope.specificity()));
        candidates.first().copied()
    }

    /// Resolve the most specific [`ClearanceRule`] for a given net, net class, and room.
    pub fn resolve_clearance_rule(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
    ) -> Option<&ClearanceRule> {
        let mut candidates: Vec<&ClearanceRule> = self
            .rules
            .iter()
            .filter_map(|r| match r {
                DesignRule::Clearance(c) if c.scope.matches(net, net_class, room) => Some(c),
                _ => None,
            })
            .collect();

        candidates.sort_by_key(|c| std::cmp::Reverse(c.scope.specificity()));
        candidates.first().copied()
    }

    /// Resolve high-speed routing rule for a given net class.
    pub fn resolve_high_speed_rule(&self, net_class: &str) -> Option<&HighSpeedRule> {
        self.rules.iter().find_map(|r| match r {
            DesignRule::HighSpeed(hs) if hs.net_class == net_class => Some(hs),
            _ => None,
        })
    }

    /// Validate that a routed trace width conforms to the hierarchical width rule.
    pub fn validate_trace_width(
        &self,
        net: &str,
        net_class: Option<&str>,
        room: Option<&str>,
        actual_width: Microns,
    ) -> Result<(), RuleViolation> {
        if let Some(rule) = self.resolve_width_rule(net, net_class, room) {
            if actual_width < rule.min_width {
                return Err(RuleViolation::width_too_small(
                    net,
                    rule.scope.clone(),
                    rule.min_width,
                    actual_width,
                ));
            }
        }
        Ok(())
    }

    /// Validate electrical clearance distance between two nets or objects.
    pub fn validate_clearance(
        &self,
        net_a: &str,
        class_a: Option<&str>,
        net_b: &str,
        class_b: Option<&str>,
        room: Option<&str>,
        actual_distance: Microns,
    ) -> Result<(), RuleViolation> {
        // Evaluate rules for both net_a and net_b, choosing the stricter (larger) requirement
        let rule_a = self.resolve_clearance_rule(net_a, class_a, room);
        let rule_b = self.resolve_clearance_rule(net_b, class_b, room);

        let required_clearance = match (rule_a, rule_b) {
            (Some(a), Some(b)) => {
                if a.min_distance >= b.min_distance {
                    (a.min_distance, a.scope.clone())
                } else {
                    (b.min_distance, b.scope.clone())
                }
            }
            (Some(a), None) => (a.min_distance, a.scope.clone()),
            (None, Some(b)) => (b.min_distance, b.scope.clone()),
            (None, None) => return Ok(()),
        };

        if actual_distance < required_clearance.0 {
            return Err(RuleViolation::clearance_violation(
                net_a,
                net_b,
                required_clearance.1,
                required_clearance.0,
                actual_distance,
            ));
        }

        Ok(())
    }
}
