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
    assert!(
        valid_result.is_ok(),
        "200µm trace should satisfy 150µm min rule"
    );

    // 3. Validate a 0.10mm (100 µm) trace -> FAIL with exact violation details
    let invalid_result = cm.validate_trace_width("NET_SIG1", None, None, 100);
    assert!(
        invalid_result.is_err(),
        "100µm trace must violate 150µm min rule"
    );

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
    assert!(
        cm.validate_trace_width("VBUS", Some("Power"), None, 1200)
            .is_ok()
    );
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
    assert!(
        cm.validate_clearance("SIG_A", None, "SIG_B", None, None, 200)
            .is_ok()
    );

    // Normal signal to signal at 100 µm distance -> FAIL (< 150 µm global)
    assert!(
        cm.validate_clearance("SIG_A", None, "SIG_B", None, None, 100)
            .is_err()
    );

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

#[test]
fn test_toml_schema_deserialization_and_evaluation() {
    let toml_doc = r#"
version = "1.0"
profile_name = "IPC-2221-Class2-4Layer-Standard"

[[rules]]
rule_type = "clearance"
scope = { type = "global" }
min_distance = 150
object_types = ["track", "via", "pad", "polygon"]

[[rules]]
rule_type = "clearance"
scope = { type = "net_class", name = "POWER_48V" }
min_distance = 800
object_types = ["track", "via", "pad", "polygon", "hole"]

[[rules]]
rule_type = "width"
scope = { type = "global" }
min_width = 150
preferred_width = 200
max_width = 500

[[rules]]
rule_type = "width"
scope = { type = "net_class", name = "POWER_RAILS" }
min_width = 500
preferred_width = 1000
max_width = 3000

[[rules]]
rule_type = "high_speed"
net_class = "DIFF_USB2_90R"
impedance_target = 90.0
length_tolerance = 50
max_uncoupled_length = 500

[[rules]]
rule_type = "via_style"
scope = { type = "global" }
min_drill = 300
min_diameter = 600
preferred_drill = 300
preferred_diameter = 600

[[rules]]
rule_type = "polygon_connect"
scope = { type = "global" }
direct_connect = false
spoke_count = 4
min_spoke_width = 250
air_gap = 200
"#;

    let cm = ConstraintManager::from_toml_str(toml_doc).expect("Failed to parse TOML");
    assert_eq!(cm.rules.len(), 7);

    // Test clearance validation
    assert!(
        cm.validate_clearance("SIG1", None, "SIG2", None, None, 180)
            .is_ok()
    );
    assert!(
        cm.validate_clearance("SIG1", None, "SIG2", None, None, 120)
            .is_err()
    );
    assert!(
        cm.validate_clearance("V_IN", Some("POWER_48V"), "GND", None, None, 500)
            .is_err()
    );
    assert!(
        cm.validate_clearance("V_IN", Some("POWER_48V"), "GND", None, None, 900)
            .is_ok()
    );

    // Test width validation
    assert!(cm.validate_trace_width("SIG1", None, None, 200).is_ok());
    assert!(
        cm.validate_trace_width("+12V", Some("POWER_RAILS"), None, 400)
            .is_err()
    );
    assert!(
        cm.validate_trace_width("+12V", Some("POWER_RAILS"), None, 600)
            .is_ok()
    );

    // Test high speed rule resolution
    let hs = cm
        .resolve_high_speed_rule("DIFF_USB2_90R")
        .expect("DIFF_USB2_90R rule should exist");
    assert_eq!(hs.impedance_target, 90.0);

    // Test via style and polygon connect resolution
    let via = cm
        .resolve_via_style_rule("NET1", None, None)
        .expect("Via rule should exist");
    assert_eq!(via.min_drill, 300);

    let poly = cm
        .resolve_polygon_connect_rule("GND", None, None)
        .expect("Poly connect rule should exist");
    assert_eq!(poly.spoke_count, 4);
}

#[test]
fn test_toml_roundtrip_serialization() {
    let cm = ConstraintManager::standard_default();
    let toml_str = cm.to_toml_string().expect("Failed to serialize to TOML");
    assert!(toml_str.contains("rule_type = \"width\""));
    assert!(toml_str.contains("rule_type = \"clearance\""));
    assert!(toml_str.contains("rule_type = \"via_style\""));
    assert!(toml_str.contains("rule_type = \"solder_mask\""));
    assert!(toml_str.contains("rule_type = \"silkscreen\""));
    assert!(toml_str.contains("rule_type = \"net_antenna\""));

    let deserialized =
        ConstraintManager::from_toml_str(&toml_str).expect("Failed to deserialize from TOML");
    assert_eq!(deserialized.rules.len(), cm.rules.len());
}

