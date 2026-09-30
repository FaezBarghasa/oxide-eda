//! Altium Designer Design Rule Importer.
//!
//! Translates proprietary Altium Rule records into native [`oxide_rules::ConstraintManager`].

use oxide_rules::{
    ClearanceRule, ConstraintManager, DesignRule, HighSpeedRule,
    RuleScope, WidthRule,
};
use crate::record::AltiumRecord;

/// Parse Altium Design Rule records into a native [`ConstraintManager`].
pub fn import_rules_from_records(records: &[AltiumRecord]) -> ConstraintManager {
    let mut manager = ConstraintManager::new();

    for rec in records {
        let record_type = rec.get("RECORD").or_else(|| rec.get("OBJECTTYPE")).unwrap_or("");
        if !record_type.eq_ignore_ascii_case("Rule") && record_type != "17" {
            continue;
        }

        let rule_kind = rec.get("RULEKIND").or_else(|| rec.get("NAME")).unwrap_or("");
        let scope_1 = rec.get("SCOPE1EXPRESSION").or_else(|| rec.get("SCOPE1")).unwrap_or("All");
        let scope = parse_altium_scope(scope_1);

        if rule_kind.contains("Clearance") {
            let gap_mm = rec.get_coord_mm("GAP").unwrap_or(0.15);
            let gap_um = (gap_mm * 1000.0).round() as i64;
            manager.add_rule(DesignRule::Clearance(ClearanceRule::new(scope, gap_um)));
        } else if rule_kind.contains("Width") {
            let min_w = rec.get_coord_mm("MINWIDTH").unwrap_or(0.15);
            let opt_w = rec.get_coord_mm("FAVOREDWIDTH").unwrap_or(0.20);
            let max_w = rec.get_coord_mm("MAXWIDTH").unwrap_or(0.50);

            manager.add_rule(DesignRule::Width(WidthRule::new(
                scope,
                (min_w * 1000.0).round() as i64,
                (opt_w * 1000.0).round() as i64,
                (max_w * 1000.0).round() as i64,
            )));
        } else if rule_kind.contains("DiffPairsRouting") || rule_kind.contains("MatchedLength") {
            let max_tol = rec.get_coord_mm("TOLERANCE").unwrap_or(0.10);
            let net_class = match &scope {
                RuleScope::NetClass(cls) => cls.clone(),
                RuleScope::Net(n) => n.clone(),
                _ => "DIFF_PAIR".to_string(),
            };
            manager.add_rule(DesignRule::HighSpeed(HighSpeedRule {
                net_class,
                impedance_target: 90.0,
                length_tolerance: (max_tol * 1000.0).round() as i64,
                max_uncoupled_length: 500,
            }));
        }
    }

    if manager.rules.is_empty() {
        manager = ConstraintManager::standard_default();
    }

    manager
}

fn parse_altium_scope(expr: &str) -> RuleScope {
    let clean = expr.trim();
    if clean.eq_ignore_ascii_case("All") || clean.is_empty() {
        RuleScope::Global
    } else if clean.to_lowercase().starts_with("innet('") {
        let net = clean.trim_start_matches("InNet('").trim_start_matches("innet('").trim_end_matches("')");
        RuleScope::Net(net.to_string())
    } else if clean.to_lowercase().starts_with("innetclass('") {
        let cls = clean.trim_start_matches("InNetClass('").trim_start_matches("innetclass('").trim_end_matches("')");
        RuleScope::NetClass(cls.to_string())
    } else {
        RuleScope::Global
    }
}
