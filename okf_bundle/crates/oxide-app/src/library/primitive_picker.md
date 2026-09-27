---
okf_version: "0.2"
type: Module
title: primitive_picker
description: Primitive picker modal — Pick Symbol / Pick Footprint.
resource: crates/oxide-app/src/library/primitive_picker.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/primitive_picker
language: rust
---

# primitive_picker

Primitive picker modal — Pick Symbol / Pick Footprint.

## Docstring

Primitive picker modal — Pick Symbol / Pick Footprint.

Listing source: every mounted library's primitives surfaced via
`LibraryAdapter::list_symbols` (resp. `list_footprints`). Filtering
is case-insensitive across the primitive name.

Shape:

```text
┌─[ Pick Symbol ]────────────────────────────────────┐
│ Filter: [_______________]                          │
│                                                    │
│ ▼ Loratis-SN-lib-2.snxlib                          │
│     ESP32-WROOM-32.snxsym       (uuid abc12345…)  │
│     LM7805.snxsym               (uuid bd45ee01…)  │
│ ▼ Loratis-SN-lib-3.snxlib                          │
│     74HC595.snxsym                                 │
│                                                    │
│ [ Browse filesystem… ]    [ Cancel ]   [ Pick ]    │
└────────────────────────────────────────────────────┘
```

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/primitive_picker/view.md) |
| related | [close_x](/crates/oxide-app/src/library/primitive_picker/close_x.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
