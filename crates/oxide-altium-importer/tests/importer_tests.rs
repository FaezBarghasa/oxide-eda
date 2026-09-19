use byteorder::{LittleEndian, WriteBytesExt};
use oxide_altium_importer::record::{parse_record_stream, AltiumRecord};
use oxide_altium_importer::sch_importer::parse_schdoc_records;
use oxide_altium_importer::schlib_importer::parse_symbols_from_records;
use oxide_altium_importer::pcblib_importer::parse_footprints_from_records;

#[test]
fn test_altium_record_pipe_string_parsing() {
    let raw = "|RECORD=1|LIBREFERENCE=STM32F401CEU6|DESIGNATOR=U1|LOCATION.X=1000|LOCATION.Y=2000|ROTATION=90|MIRRORED=TRUE|";
    let rec = AltiumRecord::from_pipe_str(raw);

    assert_eq!(rec.get_i64("RECORD"), Some(1));
    assert_eq!(rec.get("LIBREFERENCE"), Some("STM32F401CEU6"));
    assert_eq!(rec.get("DESIGNATOR"), Some("U1"));
    assert_eq!(rec.get_coord_mm("LOCATION.X"), Some(25.4));
    assert_eq!(rec.get_coord_mm("LOCATION.Y"), Some(50.8));
    assert_eq!(rec.get_f64("ROTATION"), Some(90.0));
    assert!(rec.get_bool("MIRRORED"));
}

#[test]
fn test_length_prefixed_record_stream_parsing() {
    let r1 = "|RECORD=31|TITLE=Main Board|REVISION=A.1|";
    let r2 = "|RECORD=4|TEXT=SPI1_SCK|LOCATION.X=500|LOCATION.Y=600|";

    let mut stream_bytes = Vec::new();
    stream_bytes.write_u32::<LittleEndian>(r1.len() as u32).unwrap();
    stream_bytes.extend_from_slice(r1.as_bytes());

    stream_bytes.write_u32::<LittleEndian>(r2.len() as u32).unwrap();
    stream_bytes.extend_from_slice(r2.as_bytes());

    let records = parse_record_stream(&stream_bytes).expect("Should parse stream");
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].get("TITLE"), Some("Main Board"));
    assert_eq!(records[1].get("TEXT"), Some("SPI1_SCK"));
}

#[test]
fn test_schdoc_record_conversion_to_schematic_sheet() {
    let records = vec![
        AltiumRecord::from_pipe_str("|RECORD=31|TITLE=Sensor Node|REVISION=1.0|DOCUMENTNUMBER=DOC-001|"),
        AltiumRecord::from_pipe_str("|RECORD=1|LIBREFERENCE=RES_0805|DESIGNATOR=R1|LOCATION.X=1000|LOCATION.Y=1500|ROTATION=0|"),
        AltiumRecord::from_pipe_str("|RECORD=34|OWNERINDEX=1|NAME=Comment|TEXT=10k|"),
        AltiumRecord::from_pipe_str("|RECORD=34|OWNERINDEX=1|NAME=Footprint|TEXT=RESC2012X60N|"),
        AltiumRecord::from_pipe_str("|RECORD=27|LOCATIONCOUNT=2|X1=1000|Y1=1500|X2=2000|Y2=1500|"),
        AltiumRecord::from_pipe_str("|RECORD=29|LOCATION.X=2000|LOCATION.Y=1500|"),
        AltiumRecord::from_pipe_str("|RECORD=4|TEXT=VCC_3V3|LOCATION.X=2000|LOCATION.Y=1500|"),
        AltiumRecord::from_pipe_str("|RECORD=17|TEXT=GND|LOCATION.X=1000|LOCATION.Y=1000|"),
        AltiumRecord::from_pipe_str("|RECORD=26|LOCATIONCOUNT=2|X1=500|Y1=500|X2=1500|Y2=500|"),
        AltiumRecord::from_pipe_str("|RECORD=25|LOCATION.X=500|LOCATION.Y=500|CORNER.X=600|CORNER.Y=600|"),
        AltiumRecord::from_pipe_str("|RECORD=14|SHEETNAME=PowerSupply|FILENAME=power.SchDoc|LOCATION.X=3000|LOCATION.Y=3000|XSIZE=4000|YSIZE=2000|"),
    ];

    let sheet = parse_schdoc_records(&records).expect("Failed to convert schematic records");

    // Title Block
    assert_eq!(sheet.title_block.get("title").map(|s| s.as_str()), Some("Sensor Node"));
    assert_eq!(sheet.title_block.get("rev").map(|s| s.as_str()), Some("1.0"));

    // Symbol instance
    assert_eq!(sheet.symbols.len(), 1);
    let sym = &sheet.symbols[0];
    assert_eq!(sym.lib_id, "RES_0805");
    assert_eq!(sym.reference, "R1");
    assert_eq!(sym.value, "10k");
    assert_eq!(sym.footprint, "RESC2012X60N");
    assert_eq!(sym.position.x, 25.4);
    assert_eq!(sym.position.y, 38.1);

    // Wires & Junctions
    assert_eq!(sheet.wires.len(), 1);
    assert_eq!(sheet.wires[0].start.x, 25.4);
    assert_eq!(sheet.wires[0].end.x, 50.8);
    assert_eq!(sheet.junctions.len(), 1);

    // Labels & Power Ports
    assert_eq!(sheet.labels.len(), 2);
    let net_label = &sheet.labels[0];
    assert_eq!(net_label.text, "VCC_3V3");
    assert_eq!(net_label.label_type, oxide_types::schematic::LabelType::Net);
    let pwr_port = &sheet.labels[1];
    assert_eq!(pwr_port.text, "GND");
    assert_eq!(pwr_port.label_type, oxide_types::schematic::LabelType::Power);

    // Bus & Bus Entry
    assert_eq!(sheet.buses.len(), 1);
    assert_eq!(sheet.bus_entries.len(), 1);

    // Child Sheet
    assert_eq!(sheet.child_sheets.len(), 1);
    assert_eq!(sheet.child_sheets[0].name, "PowerSupply");
    assert_eq!(sheet.child_sheets[0].filename, "power.SchDoc");
}

