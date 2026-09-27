---
okf_version: "0.2"
type: Function
title: run
resource: crates/oxide-sim/src/simulator/pspice_cli.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:14:10Z"
concept_id: crates/oxide-sim/src/simulator/pspice_cli/run
language: rust
---

# run

## Signature

```rust
impl PSpiceCliSimulator { fn run(
        &self,
        deck: &str,
        work_dir: &Path,
        progress_tx: Option<tokio::sync::mpsc::Sender<SimProgress>>,
    ) -> Pin<Box<dyn Future<Output = Result<WaveformDataset, SimError>> + Send + '_>> }
```

## Source
Lines 40–114 in `crates/oxide-sim/src/simulator/pspice_cli.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pspice_cli](/crates/oxide-sim/src/simulator/pspice_cli.md) |
| calls | [parse_csdf](/crates/oxide-sim/src/parser/csdf/parse_csdf.md) |
