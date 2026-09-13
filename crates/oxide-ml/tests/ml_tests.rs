use std::collections::HashMap;
use oxide_ml::{
    BoardTensorBuilder, MlConfig, MlEngine, PlacementAdvisor, Point2D, RoutingAction,
    RoutingAdvisorInputConfig, ViaPlanner,
};

#[test]
fn test_ml_routing_advisor_prediction() {
    let config = MlConfig::default();
    let engine = MlEngine::new(config);
    let mut advisor = engine.create_routing_advisor(4);

    let start = Point2D::new(0, 0);
    let target = Point2D::new(1_000_000, 0); // Directly East (+X)
    let obstacles = vec![];

    let prediction = advisor.predict(start, 0, target, 1, &obstacles);

    assert!(prediction.confidence > 0.5);
    // Highest scoring directional move should be MoveEast (index 2)
    let best_action_idx = prediction
        .action_scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    assert_eq!(best_action_idx, RoutingAction::MoveEast as usize);
}

#[test]
fn test_ml_via_planner() {
    let engine = MlEngine::new(MlConfig::default());
    let planner = engine.via_planner();

    let start = Point2D::new(100_000, 100_000);

    // Blocked ahead -> should place via to layer 1
    let pred = planner.should_place_via(start, 0, 0, true);
    assert!(pred.should_place);
    assert_eq!(pred.target_layer, 1);

    // Not blocked on same layer -> should not place via
    let pred_clean = planner.should_place_via(start, 0, 0, false);
    assert!(!pred_clean.should_place);
}

#[test]
fn test_ml_placement_advisor() {
    let engine = MlEngine::new(MlConfig::default());
    let advisor = engine.placement_advisor();

    let mut current_positions = HashMap::new();
    current_positions.insert(1, Point2D::new(0, 0));
    current_positions.insert(2, Point2D::new(2_000_000, 2_000_000));

    let nets = vec![(100, vec![1, 2, 3])];
    let suggestions = advisor.suggest_placement(&[3], &nets, &current_positions);

    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].component_id, 3);
    // Centroid of comp 1 (0,0) and comp 2 (2M, 2M) is (1M, 1M)
    assert_eq!(suggestions[0].suggested_position, Point2D::new(1_000_000, 1_000_000));
}

#[test]
fn test_board_tensor_builder_dimensions() {
    let config = RoutingAdvisorInputConfig {
        num_layers: 4,
        grid_height: 16,
        grid_width: 16,
        num_features: 12,
    };
    let builder = BoardTensorBuilder::new(config);
    let tensor = builder.build_window_tensor(
        Point2D::new(0, 0),
        0,
        Point2D::new(500_000, 500_000),
        1,
        &[],
    );

    assert_eq!(tensor.shape(), &[4, 16, 16, 12]);
}
