use oxide_physics::{
    HdiError, ImpedanceCalculator, LayerStackup, MaterialProperties, ViaDefinition, ViaType,
    check_hdi_rules,
};

#[test]
fn test_microstrip_standard_fr4_50ohm() {
    // Standard surface microstrip on FR-4 (Er = 4.4):
    // w = 140 µm (~5.5 mil), h = 100 µm (~3.9 mil), t = 35 µm (1 oz)
    let z0 = ImpedanceCalculator::calculate_microstrip(140, 100, 35, 4.4);
    // IPC-2141 produces ~50.6 Ohms for these geometry values
    assert!(
        (z0 - 50.6).abs() < 1.0,
        "Expected microstrip Z0 near 50.6Ω, got {z0:.2}Ω"
    );
}

#[test]
fn test_microstrip_rogers_ro4350b() {
    // High frequency laminate: Er = 3.66, w = 160 µm, h = 100 µm, t = 35 µm
    let rogers = MaterialProperties::rogers_ro4350b();
    let z0 = ImpedanceCalculator::calculate_microstrip(160, 100, 35, rogers.dielectric_constant);
    assert!(
        (z0 - 50.3).abs() < 1.0,
        "Expected Rogers Z0 near 50.3Ω, got {z0:.2}Ω"
    );
}

#[test]
fn test_stripline_fr4_50ohm() {
    // Symmetrical stripline:
    // w = 120 µm, b = 400 µm total GND separation, t = 35 µm, Er = 4.4
    let z0 = ImpedanceCalculator::calculate_stripline(120, 400, 35, 4.4);
    assert!(
        (z0 - 50.0).abs() < 3.0,
        "Expected stripline Z0 near 50.0Ω, got {z0:.2}Ω"
    );
}

#[test]
fn test_differential_microstrip_90ohm_usb() {
    // 90 Ohm differential pair for USB 2.0 High Speed (s = 180 µm, w = 145 µm, h = 100 µm)
    let single_z0 = ImpedanceCalculator::calculate_microstrip(145, 100, 35, 4.4);
    let z_diff = ImpedanceCalculator::calculate_differential_microstrip(single_z0, 180, 100);

    assert!(
        (z_diff - 90.0).abs() < 2.0,
        "Expected differential Z_diff near 90.0Ω, got {z_diff:.2}Ω (single Z0: {single_z0:.2}Ω)"
    );
}

#[test]
fn test_solve_microstrip_width() {
    let target_z0 = 50.0;
    let h = 100;
    let t = 35;
    let er = 4.4;

    let solved_w = ImpedanceCalculator::solve_microstrip_width(target_z0, h, t, er)
        .expect("Should find matching width");
    let calculated_z0 = ImpedanceCalculator::calculate_microstrip(solved_w, h, t, er);

    assert!(
        (calculated_z0 - target_z0).abs() < 0.1,
        "Solver error: target 50Ω, width {solved_w}µm gave {calculated_z0:.2}Ω"
    );
}

#[test]
fn test_stackup_reference_plane_detection() {
    let stackup = LayerStackup::standard_4layer_1_6mm();
    // In 4-layer stackup:
    // idx 0: Top Solder Mask
    // idx 1: Top Layer (L1) -> Signal
    // idx 2: Prepreg (200µm)
    // idx 3: Inner GND (L2) -> InternalPlane
    // idx 4: Core (1065µm)
    // idx 5: Inner PWR (L3) -> InternalPlane
    // idx 6: Prepreg (200µm)
    // idx 7: Bottom Layer (L4) -> Signal
    // idx 8: Bottom Solder Mask

    assert_eq!(stackup.get_reference_plane(1), Some(3)); // L1 references L2 GND
    assert_eq!(stackup.get_reference_plane(7), Some(5)); // L4 references L3 PWR

    let h = stackup
        .get_dielectric_height_to_plane(1, 3)
        .expect("dielectric height");
    assert_eq!(h, 200); // 200µm prepreg
}

#[test]
fn test_hdi_via_validation() {
    let stackup = LayerStackup::standard_4layer_1_6mm();

    // 1. Valid through-hole via
    let th_via = ViaDefinition::standard_through_hole();
    assert!(check_hdi_rules(&th_via, &stackup).is_ok());

    // 2. Valid blind via from Top Layer (idx 1) to Inner GND (idx 3)
    let valid_blind = ViaDefinition {
        name: "Blind L1-L2".into(),
        via_type: ViaType::Blind {
            start_layer: 1,
            end_layer: 3,
        },
        drill_diameter: 150,
        pad_diameter: 350,
    };
    assert!(check_hdi_rules(&valid_blind, &stackup).is_ok());

    // 3. Invalid blind via between inner layers (L2 to L3) -> should be Buried instead
    let invalid_blind = ViaDefinition {
        name: "Invalid Blind L2-L3".into(),
        via_type: ViaType::Blind {
            start_layer: 3,
            end_layer: 5,
        },
        drill_diameter: 150,
        pad_diameter: 350,
    };
    assert!(matches!(
        check_hdi_rules(&invalid_blind, &stackup),
        Err(HdiError::BlindViaNotOnOuterSurface { .. })
    ));

    // 4. Valid buried via between Inner GND (idx 3) and Inner PWR (idx 5)
    let valid_buried = ViaDefinition {
        name: "Buried L2-L3".into(),
        via_type: ViaType::Buried {
            start_layer: 3,
            end_layer: 5,
        },
        drill_diameter: 200,
        pad_diameter: 450,
    };
    assert!(check_hdi_rules(&valid_buried, &stackup).is_ok());

    // 5. Invalid buried via touching Top Layer (idx 1)
    let invalid_buried = ViaDefinition {
        name: "Invalid Buried Touching L1".into(),
        via_type: ViaType::Buried {
            start_layer: 1,
            end_layer: 3,
        },
        drill_diameter: 200,
        pad_diameter: 450,
    };
    assert!(matches!(
        check_hdi_rules(&invalid_buried, &stackup),
        Err(HdiError::BuriedViaTouchesOuterSurface { .. })
    ));
}
