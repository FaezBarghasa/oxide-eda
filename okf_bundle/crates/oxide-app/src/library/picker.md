---
okf_version: "0.2"
type: Module
title: picker
description: Component picker modal.
resource: crates/oxide-app/src/library/picker.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/picker
language: rust
---

# picker

Component picker modal.

## Docstring

Component picker modal.

Opened from File ▸ Library ▸ Place Component… (and, eventually,
the `P` shortcut once Phase 2 wires the placement flow).

Shape:

```text
┌─[Place Component ─────────────────────────────────── X]─┐
│ [Search internal_pn / mpn / description…]              │
├────────────────────────────────────────────────────────┤
│ ► R0805_10k    1.2  Released   Yageo  RC0805FR-…      │
│   C0805_100n   1.0  Released   Murata GRM21BR…        │
│   …                                                    │
├────────────────────────────────────────────────────────┤
│                                  [ Cancel ] [ Place ]  │
└────────────────────────────────────────────────────────┘
```

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/picker/view.md) |
| related | [close_x](/crates/oxide-app/src/library/picker/close_x.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
