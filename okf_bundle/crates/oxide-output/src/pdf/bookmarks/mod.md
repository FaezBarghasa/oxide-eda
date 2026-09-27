---
okf_version: "0.2"
type: Module
title: bookmarks
description: PDF /Outlines (bookmark) emission for the schematic exporter.
resource: crates/oxide-output/src/pdf/bookmarks/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/bookmarks/mod
language: rust
---

# bookmarks

PDF /Outlines (bookmark) emission for the schematic exporter.

## Docstring

PDF /Outlines (bookmark) emission for the schematic exporter.

Builds a flat tree of bookmark items in two passes so that each
item gets a stable `Ref` before its parent/sibling links need to
be written. Pass 1 walks every sheet in the export and collects
`PendingBookmark`s gated by the `PdfOptions` toggles. Pass 2
assigns sequential `Ref`s and writes the outline dict + every
item dict.

Layout (Altium parity):
```text
Outline root
├── Sheet 1: Power
│   ├── Components
│   │   ├── R1
│   │   └── U1
│   └── Nets
│       ├── /VCC
│       ├── /GND
│       └── pin U1.3
└── Sheet 2: ...
```

With `global_bookmarks = true` the Components / Nets groups are
pulled out from under each sheet and aggregated into two
top-level groups instead. Per-sheet items still appear as their
own top-level entries so navigation by page is preserved.

## Relationships

| Type | Target |
|------|--------|
| related | [PendingBookmark](/crates/oxide-output/src/pdf/bookmarks/mod/PendingBookmark.md) |
| related | [new_root_node](/crates/oxide-output/src/pdf/bookmarks/mod/new_root_node.md) |
| related | [new_root_node](/crates/oxide-output/src/pdf/bookmarks/mod/new_root_node.md) |
| related | [build_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/build_bookmarks.md) |
| related | [emit_bookmarks](/crates/oxide-output/src/pdf/bookmarks/mod/emit_bookmarks.md) |
| related | [any_bookmark_toggle_on](/crates/oxide-output/src/pdf/bookmarks/mod/any_bookmark_toggle_on.md) |
| related | [nets_group_active](/crates/oxide-output/src/pdf/bookmarks/mod/nets_group_active.md) |
| related | [build_sheet_title](/crates/oxide-output/src/pdf/bookmarks/mod/build_sheet_title.md) |
| related | [format_component_title](/crates/oxide-output/src/pdf/bookmarks/mod/format_component_title.md) |
| related | [bookmark_zoom_factor](/crates/oxide-output/src/pdf/bookmarks/mod/bookmark_zoom_factor.md) |
