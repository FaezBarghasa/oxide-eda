use oxide_physics::backdrill::{BackdrillViaInput, ViaCoord, calculate_backdrilling};
use oxide_physics::stackup::LayerStackup;

#[test]
fn test_backdrill_calculation() {
    let vias = vec![BackdrillViaInput {
        position: ViaCoord { x: 20.0, y: 30.0 },
        net_id: 42,
        diameter_um: 600,
        drill_um: 300,
    }];

    let stackup = LayerStackup::standard_4layer_1_6mm();
    let summary = calculate_backdrilling(&vias, &stackup, &[42], 150, 200);

    assert_eq!(summary.total_stubs_identified, 1);
    assert_eq!(summary.total_backdrilled, 1);
    assert_eq!(summary.targets[0].net_id, 42);
    assert_eq!(summary.targets[0].backdrill_diameter_um, 800); // 600um + 200um oversize
}