#[test]
fn test_solder_mask_sliver_validation() {
    let cm = ConstraintManager::standard_default();

    // 120µm bridge passes 100µm standard min sliver
    assert!(
        cm.validate_solder_mask_sliver("PAD_U1_1_TO_2", "NET1", None, None, 120)
            .is_ok()
    );

    // 80µm bridge fails 100µm min sliver
    let err = cm
        .validate_solder_mask_sliver("PAD_U1_1_TO_2", "NET1", None, None, 80)
        .unwrap_err();
    assert_eq!(
        err.violation_type,
        RuleViolationType::SolderMaskSliverTooSmall
    );
    assert_eq!(err.required_value, "0.100mm");
    assert_eq!(err.actual_value, "0.080mm");
}

#[test]
fn test_silkscreen_clearance_validation() {
    let cm = ConstraintManager::standard_default();

    // 200µm distance to solder mask opening passes 150µm standard min
    assert!(
        cm.validate_silkscreen_clearance("SILK_TEXT_R1", "PAD_R1_1", "NET_R1", None, None, 200)
            .is_ok()
    );

    // 100µm distance fails
    let err = cm
        .validate_silkscreen_clearance("SILK_TEXT_R1", "PAD_R1_1", "NET_R1", None, None, 100)
        .unwrap_err();
    assert_eq!(
        err.violation_type,
        RuleViolationType::SilkscreenClearanceViolation
    );
    assert_eq!(err.required_value, "0.150mm");
    assert_eq!(err.actual_value, "0.100mm");
}

#[test]
fn test_net_antenna_and_return_path_validation() {
    let mut cm = ConstraintManager::standard_default();
    cm.add_rule(DesignRule::ReturnPath(
        oxide_rules::ReturnPathRule {
            net_class: "PCIE_GEN4".to_string(),
            max_plane_distance_microns: 150,
            forbid_split_crossing: true,
        },
    ));

    // Net antenna: 0 stub allowed by standard default, 50µm stub fails
    let antenna_err = cm
        .validate_antenna_length("NET_CLK", None, None, 50)
        .unwrap_err();
    assert_eq!(
        antenna_err.violation_type,
        RuleViolationType::NetAntennaExceeded
    );

    // Return path: crossing plane split fails
    let rp_err = cm
        .validate_return_path(
            "PCIE_TX0_P",
            "PCIE_GEN4",
            "GND_SPLIT",
            true,
            Some((12.5, 45.0)),
        )
        .unwrap_err();
    assert_eq!(
        rp_err.violation_type,
        RuleViolationType::ReturnPathSplitCrossing
    );
    assert_eq!(rp_err.location, Some((12.5, 45.0)));
}

#[test]
fn test_component_clearance_and_routing_layer_and_phase_skew_validation() {
    let mut cm = ConstraintManager::standard_default();

    // 1. Component clearance rule (500µm horizontal, 1000µm vertical)
    cm.add_rule(DesignRule::ComponentClearance(
        oxide_rules::ComponentClearanceRule {
            scope: RuleScope::Global,
            min_horizontal_clearance: 500,
            min_vertical_clearance: 1000,
            min_height: None,
            max_height: None,
        },
    ));

    // 2. Routing layer rule for RF class (only Inner1 allowed)
    cm.add_rule(DesignRule::RoutingLayer(
        oxide_rules::RoutingLayerRule {
            scope: RuleScope::NetClass("RF_50R".into()),
            permitted_layers: vec!["Inner1".to_string()],
            topology: "shortest".to_string(),
        },
    ));

    // 3. Diff pair phase skew rule (25µm intra-pair, 100µm inter-pair)
    cm.add_rule(DesignRule::DiffPairPhase(
        oxide_rules::DiffPairPhaseRule {
            net_class: "DDR4_DQ".to_string(),
            max_intra_pair_skew: 25,
            max_inter_pair_skew: 100,
        },
    ));

    // Horizontal component clearance: 300µm fails 500µm
    let comp_err = cm
        .validate_component_clearance("U1", "C1", None, 300, false)
        .unwrap_err();
    assert_eq!(
        comp_err.violation_type,
        RuleViolationType::ComponentClearanceViolation
    );

    // Routing layer: routing RF trace on TopLayer fails (only Inner1 permitted)
    let layer_err = cm
        .validate_routing_layer("RF_ANT", Some("RF_50R"), None, "TopLayer")
        .unwrap_err();
    assert_eq!(
        layer_err.violation_type,
        RuleViolationType::UnpermittedRoutingLayer
    );

    // Diff pair phase: 40µm intra-pair skew fails 25µm max tolerance
    let phase_err = cm
        .validate_diff_pair_phase("DDR4_DQ", "DDR4_DQS0", 40, true)
        .unwrap_err();
    assert_eq!(
        phase_err.violation_type,
        RuleViolationType::PhaseSkewViolation
    );
}


