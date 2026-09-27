---
okf_version: "0.2"
type: Function
title: port_label_priority
description: "---------------------------------------------------------------------------"
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/port_label_priority
language: rust
---

# port_label_priority

---------------------------------------------------------------------------

## Signature

```rust
fn port_label_priority(label_type: LabelType) -> Option<u8>
```

## Docstring

---------------------------------------------------------------------------
Sheet-pin helpers (used by sheet.rs and transform.rs)
---------------------------------------------------------------------------

## Source
Lines 45–51 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| called_by | [collect_exposed_sheet_ports](/crates/oxide-engine/src/sheet/collect_exposed_sheet_ports.md) |
