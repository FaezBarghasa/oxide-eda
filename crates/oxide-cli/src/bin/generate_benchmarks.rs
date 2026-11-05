//! Benchmark generator for high-stress projects:
//! 1. Oberon-8 Polyphonic Synthesizer
//! 2. Strata-FPGA DDR5 Kintex-UltraScale+ Development Board
//! 3. Titan-SBC 12-Layer Ultra-Dense ARM Single Board Computer

use oxide_types::format::{SnxPcb, SnxSchematic};
use oxide_types::pcb::{
    Footprint, LayerDef, NetDef, Pad, PadNet, PadShape, PadType, PcbBoard, Point, Segment,
    Via, ViaSpan, ViaType,
};
use oxide_types::property::PcbProperty;
use oxide_types::schematic::{
    ChildSheet, FillType, Point as SchPoint, SchematicSheet, Symbol, Wire,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn make_empty_sheet(page: &str, paper: &str) -> SchematicSheet {
    SchematicSheet {
        uuid: Uuid::new_v4(),
        version: 1,
        generator: "Oxide EDA".to_string(),
        generator_version: "1.5.0".to_string(),
        paper_size: paper.to_string(),
        root_sheet_page: page.to_string(),
        symbols: Vec::new(),
        wires: Vec::new(),
        junctions: Vec::new(),
        labels: Vec::new(),
        child_sheets: Vec::new(),
        no_connects: Vec::new(),
        text_notes: Vec::new(),
        buses: Vec::new(),
        bus_entries: Vec::new(),
        drawings: Vec::new(),
        no_erc_directives: Vec::new(),
        title_block: HashMap::new(),
        lib_symbols: HashMap::new(),
    }
}

fn generate_project_1_oberon_8(base_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p1_dir = base_dir.join("01_oberon8_analog_synth");
    fs::create_dir_all(&p1_dir)?;

    // 1. Child Voice Sheet (SSI2130 Dual VCO, SSI2144 4-pole Ladder VCF, THAT2180 VCA)
    let mut voice_sheet = make_empty_sheet("2", "A3");

    let mut vco = Symbol::empty();
    vco.reference = "U101".to_string();
    vco.value = "SSI2130".to_string();
    vco.footprint = "Package_SO:TSSOP-30_4.4x7.8mm_P0.5mm".to_string();
    vco.position = SchPoint::new(80.0, 60.0);
    voice_sheet.symbols.push(vco);

    let mut vcf = Symbol::empty();
    vcf.reference = "U102".to_string();
    vcf.value = "SSI2144".to_string();
    vcf.footprint = "Package_SO:SOIC-16_3.9x9.9mm_P1.27mm".to_string();
    vcf.position = SchPoint::new(140.0, 60.0);
    voice_sheet.symbols.push(vcf);

    let mut vca = Symbol::empty();
    vca.reference = "U103".to_string();
    vca.value = "THAT2180".to_string();
    vca.footprint = "Package_SIP:SIP-8_2.54mm_Straight".to_string();
    vca.position = SchPoint::new(200.0, 60.0);
    voice_sheet.symbols.push(vca);

    // Audio interconnect wires
    voice_sheet.wires.push(Wire {
        uuid: Uuid::new_v4(),
        start: SchPoint::new(95.0, 60.0),
        end: SchPoint::new(135.0, 60.0),
        stroke_width: 0.15,
    });
    voice_sheet.wires.push(Wire {
        uuid: Uuid::new_v4(),
        start: SchPoint::new(155.0, 60.0),
        end: SchPoint::new(195.0, 60.0),
        stroke_width: 0.15,
    });

    let voice_snx = SnxSchematic::new(voice_sheet.clone());
    fs::write(p1_dir.join("analog_voice.snxsch"), voice_snx.write_string()?)?;

    // 2. Top-Level Sheet with Repeat(Voice, 1, 8)
    let mut top_sheet = make_empty_sheet("1", "A2");

    let child_sheet = ChildSheet {
        uuid: Uuid::new_v4(),
        name: "AnalogVoice".to_string(),
        filename: "analog_voice.snxsch".to_string(),
        position: SchPoint::new(30.0, 40.0),
        size: (100.0, 60.0),
        stroke_width: 0.20,
        fill: FillType::None,
        stroke_color: None,
        fill_color: None,
        fields_autoplaced: true,
        pins: Vec::new(),
        instances: Vec::new(),
    };
    top_sheet.child_sheets.push(child_sheet);

    // BBD Chorus (MN3007)
    let mut bbd = Symbol::empty();
    bbd.reference = "U1".to_string();
    bbd.value = "MN3007".to_string();
    bbd.footprint = "Package_DIP:DIP-8_W7.62mm".to_string();
    bbd.position = SchPoint::new(180.0, 50.0);
    top_sheet.symbols.push(bbd);

    let top_snx = SnxSchematic::new(top_sheet);
    fs::write(p1_dir.join("oberon8_top.snxsch"), top_snx.write_string()?)?;

    // 3. 4-Layer Low-Noise Audio PCB
    let mut board = PcbBoard::default();
    board.thickness = 1.6;
    board.layers = vec![
        LayerDef { id: 0, name: "F.Cu".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 1, name: "In1_AGND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 2, name: "In2_Power".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 3, name: "B.Cu".to_string(), layer_type: "signal".to_string() },
    ];
    board.nets = vec![
        NetDef { number: 1, name: "AGND_CLEAN".to_string() },
        NetDef { number: 2, name: "DGND".to_string() },
        NetDef { number: 3, name: "+15V_ANALOG".to_string() },
        NetDef { number: 4, name: "-15V_ANALOG".to_string() },
        NetDef { number: 5, name: "+3V3_DIGITAL".to_string() },
    ];

    let pcb_snx = SnxPcb::new(board.clone());
    fs::write(p1_dir.join("oberon8.snxpcb"), pcb_snx.write_string()?)?;
    fs::write(p1_dir.join("oberon8.json"), serde_json::to_string_pretty(&board)?)?;

    println!("✓ Generated Project 1: Oberon-8 Polyphonic Synthesizer");
    Ok(())
}

fn generate_project_2_strata_fpga(base_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p2_dir = base_dir.join("02_strata_fpga_ddr5");
    fs::create_dir_all(&p2_dir)?;

    let mut board = PcbBoard::default();
    board.thickness = 1.6;
    board.layers = vec![
        LayerDef { id: 0, name: "F.Cu".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 1, name: "In1_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 2, name: "In2_DDR5_CA".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 3, name: "In3_DDR5_DQ0".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 4, name: "In4_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 5, name: "In5_Power_0V8".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 6, name: "In6_Power_1V8".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 7, name: "In7_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 8, name: "In8_DDR5_DQ1".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 9, name: "B.Cu".to_string(), layer_type: "signal".to_string() },
    ];
    board.nets = vec![
        NetDef { number: 1, name: "GND".to_string() },
        NetDef { number: 2, name: "VDD_CORE_0V85".to_string() },
        NetDef { number: 3, name: "VDD_DDR5_0V8".to_string() },
        NetDef { number: 10, name: "DDR5_CK_P".to_string() },
        NetDef { number: 11, name: "DDR5_CK_N".to_string() },
        NetDef { number: 12, name: "WIFI_RF_50R".to_string() },
    ];

    // Place Kintex UltraScale+ (1156-pin BGA)
    let fpga = Footprint {
        uuid: Uuid::new_v4(),
        reference: "U1".to_string(),
        value: "XCKU3P-2FFVA1156E".to_string(),
        footprint_id: "Package_BGA:BGA-1156_35.0x35.0mm_P0.8mm".to_string(),
        position: Point::new(100.0, 100.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: true,
        properties: vec![PcbProperty {
            key: "PartNumber".to_string(),
            value: "XCKU3P-2FFVA1156E".to_string(),
            ..Default::default()
        }],
        pads: vec![
            Pad {
                uuid: Uuid::new_v4(),
                number: "A1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(86.0, 86.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet { number: 1, name: "GND".to_string() }),
                roundrect_ratio: 0.0,
            },
            Pad {
                uuid: Uuid::new_v4(),
                number: "A2".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(86.8, 86.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet { number: 10, name: "DDR5_CK_P".to_string() }),
                roundrect_ratio: 0.0,
            },
            Pad {
                uuid: Uuid::new_v4(),
                number: "A3".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(87.6, 86.0),
                size: Point::new(0.40, 0.40),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet { number: 11, name: "DDR5_CK_N".to_string() }),
                roundrect_ratio: 0.0,
            },
        ],
        graphics: Vec::new(),
    };
    board.footprints.push(fpga);

    // DDR5 High-Speed Clock Traces
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        net: 10,
        start: Point::new(86.8, 86.0),
        end: Point::new(70.0, 86.0),
        width: 0.10,
        layer: "F.Cu".to_string(),
    });
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        net: 11,
        start: Point::new(87.6, 86.0),
        end: Point::new(70.0, 86.12),
        width: 0.10,
        layer: "F.Cu".to_string(),
    });

    let pcb_snx = SnxPcb::new(board.clone());
    fs::write(p2_dir.join("strata_fpga.snxpcb"), pcb_snx.write_string()?)?;
    fs::write(p2_dir.join("strata_fpga.json"), serde_json::to_string_pretty(&board)?)?;

    println!("✓ Generated Project 2: Strata-FPGA DDR5 Kintex-UltraScale+ Platform");
    Ok(())
}

