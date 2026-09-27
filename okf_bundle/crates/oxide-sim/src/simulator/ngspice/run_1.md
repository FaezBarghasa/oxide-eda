---
okf_version: "0.2"
type: Function
title: run
resource: crates/oxide-sim/src/simulator/ngspice.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:14:51Z"
concept_id: crates/oxide-sim/src/simulator/ngspice/run_1
language: rust
---

# run

## Signature

```rust
fn run(
        &self,
        deck: &str,
        work_dir: &Path,
        progress_tx: Option<tokio::sync::mpsc::Sender<SimProgress>>,
    ) -> Pin<Box<dyn Future<Output = Result<WaveformDataset, SimError>> + Send + '_>>
```

## Source
Lines 50–154 in `crates/oxide-sim/src/simulator/ngspice.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ngspice](/crates/oxide-sim/src/simulator/ngspice.md) |
| calls | [parse_spice_raw](/crates/oxide-sim/src/parser/raw/parse_spice_raw.md) |