#[test]
fn test_schlib_record_conversion_to_lib_symbols() {
    let records = vec![
        AltiumRecord::from_pipe_str("|RECORD=1|LIBREFERENCE=OPAMP_DUAL|DESCRIPTION=Dual Low-Noise OpAmp|DESIGNATOR=U?|COMMENT=NE5532|"),
        AltiumRecord::from_pipe_str("|RECORD=2|DESIGNATOR=1|NAME=OUTA|ELECTRICAL=2|LOCATION.X=1000|LOCATION.Y=0|PINCONGLOMERATE=0|PINLENGTH=100|"),
        AltiumRecord::from_pipe_str("|RECORD=2|DESIGNATOR=2|NAME=INA-|ELECTRICAL=0|LOCATION.X=0|LOCATION.Y=500|PINCONGLOMERATE=2|PINLENGTH=100|"),
        AltiumRecord::from_pipe_str("|RECORD=2|DESIGNATOR=3|NAME=INA+|ELECTRICAL=0|LOCATION.X=0|LOCATION.Y=0|PINCONGLOMERATE=2|PINLENGTH=100|"),
        AltiumRecord::from_pipe_str("|RECORD=6|LOCATION.X=0|LOCATION.Y=0|CORNER.X=1000|CORNER.Y=1000|"),
    ];

    let symbols = parse_symbols_from_records(&records).expect("Failed to convert SchLib records");
    assert_eq!(symbols.len(), 1);

    let opamp = &symbols[0];
    assert_eq!(opamp.name, "OPAMP_DUAL");
    assert_eq!(opamp.description, "Dual Low-Noise OpAmp");
    assert_eq!(opamp.designator, "U?");
    assert_eq!(opamp.comment, "NE5532");
    assert_eq!(opamp.pins.len(), 3);

    // Pin 1 (Output)
    assert_eq!(opamp.pins[0].number, "1");
    assert_eq!(opamp.pins[0].name, "OUTA");
    assert_eq!(opamp.pins[0].electrical, oxide_library::primitive::symbol::PinDirection::Output);

    // Pin 2 (Input)
    assert_eq!(opamp.pins[1].number, "2");
    assert_eq!(opamp.pins[1].name, "INA-");
    assert_eq!(opamp.pins[1].electrical, oxide_library::primitive::symbol::PinDirection::Input);

    // Graphics
    assert_eq!(opamp.graphics.len(), 1);
}

#[test]
fn test_pcblib_record_conversion_to_footprints() {
    let records = vec![
        AltiumRecord::from_pipe_str("|RECORD=Component|PATTERN=SOIC-8|DESCRIPTION=Small Outline IC 8-Lead|"),
        AltiumRecord::from_pipe_str("|RECORD=Pad|NAME=1|X=0|Y=0|TOPXSIZE=100|TOPYSIZE=40|SHAPE=RoundedRectangle|HOLESIZE=0|"),
        AltiumRecord::from_pipe_str("|RECORD=Pad|NAME=2|X=0|Y=50|TOPXSIZE=100|TOPYSIZE=40|SHAPE=RoundedRectangle|HOLESIZE=0|"),
        AltiumRecord::from_pipe_str("|RECORD=Track|X1=0|Y1=0|X2=200|Y2=0|WIDTH=10|"),
    ];

    let footprints = parse_footprints_from_records(&records).expect("Failed to convert PcbLib records");
    assert_eq!(footprints.len(), 1);

    let fp = &footprints[0];
    assert_eq!(fp.name, "SOIC-8");
    assert_eq!(fp.description, "Small Outline IC 8-Lead");
    assert_eq!(fp.pads.len(), 2);
    assert_eq!(fp.pads[0].number, "1");
    assert_eq!(fp.pads[0].kind, oxide_library::primitive::footprint::PadKind::Smd);
    assert_eq!(fp.silk_f.len(), 1);
}