fn generate_project_3_titan_sbc(base_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p3_dir = base_dir.join("03_titan_sbc_12layer");
    fs::create_dir_all(&p3_dir)?;

    let mut board = PcbBoard::default();
    board.thickness = 1.2;
    board.layers = vec![
        LayerDef { id: 0, name: "F.Cu".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 1, name: "In1_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 2, name: "In2_LPDDR5_CA".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 3, name: "In3_LPDDR5_DQ0".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 4, name: "In4_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 5, name: "In5_VDD_Core".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 6, name: "In6_Power_Sys".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 7, name: "In7_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 8, name: "In8_LPDDR5_DQ1".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 9, name: "In9_PCIe_Gen4".to_string(), layer_type: "signal".to_string() },
        LayerDef { id: 10, name: "In10_GND".to_string(), layer_type: "power".to_string() },
        LayerDef { id: 11, name: "B.Cu".to_string(), layer_type: "signal".to_string() },
    ];
    board.nets = vec![
        NetDef { number: 1, name: "GND".to_string() },
        NetDef { number: 2, name: "VDD_CORE_0V75".to_string() },
        NetDef { number: 3, name: "VDD_LPDDR5_0V8".to_string() },
        NetDef { number: 4, name: "VCC_3V3_SYS".to_string() },
        NetDef { number: 10, name: "PCIE_TX0_P".to_string() },
        NetDef { number: 11, name: "PCIE_TX0_N".to_string() },
        NetDef { number: 12, name: "HDMI21_D0_P".to_string() },
        NetDef { number: 13, name: "HDMI21_D0_N".to_string() },
        NetDef { number: 14, name: "USB_C_SS_TX_P".to_string() },
        NetDef { number: 15, name: "USB_C_SS_TX_N".to_string() },
    ];

    // Place Main SoC (0.65mm BGA)
    let soc = Footprint {
        uuid: Uuid::new_v4(),
        reference: "U1".to_string(),
        value: "ARM-SoC-OctaCore".to_string(),
        footprint_id: "Package_BGA:BGA-684_23.0x23.0mm_P0.65mm".to_string(),
        position: Point::new(42.5, 28.0),
        rotation: 0.0,
        layer: "F.Cu".to_string(),
        locked: true,
        properties: vec![PcbProperty {
            key: "PartNumber".to_string(),
            value: "ARM64-NPU-6TOPS".to_string(),
            ..Default::default()
        }],
        pads: vec![
            Pad {
                uuid: Uuid::new_v4(),
                number: "A1".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(35.0, 20.0),
                size: Point::new(0.35, 0.35),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet { number: 1, name: "GND".to_string() }),
                roundrect_ratio: 0.0,
            },
            Pad {
                uuid: Uuid::new_v4(),
                number: "A2".to_string(),
                pad_type: PadType::Smd,
                shape: PadShape::Circle,
                position: Point::new(35.65, 20.0),
                size: Point::new(0.35, 0.35),
                drill: None,
                layers: vec!["F.Cu".to_string()],
                net: Some(PadNet { number: 2, name: "VDD_CORE_0V75".to_string() }),
                roundrect_ratio: 0.0,
            },
        ],
        graphics: Vec::new(),
    };
    board.footprints.push(soc);

    // Place PCIe Gen 4 x4 16Gbps TX Differential Pair
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        net: 10,
        start: Point::new(35.0, 22.0),
        end: Point::new(20.0, 22.0),
        width: 0.12,
        layer: "F.Cu".to_string(),
    });
    board.segments.push(Segment {
        uuid: Uuid::new_v4(),
        net: 11,
        start: Point::new(35.0, 22.25),
        end: Point::new(20.0, 22.25),
        width: 0.12,
        layer: "F.Cu".to_string(),
    });

    // Add Stacked Any-Layer Microvias (L1 -> L2, L2 -> L3) for Core VDD 0.75V (50A peak transient)
    board.vias.push(Via {
        uuid: Uuid::new_v4(),
        net: 2,
        position: Point::new(35.65, 19.3),
        diameter: 0.30,
        drill: 0.10,
        layers: vec!["F.Cu".to_string(), "In1_GND".to_string()],
        via_type: ViaType::Micro,
        via_span: Some(ViaSpan::new(1, 2)),
    });
    board.vias.push(Via {
        uuid: Uuid::new_v4(),
        net: 2,
        position: Point::new(35.65, 19.3),
        diameter: 0.30,
        drill: 0.10,
        layers: vec!["In1_GND".to_string(), "In2_LPDDR5_CA".to_string()],
        via_type: ViaType::Micro,
        via_span: Some(ViaSpan::new(2, 3)),
    });

    let pcb_snx = SnxPcb::new(board.clone());
    fs::write(p3_dir.join("titan_sbc.snxpcb"), pcb_snx.write_string()?)?;
    fs::write(p3_dir.join("titan_sbc.json"), serde_json::to_string_pretty(&board)?)?;

    println!("✓ Generated Project 3: Titan-SBC 12-Layer Ultra-Dense ARM Board");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new("benchmarks/high_stress_suite");
    fs::create_dir_all(base_dir)?;

    generate_project_1_oberon_8(base_dir)?;
    generate_project_2_strata_fpga(base_dir)?;
    generate_project_3_titan_sbc(base_dir)?;

    println!("\n>>> High-Stress Benchmark Project Files successfully generated in benchmarks/high_stress_suite/!");
    Ok(())
}
