---
okf_version: "0.2"
type: Module
title: build_stock_library
description: Build the 5 reference parametric footprints that ship in
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library
language: rust
---

# build_stock_library

Build the 5 reference parametric footprints that ship in

## Docstring

Build the 5 reference parametric footprints that ship in
`assets/stock-library/footprints/`.

Run via:

```text
cargo run --example build_stock_library -p oxide-app
```

Each footprint authors a single `BoardTopPlane`, parameterises the
interesting dimensions through `SketchData::parameters`, and lays
down one Point per pad with the per-pad delta carried on
`offset_x_expr` / `offset_y_expr`. The bake walker (oxide-app's
`apply_sketch_edit`) is the consumer; here we only emit the
authored sketch — the bake fires the first time the user opens the
file in the footprint editor.

Apache-clean: industry-standard footprint dimensions (JEDEC
SOIC-8 / QFN-16 / IPC-7351 R0805) only. No third-party EDA-tool
source / file-format docs / wikis consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [main](/crates/oxide-app/examples/build_stock_library/main.md) |
| related | [count_sketch_pads](/crates/oxide-app/examples/build_stock_library/count_sketch_pads.md) |
| related | [smd_pad](/crates/oxide-app/examples/build_stock_library/smd_pad.md) |
| related | [build_soic8](/crates/oxide-app/examples/build_stock_library/build_soic8.md) |
| related | [build_qfn16](/crates/oxide-app/examples/build_stock_library/build_qfn16.md) |
| related | [build_r0805](/crates/oxide-app/examples/build_stock_library/build_r0805.md) |
| related | [build_mounting_hole_3p2](/crates/oxide-app/examples/build_stock_library/build_mounting_hole_3p2.md) |
| related | [build_fiducial_1mm](/crates/oxide-app/examples/build_stock_library/build_fiducial_1mm.md) |
