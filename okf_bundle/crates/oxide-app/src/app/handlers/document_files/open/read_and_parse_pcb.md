---
okf_version: "0.2"
type: Function
title: read_and_parse_pcb
description: "Read + parse a `.snxpcb` off the UI thread — see"
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/read_and_parse_pcb
language: rust
---

# read_and_parse_pcb

Read + parse a `.snxpcb` off the UI thread — see

## Signature

```rust
fn read_and_parse_pcb(path: &std::path::Path) -> Result<oxide_types::pcb::PcbBoard, String>
```

## Docstring

Read + parse a `.snxpcb` off the UI thread — see
`read_and_parse_schematic`.

## Source
Lines 430–439 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| called_by | [open_pcb_file](/crates/oxide-app/src/app/handlers/document_files/open/open_pcb_file.md) |
