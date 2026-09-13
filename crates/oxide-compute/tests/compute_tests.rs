use oxide_compute::{
    create_backend, BackendPreference, CpuBackend, GpuBBox, GpuDrcChecker,
    ThermalSimulator, CongestionMapGenerator, GpuTrack,
};

#[test]
fn test_cpu_drc_detection() {
    let backend = Box::new(CpuBackend::new());
    let mut checker = GpuDrcChecker::new(backend);

    let objects = vec![
        GpuBBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 100.0,
            max_y: 100.0,
            layer: 1,
            net_id: 1,
            object_type: 0,
            object_id: 10,
        },
        GpuBBox {
            min_x: 110.0,
            min_y: 0.0,
            max_x: 200.0,
            max_y: 100.0,
            layer: 1,
            net_id: 2, // Different net
            object_type: 0,
            object_id: 20,
        },
        GpuBBox {
            min_x: 300.0,
            min_y: 0.0,
            max_x: 400.0,
            max_y: 100.0,
            layer: 1,
            net_id: 3,
            object_type: 0,
            object_id: 30,
        },
    ];

    // Distance between obj 10 and 20 is 10.0 (110 - 100). Clearance rule is 20.0 -> violation!
    // Distance between obj 20 and 30 is 100.0 (300 - 200). Clearance rule is 20.0 -> OK.
    let violations = checker.check_clearance(&objects, 20.0).expect("DRC check failed");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].obj_a, 10);
    assert_eq!(violations[0].obj_b, 20);
    assert!((violations[0].distance - 10.0).abs() < 1e-3);
}

#[test]
fn test_cpu_thermal_simulation() {
    let backend = Box::new(CpuBackend::new());
    let mut sim = ThermalSimulator::new(backend, 32, 32);

    let mut power_map = vec![0.0f32; 32 * 32];
    let material_map = vec![0u32; 32 * 32]; // all FR4

    // Place a heat source at center (16, 16)
    power_map[16 * 32 + 16] = 100.0;

    let res = sim.simulate(&power_map, &material_map, 50).expect("Thermal sim failed");
    assert!(res.max_temp > 25.0);
    assert_eq!(res.grid_width, 32);
    assert_eq!(res.grid_height, 32);
}

#[test]
fn test_cpu_congestion_map() {
    let backend = Box::new(CpuBackend::new());
    let mut generator = CongestionMapGenerator::new(backend, 20, 20, 10.0);

    let tracks = vec![
        GpuTrack {
            x0: 10.0,
            y0: 10.0,
            x1: 50.0,
            y1: 10.0,
            layer: 0,
            width: 1.0,
            net_id: 1,
            _pad: 0,
        },
        GpuTrack {
            x0: 20.0,
            y0: 0.0,
            x1: 20.0,
            y1: 50.0,
            layer: 0,
            width: 1.0,
            net_id: 2,
            _pad: 0,
        },
    ];

    let result = generator.compute_congestion(&tracks).expect("Congestion mapping failed");
    assert!(result.max_density >= 2); // intersection at (2, 1) has both tracks
}

#[test]
fn test_auto_backend_factory() {
    let backend = create_backend(BackendPreference::Auto);
    assert!(backend.supports_compute());
}
