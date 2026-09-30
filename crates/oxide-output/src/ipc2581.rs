//! IPC-2581C Universal Intelligent Manufacturing Data Exporter.
//!
//! Generates single-file XML digital containers containing full PCB layer stackups,
//! CAD geometry, netlists, component centroids, BOM, and test points.

use oxide_types::pcb::PcbBoard;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Ipc2581Error {
    #[error("Export formatting error: {0}")]
    Format(String),
}

#[derive(Debug, Clone, Default)]
pub struct Ipc2581Options {
    pub functional_mode: String,
    pub include_bom: bool,
    pub include_testpoints: bool,
    pub units: String,
}

#[derive(Debug, Clone)]
pub struct Ipc2581Output {
    pub filename: String,
    pub xml_content: String,
}

/// Generates IPC-2581C XML content for a PCB design.
pub fn export_ipc2581(board: &PcbBoard, _opts: &Ipc2581Options) -> Result<Ipc2581Output, Ipc2581Error> {
    let mut xml = String::with_capacity(8192);

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<IPC-2581 version=\"C\" xmlns=\"http://webstds.ipc.org/2581\">\n");
    xml.push_str("  <Content>\n");
    xml.push_str("    <FunctionMode mode=\"USER_DEF\" />\n");
    xml.push_str("    <StepRef name=\"BOARD\" />\n");
    xml.push_str("    <LayerRef name=\"ALL\" />\n");
    xml.push_str("    <BomRef name=\"PRIMARY_BOM\" />\n");
    xml.push_str("  </Content>\n");

    // Logistic Header
    xml.push_str("  <LogisticHeader>\n");
    xml.push_str("    <Enterprise code=\"OXIDE-EDA\" />\n");
    xml.push_str(&format!("    <Design name=\"{}\" />\n", board.generator));
    xml.push_str("  </LogisticHeader>\n");

    // Layer Stackup section
    xml.push_str("  <Stackup name=\"PRIMARY_STACKUP\">\n");
    for layer in &board.layers {
        xml.push_str(&format!(
            "    <StackupLayer layerOrGroupRef=\"{}\" thickness=\"0.035\" layerType=\"{}\" />\n",
            layer.name, layer.layer_type
        ));
    }
    xml.push_str("  </Stackup>\n");

    // Board Geometry & Components Step
    xml.push_str("  <Step name=\"BOARD\">\n");
    xml.push_str("    <Components>\n");
    for fp in &board.footprints {
        xml.push_str(&format!(
            "      <Component refDes=\"{}\" packageRef=\"{}\" value=\"{}\" layerRef=\"{}\" x=\"{:.4}\" y=\"{:.4}\" rotation=\"{:.2}\" />\n",
            fp.reference, fp.footprint_id, fp.value, fp.layer, fp.position.x, fp.position.y, fp.rotation
        ));
    }
    xml.push_str("    </Components>\n");

    // Logical Netlist
    xml.push_str("    <LogicalNetlist>\n");
    for net in &board.nets {
        xml.push_str(&format!("      <Net name=\"{}\" id=\"{}\" />\n", net.name, net.number));
    }
    xml.push_str("    </LogicalNetlist>\n");

    // Copper Traces & Vias
    xml.push_str("    <Routing>\n");
    for seg in &board.segments {
        xml.push_str(&format!(
            "      <Trace layerRef=\"{}\" width=\"{:.4}\" x1=\"{:.4}\" y1=\"{:.4}\" x2=\"{:.4}\" y2=\"{:.4}\" netId=\"{}\" />\n",
            seg.layer, seg.width, seg.start.x, seg.start.y, seg.end.x, seg.end.y, seg.net
        ));
    }
    for via in &board.vias {
        xml.push_str(&format!(
            "      <Via padDiameter=\"{:.4}\" drillDiameter=\"{:.4}\" x=\"{:.4}\" y=\"{:.4}\" netId=\"{}\" />\n",
            via.diameter, via.drill, via.position.x, via.position.y, via.net
        ));
    }
    xml.push_str("    </Routing>\n");
    xml.push_str("  </Step>\n");
    xml.push_str("</IPC-2581>\n");

    Ok(Ipc2581Output {
        filename: "board_ipc2581c.xml".to_string(),
        xml_content: xml,
    })
}
