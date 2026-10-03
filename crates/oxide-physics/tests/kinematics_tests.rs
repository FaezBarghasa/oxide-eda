use oxide_physics::kinematics::{BendLine, KinematicSubstrate, RigidFlexKinematicEngine};
use std::f64::consts::PI;

#[test]
fn test_rigid_flex_forward_kinematic_folding() {
    let mut engine = RigidFlexKinematicEngine::new();

    // Zone 1: Main rigid base [0, 0] to [100, 50]
    engine.register_zone(
        1,
        &[[0.0, 0.0], [100.0, 0.0], [100.0, 50.0], [0.0, 50.0]],
        1, // Rigid stackup
    );

    // Zone 2: Flex connector [100, 10] to [150, 40]
    engine.register_zone(
        2,
        &[[100.0, 10.0], [150.0, 10.0], [150.0, 40.0], [100.0, 40.0]],
        2, // Flex polyimide stackup
    );

    // Zone 3: Folded rigid flap [150, 0] to [200, 50]
    engine.register_zone(
        3,
        &[[150.0, 0.0], [200.0, 0.0], [200.0, 50.0], [150.0, 50.0]],
        1, // Rigid stackup
    );

    // 90 degree fold along line X = 100
    engine.register_bend(
        1,
        2,
        BendLine {
            axis_origin: [100.0, 0.0, 0.0],
            axis_direction: [0.0, 1.0, 0.0], // Y-axis hinge
            radius: 1.0,
            fold_angle_radians: PI / 2.0, // 90 deg
        },
    );

    // Another 90 degree fold along line X = 150 (making Zone 3 parallel to base)
    engine.register_bend(
        2,
        3,
        BendLine {
            axis_origin: [150.0, 0.0, 0.0],
            axis_direction: [0.0, 1.0, 0.0],
            radius: 1.0,
            fold_angle_radians: PI / 2.0,
        },
    );

    // Transform of base zone 1 should be identity
    let t1 = engine.compute_transform(1);
    let p1 = t1.transform_point([50.0, 25.0, 0.0]);
    assert!((p1[0] - 50.0).abs() < 1e-4);
    assert!((p1[1] - 25.0).abs() < 1e-4);
    assert!(p1[2].abs() < 1e-4);

    // Transform of zone 2 (folded 90 deg up)
    let t2 = engine.compute_transform(2);
    let p2 = t2.transform_point([120.0, 25.0, 0.0]);
    // Origin at 100. Point is (100 + 20, 25, 0). Folded 90 deg around Y: X becomes 100, Z becomes -20 or +20
    assert!((p2[0] - 100.0).abs() < 1e-4);
    assert!((p2[1] - 25.0).abs() < 1e-4);
    assert!((p2[2].abs() - 20.0).abs() < 1e-4);
}
