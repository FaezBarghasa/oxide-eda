use oxide_rules::{
    ClearanceRule, ConstraintManager, DesignRule, HighSpeedRule, RuleScope, RuleViolationType,
    WidthRule,
};

#[test]
fn test_trace_width_violation_0_10mm_violates_0_15mm_min_rule() {
    // 1. Setup constraint manager with standard 0.15mm (150 µm) minimum rule
    let cm = ConstraintManager::standard_default();

    // 2. Validate a valid 0.20mm (200 µm) trace -> PASS
    let valid_result = cm.validate_trace_width("NET_SIG1", None, None, 200);
    assert!(valid_result.is_ok(), "200µm trace should satisfy 150µm min rule");

    // 3. Validate a 0.10mm (100 µm) trace -> FAIL with exact violation details
    let invalid_result = cm.validate_trace_width("NET_SIG1", None, None, 100);
    assert!(invalid_result.is_err(), "100µm trace must violate 150µm min rule");

    let violation = invalid_result.unwrap_err();
    assert_eq!(violation.violation_type, RuleViolationType::WidthTooSmall);
    assert_eq!(violation.scope, RuleScope::Global);
    assert_eq!(violation.required_value, "0.150mm");
    assert_eq!(violation.actual_value, "0.100mm");
    assert!(
        violation.message.contains("trace width is 0.100mm (100µm)"),
        "Message should report actual measurement: {}",
        violation.message
    );
    assert!(
        violation.message.contains("minimum 0.150mm (150µm)"),
        "Message should report required measurement: {}",
        violation.message
    );
}

#[test]
fn test_hierarchical_net_class_override() {
    let mut cm = ConstraintManager::standard_default();

    // Add NetClass rule: "Power" nets require minimum 500 µm (0.5mm)
    cm.add_rule(DesignRule::Width(WidthRule::new(
        RuleScope::NetClass("Power".into()),
        500,
        750,
        3000,
    )));

    // A 300 µm trace on normal signal passes global 150 µm
    assert!(cm.validate_trace_width("SPI_CLK", None, None, 300).is_ok());

    // A 300 µm trace on a "Power" net class violates the 500 µm netclass rule
    let power_res = cm.validate_trace_width("+5V", Some("Power"), None, 300);
    assert!(power_res.is_err());
    let err = power_res.unwrap_err();
    assert_eq!(err.scope, RuleScope::NetClass("Power".into()));
    assert_eq!(err.required_value, "0.500mm");
}

#[test]
fn test_hierarchical_net_specific_override() {
    let mut cm = ConstraintManager::standard_default();

    // 1. NetClass "Power" -> min 500 µm
    cm.add_rule(DesignRule::Width(WidthRule::new(
        RuleScope::NetClass("Power".into()),
        500,
        750,
        3000,
    )));

    // 2. Net-specific override: "VBUS" -> min 1000 µm (1.0mm)
    cm.add_rule(DesignRule::Width(WidthRule::new(
        RuleScope::Net("VBUS".into()),
        1000,
        1500,
        4000,
    )));

    // 600 µm passes normal Power (500 µm min), but fails VBUS net-specific override (1000 µm min)
    let vbus_res = cm.validate_trace_width("VBUS", Some("Power"), None, 600);
    assert!(vbus_res.is_err());
    let err = vbus_res.unwrap_err();
    assert_eq!(err.scope, RuleScope::Net("VBUS".into()));
    assert_eq!(err.required_value, "1.000mm");
    assert_eq!(err.actual_value, "0.600mm");

    // 1200 µm passes VBUS
    assert!(cm.validate_trace_width("VBUS", Some("Power"), None, 1200).is_ok());
}

#[test]
fn test_clearance_matrix_evaluation() {
    let mut cm = ConstraintManager::standard_default();

    // High-Voltage NetClass (e.g. 230V Mains) requires 2.0mm (2000 µm) clearance to everything
    cm.add_rule(DesignRule::Clearance(ClearanceRule::new(
        RuleScope::NetClass("HV_Mains".into()),
        2000,
    )));

    // Normal signal to signal at 200 µm distance -> PASS (> 150 µm global)
    assert!(cm
        .validate_clearance("SIG_A", None, "SIG_B", None, None, 200)
        .is_ok());

    // Normal signal to signal at 100 µm distance -> FAIL (< 150 µm global)
    assert!(cm
        .validate_clearance("SIG_A", None, "SIG_B", None, None, 100)
        .is_err());

    // HV net to GND at 1000 µm (1.0mm) -> FAIL (< 2000 µm HV rule)
    let hv_res = cm.validate_clearance("MAINS_L", Some("HV_Mains"), "GND", None, None, 1000);
    assert!(hv_res.is_err());
    let err = hv_res.unwrap_err();
    assert_eq!(err.scope, RuleScope::NetClass("HV_Mains".into()));
    assert_eq!(err.required_value, "2.000mm");
    assert_eq!(err.actual_value, "1.000mm");
}

#[test]
fn test_high_speed_differential_rule_lookup() {
    let mut cm = ConstraintManager::new();
    cm.add_rule(DesignRule::HighSpeed(HighSpeedRule::new_diff_pair(
        "USB90", 90.0,
    )));

    let rule = cm
        .resolve_high_speed_rule("USB90")
        .expect("USB90 rule should exist");
    assert_eq!(rule.impedance_target, 90.0);
    assert_eq!(rule.length_tolerance, 50); // 50 µm
}
