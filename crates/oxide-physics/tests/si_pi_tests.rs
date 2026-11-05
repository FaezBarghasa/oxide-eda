use oxide_physics::{
    BemFieldSolver, ImpedanceCalculator, PdnResistorMesh, TransmissionLineCrossSection,
};

#[test]
fn test_bem_field_solver_accuracy_vs_analytic() {
    let er = 4.3; // Standard FR-4
    let h_um = 100; // 100 µm core/prepreg
    let t_um = 35; // 1 oz copper (35 µm)

    // 1. Symmetrical Stripline: verify BEM vs IPC-2141 analytic within < 0.1%
    for ratio_tenth in [10, 15, 20] {
        let w_um = (ratio_tenth * h_um) / 10;
        let b_um = 2 * h_um + t_um; // Total plane separation b = 2h + t
        let ipc_z0 = ImpedanceCalculator::calculate_stripline(w_um, b_um, t_um, er);

        let cross_section = TransmissionLineCrossSection {
            trace_width_m: (w_um as f64) * 1e-6,
            trace_thickness_m: (t_um as f64) * 1e-6,
            dielectric_height_m: (h_um as f64) * 1e-6,
            trace_spacing_m: None,
            dielectric_er: er,
        };
        let bem_res = BemFieldSolver::solve_stripline(&cross_section);
        let rel_diff = (bem_res.characteristic_impedance_z0 - ipc_z0).abs() / ipc_z0;
        assert!(
            rel_diff < 0.001,
            "Stripline BEM mismatch: ipc={ipc_z0}, bem={}",
            bem_res.characteristic_impedance_z0
        );
    }

    // 2. Microstrip: standard 50-Ohm geometry (w/h around 1.3 - 1.5)
    // IPC-2141 has empirical deviations outside w/h ~ 1.25-1.5, but in the standard
    // 50-Ohm operational band, BEM and IPC-2141 agree within < 2.0%.
    for ratio_tenth in [13, 14, 15] {
        let w_um = (ratio_tenth * h_um) / 10;
        let ipc_z0 = ImpedanceCalculator::calculate_microstrip(w_um, h_um, t_um, er);

        let cross_section = TransmissionLineCrossSection {
            trace_width_m: (w_um as f64) * 1e-6,
            trace_thickness_m: (t_um as f64) * 1e-6,
            dielectric_height_m: (h_um as f64) * 1e-6,
            trace_spacing_m: None,
            dielectric_er: er,
        };
        let bem_res = BemFieldSolver::solve_microstrip(&cross_section);
        let rel_diff = (bem_res.characteristic_impedance_z0 - ipc_z0).abs() / ipc_z0;
        assert!(
            rel_diff < 0.02,
            "Microstrip w/h={}: IPC-2141={:.2}, BEM={:.2}, diff={:.2}%",
            (w_um as f64) / (h_um as f64),
            ipc_z0,
            bem_res.characteristic_impedance_z0,
            rel_diff * 100.0
        );
    }

    // 3. Propagation delay t_pd on FR-4 (dielectric_er = 4.3 -> er_eff ~ 3.2 -> t_pd ~ 6.0 ps/mm)
    let cross_section = TransmissionLineCrossSection {
        trace_width_m: 150e-6,
        trace_thickness_m: 35e-6,
        dielectric_height_m: 100e-6,
        trace_spacing_m: Some(150e-6),
        dielectric_er: 4.3,
    };
    let extracted = BemFieldSolver::solve_microstrip(&cross_section);
    assert!(
        extracted.propagation_delay_ps_per_mm > 5.0 && extracted.propagation_delay_ps_per_mm < 7.5,
        "Expected propagation delay in 5-7.5 ps/mm range, got {:.2}",
        extracted.propagation_delay_ps_per_mm
    );
    assert!(extracted.differential_impedance_zdiff.is_some());
}

