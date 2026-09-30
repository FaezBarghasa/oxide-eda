use oxide_router::optimization::xsignals::{MatchedGroup, XSignal};

#[test]
fn test_xsignals_delay_and_matched_group_evaluation() {
    let mut sig1 = XSignal::new("DDR5_DQ0", vec![1, 2]);
    sig1.total_length = 45_000; // 45mm
    sig1.package_delay_ps = 5.0;

    let mut sig2 = XSignal::new("DDR5_DQ1", vec![3, 4]);
    sig2.total_length = 45_200; // 45.2mm
    sig2.package_delay_ps = 4.0;

    let mut group = MatchedGroup::new("BYTE_0", 500); // 500µm tolerance
    group.add_signal(sig1);
    group.add_signal(sig2);

    let status = group.evaluate_status(6.5); // 6.5 ps/mm in FR4
    assert!(status.within_tolerance);
    assert_eq!(status.delta, 200);
    assert!((status.skew_ps - 1.3).abs() < 0.01);
}
