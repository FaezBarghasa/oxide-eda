---
okf_version: "0.2"
type: Function
title: build_fiducial_1mm
description: Single fiducial vision-alignment marker at the origin.
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library/build_fiducial_1mm
language: rust
---

# build_fiducial_1mm

Single fiducial vision-alignment marker at the origin.

## Signature

```rust
fn build_fiducial_1mm() -> Footprint
```

## Docstring

Single fiducial vision-alignment marker at the origin.
1 mm round copper, 2 mm mask opening (ring of bare substrate
around the copper dot — required for camera contrast). No paste,
no drill.

## Source
Lines 425–465 in `crates/oxide-app/examples/build_stock_library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build_stock_library](/crates/oxide-app/examples/build_stock_library.md) |