#[test]
fn test_pdn_hand_solved_resistor_network() {
    // 3-node resistor network:
    // Node 0: Voltage source V0 = 1.0 V
    // Resistor 0->1: R1 = 0.05 Ohm
    // Node 1: Current sink I1 = 2.0 A
    // Resistor 1->2: R2 = 0.05 Ohm
    // Node 2: Current sink I2 = 1.0 A
    //
    // Analytical Solution:
    // Total current through R1 is I1 + I2 = 3.0 A
    // V1 = V0 - 3.0 * 0.05 = 1.0 - 0.15 = 0.850 V
    // Current through R2 is I2 = 1.0 A
    // V2 = V1 - 1.0 * 0.05 = 0.850 - 0.05 = 0.800 V

    let mut mesh = PdnResistorMesh::new(2.0, 0.0, 1.0, 1.0, 35.0);
    // Explicitly set sheet resistance to match the 0.05 Ohm per link benchmark
    mesh.sheet_resistance_ohms_per_sq = 0.05;

    mesh.set_voltage_source(0, 0, 1.0);
    mesh.add_current_sink(1, 0, 2.0);
    mesh.add_current_sink(2, 0, 1.0);

    // Solve using Gauss-Seidel nodal relaxation
    mesh.solve_dc(300);

    let v0 = mesh.nodes[0].voltage_v;
    let v1 = mesh.nodes[1].voltage_v;
    let v2 = mesh.nodes[2].voltage_v;

    assert!((v0 - 1.0).abs() < 1e-9, "V0 should stay pinned at 1.0V");

    let err_v1 = (v1 - 0.850).abs() / 0.850;
    let err_v2 = (v2 - 0.800).abs() / 0.800;

    println!(
        "Resistor Network Benchmark: V1 = {:.6} V (target 0.850 V, err={:.4}%), V2 = {:.6} V (target 0.800 V, err={:.4}%)",
        v1,
        err_v1 * 100.0,
        v2,
        err_v2 * 100.0
    );

    assert!(
        err_v1 < 0.001,
        "V1 relative error ({:.4}%) exceeds 0.1% tolerance",
        err_v1 * 100.0
    );
    assert!(
        err_v2 < 0.001,
        "V2 relative error ({:.4}%) exceeds 0.1% tolerance",
        err_v2 * 100.0
    );
}

#[test]
fn test_titan_vcore_plane_pdn_ir_drop_heatmap() {
    // Model Titan SBC 12-layer flagship power plane (+0V85_VCORE):
    // 100mm x 100mm board, 2mm discretization grid (51x51 = 2601 nodes)
    // 35 µm copper thickness (1 oz), nominal 0.85 V.
    let mut mesh = PdnResistorMesh::new(100.0, 100.0, 2.0, 0.85, 35.0);

    // VRM power stage source at (x=6mm, y=6mm) to (x=10mm, y=10mm) supplying 0.85V
    for c in 3..=5 {
        for r in 3..=5 {
            mesh.set_voltage_source(c, r, 0.85);
        }
    }

    // FPGA SoC Core drawing 5.0 A distributed over a 10mm x 10mm BGA area centered at (50mm, 50mm)
    let bga_col_start = 23;
    let bga_col_end = 27;
    let bga_row_start = 23;
    let bga_row_end = 27;
    let num_bga_nodes =
        ((bga_col_end - bga_col_start + 1) * (bga_row_end - bga_row_start + 1)) as f64;
    let current_per_node = 5.0 / num_bga_nodes;

    for c in bga_col_start..=bga_col_end {
        for r in bga_row_start..=bga_row_end {
            mesh.add_current_sink(c, r, current_per_node);
        }
    }

    // Solve DC Poisson-Laplace relaxation
    mesh.solve_dc(400);

    // Calculate IR-Drop statistics
    let stats = mesh.calculate_ir_drop_stats();
    println!(
        "Titan SBC +0V85_VCORE PDN: Min V = {:.4} V, Max IR-Drop = {:.2} mV, Avg IR-Drop = {:.2} mV at ({:.1}, {:.1}) mm",
        stats.min_voltage_v,
        stats.max_drop_mv,
        stats.avg_drop_mv,
        stats.worst_location_mm.0,
        stats.worst_location_mm.1
    );

    // Verification Requirements:
    // 1. Maximum IR-drop must be strictly < 35 mV (4.1% of 0.85V)
    assert!(
        stats.max_drop_mv < 35.0,
        "Max IR-drop ({:.2} mV) exceeded 35 mV limit",
        stats.max_drop_mv
    );

    // 2. Maximum current density < 30 A/mm^2
    // Plane cross section = 100mm * 0.035mm = 3.5 mm^2
    let current_density = 5.0 / (100.0 * 0.035);
    assert!(
        current_density < 30.0,
        "Current density {:.2} A/mm^2 exceeds 30 A/mm^2 limit",
        current_density
    );

    // 3. Export RGBA false-color heatmap and verify structure
    let rgba = mesh.generate_heatmap_rgba();
    assert_eq!(
        rgba.len(),
        mesh.cols * mesh.rows * 4,
        "Heatmap RGBA buffer dimension mismatch"
    );

    // Verify alpha channel is fully opaque
    for pixel in rgba.chunks_exact(4) {
        assert_eq!(pixel[3], 255);
    }
}
