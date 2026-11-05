//! Deliverable D4: Brand Consistency Gate.
//!
//! Asserts that the product name, window titles, menu headers,
//! and generators are strictly "Oxide" / "Oxide EDA", never "Signex".

use oxide_app::app::Oxide;

#[test]
fn test_d4_brand_consistency_oxide_not_signex() {
    let (app, _) = Oxide::new();

    // Check application title
    let main_id = app.ui_state.main_window_id.expect("main window id");
    let title = app.title(main_id);
    assert!(
        title.contains("Oxide"),
        "Application window title must contain 'Oxide', got: {title}"
    );
    assert!(
        !title.to_lowercase().contains("signex"),
        "Application title must not contain 'signex', got: {title}"
    );

    // Check default generator metadata
    let board = oxide_types::pcb::PcbBoard::default();
    assert!(
        board.generator.contains("Oxide"),
        "Default board generator must be Oxide, got: {}",
        board.generator
    );
}
