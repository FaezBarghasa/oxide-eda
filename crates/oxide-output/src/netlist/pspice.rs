//! PSpice simulation netlist (`.cir`) exporter.

use std::collections::HashMap;
use oxide_sim::PSpiceDeckBuilder;
use oxide_types::sim::SimulationConfig;
use thiserror::Error;

use crate::{ExportContext, Exporter};

pub struct PSpiceNetlistExporter;

#[derive(Debug, Clone, Default)]
pub struct PSpiceNetlistOptions {
    pub config: SimulationConfig,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PSpiceNetlistOutput {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum PSpiceNetlistError {
    #[error("no netlist was derived for this export — the app must attach ExportContext.netlist before exporting")]
    NoNetlist,
}

impl Exporter for PSpiceNetlistExporter {
    type Options = PSpiceNetlistOptions;
    type Output = PSpiceNetlistOutput;
    type Error = PSpiceNetlistError;

    fn export(
        &self,
        ctx: &ExportContext,
        opts: &Self::Options,
    ) -> Result<Self::Output, Self::Error> {
        let netlist = ctx.netlist.as_ref().ok_or(PSpiceNetlistError::NoNetlist)?;
        let sheets: Vec<_> = ctx.sheets.iter().map(|s| s.schematic.clone()).collect();
        let models = HashMap::new(); // In standalone export, empty model registry unless supplied
        let title = opts.title.clone().unwrap_or_else(|| {
            if !ctx.metadata.title.is_empty() {
                ctx.metadata.title.clone()
            } else {
                "Oxide EDA Circuit Simulation".to_string()
            }
        });

        let builder = PSpiceDeckBuilder::new(netlist, &sheets, &models, &opts.config).with_title(title);
        let deck = builder.build();

        Ok(PSpiceNetlistOutput {
            bytes: deck.into_bytes(),
        })
    }
}
