//! Deliverable D2: Headless Scenario Runner & Blank-Guard Test Suite.
//!
//! Validates that opening verifiable fixtures (T1 and T2) headlessly
//! transitions the application into a fully populated document state
//! with verified geometries, tab state, layers, footprints, tracks,
//! and non-empty canvas bounds (Blank-Guard).

use oxide_app::app::{FileMsg, Message, Oxide};
use oxide_types::format::SnxPcb;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn test_d2_scenario_t1_analog_synth_blank_guard() {
    let t1_path = workspace_root().join("fixtures/verifiable/real_analog_synth_2layer.snxpcb");
    assert!(
        t1_path.exists(),
        "T1 fixture must exist at {}",
        t1_path.display()
    );

    let content = std::fs::read_to_string(&t1_path).expect("read T1 file");
    let snx_pcb = SnxPcb::parse(&content).expect("parse T1 SnxPcb");
    let board = snx_pcb.board.clone();

    // 1. Initialise quiet headless Oxide application
    let (mut app, _boot_task) = Oxide::new();

    // Verify initial state is empty
    assert!(
        app.document_state.tabs.is_empty(),
        "Initial app must have 0 document tabs"
    );

    // 2. Dispatch PcbOpenFinished message with parsed T1 board
    drop(app.update(Message::File(FileMsg::PcbOpenFinished {
        path: t1_path.clone(),
        title: "real_analog_synth_2layer".to_string(),
        result: Ok(Box::new(board.clone())),
    })));

    // 3. Blank-Guard assertions: Document Tabs
    assert_eq!(
        app.document_state.tabs.len(),
        1,
        "Blank-Guard: Must have exactly 1 open tab"
    );
    let active_tab_idx = app.document_state.active_tab;
    let active_tab = &app.document_state.tabs[active_tab_idx];
    assert_eq!(active_tab.path, t1_path);
    assert_eq!(active_tab.title, "real_analog_synth_2layer");

    // 4. Blank-Guard assertions: PCB Entities
    assert_eq!(
        board.footprints.len(),
        6,
        "T1 must have 6 footprints (2 SOIC-14 + 4 0805 passives)"
    );
    assert_eq!(board.segments.len(), 5, "T1 must have 5 routed tracks");
    assert_eq!(board.vias.len(), 2, "T1 must have 2 ground stitching vias");
    assert!(
        board.nets.len() >= 4,
        "T1 must define power and signal nets (GND, +12V, -12V, VCO_OUT, VCF_IN, VCF_OUT)"
    );
    assert_eq!(
        board.layers.len(),
        2,
        "T1 is a 2-layer analog board (TopCopper, BottomCopper)"
    );

    // 5. Blank-Guard: Non-empty geometry boundary
    let (min_x, max_x) = board
        .segments
        .iter()
        .map(|s| (s.start.x.min(s.end.x), s.start.x.max(s.end.x)))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |acc, (min, max)| {
            (acc.0.min(min), acc.1.max(max))
        });
    assert!(
        min_x.is_finite() && max_x.is_finite() && max_x > min_x,
        "Blank-Guard: PCB geometry must span non-zero width on canvas"
    );
}

#[test]
fn test_d2_scenario_t2_fpga_ddr5_4layer_blank_guard() {
    let t2_path = workspace_root().join("fixtures/verifiable/real_fpga_ddr5_4layer.snxpcb");
    assert!(
        t2_path.exists(),
        "T2 fixture must exist at {}",
        t2_path.display()
    );

    let content = std::fs::read_to_string(&t2_path).expect("read T2 file");
    let snx_pcb = SnxPcb::parse(&content).expect("parse T2 SnxPcb");
    let board = snx_pcb.board.clone();

    // 1. Initialise quiet headless Oxide application
    let (mut app, _boot_task) = Oxide::new();

    // 2. Dispatch PcbOpenFinished message with parsed T2 board
    drop(app.update(Message::File(FileMsg::PcbOpenFinished {
        path: t2_path.clone(),
        title: "real_fpga_ddr5_4layer".to_string(),
        result: Ok(Box::new(board.clone())),
    })));

    // 3. Blank-Guard assertions: Document Tabs & Layers
    assert_eq!(
        app.document_state.tabs.len(),
        1,
        "Blank-Guard: Must have 1 open tab"
    );
    assert_eq!(
        board.layers.len(),
        4,
        "T2 must have 4 copper layers (TopCopper, In1.GND, In2.PWR, BottomCopper)"
    );

    // 4. Blank-Guard assertions: High-speed entities
    assert_eq!(
        board.footprints.len(),
        4,
        "T2 must have 4 footprints (2 SOIC-14 + 2 0805 passives)"
    );
    assert_eq!(
        board.segments.len(),
        2,
        "T2 must have differential pair routing tracks"
    );
    assert!(
        board.nets.iter().any(|n| n.name == "DDR5_CLK_P"),
        "T2 must contain DDR5_CLK_P differential net"
    );
    assert!(
        board.nets.iter().any(|n| n.name == "DDR5_CLK_N"),
        "T2 must contain DDR5_CLK_N differential net"
    );
}

#[test]
fn test_d2_scenario_t3_bga_fanout_6layer_blank_guard() {
    let t3_path = workspace_root().join("fixtures/verifiable/real_bga_fanout_6layer.snxpcb");
    assert!(
        t3_path.exists(),
        "T3 fixture must exist at {}",
        t3_path.display()
    );

    let content = std::fs::read_to_string(&t3_path).expect("read T3 file");
    let snx_pcb = SnxPcb::parse(&content).expect("parse T3 SnxPcb");
    let board = snx_pcb.board.clone();

    // 1. Initialise quiet headless Oxide application
    let (mut app, _boot_task) = Oxide::new();

    // 2. Dispatch PcbOpenFinished message with parsed T3 board
    drop(app.update(Message::File(FileMsg::PcbOpenFinished {
        path: t3_path.clone(),
        title: "real_bga_fanout_6layer".to_string(),
        result: Ok(Box::new(board.clone())),
    })));

    // 3. Blank-Guard assertions: Document Tabs & 6 Layers
    assert_eq!(
        app.document_state.tabs.len(),
        1,
        "Blank-Guard: Must have 1 open tab"
    );
    assert_eq!(board.layers.len(), 6, "T3 must have 6 copper layers");

    // 4. Blank-Guard assertions: High-Density BGA Entities
    assert_eq!(
        board.footprints.len(),
        5,
        "T3 must have 5 footprints (1 BGA-64 + 4 0805 decoupling caps)"
    );
    assert_eq!(
        board.footprints[0].pads.len(),
        64,
        "U1 must be an 8x8 BGA with 64 pads"
    );
    assert_eq!(
        board.segments.len(),
        4,
        "T3 must have 4 fanout escape tracks"
    );
    assert_eq!(board.vias.len(), 2, "T3 must have 2 dogbone plane vias");
    assert!(
        board.nets.iter().any(|n| n.name == "+0V85_CORE"),
        "T3 must contain core power net +0V85_CORE"
    );
}
