---
okf_version: "0.2"
type: Module
title: browser
description: Library Browser tab — the main-window surface for working with
resource: crates/oxide-app/src/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/mod
language: rust
---

# browser

Library Browser tab — the main-window surface for working with

## Docstring

Library Browser tab — the main-window surface for working with
library rows.

Layout:

```text
┌─[ <name>.snxlib ]──────────────────────────────────────────┐
│ [Resistors] [Capacitors] [Connectors] [+]   Search: [____] │  ← tab strip
├──────────────────────────────────────────┬─────────────────┤
│ ┌──────┬────────┬──────┬─────┬─────┐     │  [ Preview  ]   │
│ │ PN   │ Mfr    │ MPN  │ Val │ Pkg │     │  [   symbol  ]  │
│ │ R10K │ Vishay │ CRC… │ 10k │0805 │  ←  │  ───────────── │
│ │ R47K │ Yageo  │ RC0… │ 47k │0805 │     │  [ footprint ]  │
│ └──────┴────────┴──────┴─────┴─────┘     │                 │
│   Add Component  Delete Selected         │                 │
└──────────────────────────────────────────┴─────────────────┘
```

Phase 1 = read-only-plus-modal-edit semantics. The grid is rendered
as `text` widgets; row click selects (drives the side preview pane);
row double-click is reserved for the upcoming Edit Component Details
modal (Phase 2). Add Component and Delete Selected are wired through
the existing library messages; Delete fires immediately without a
confirm modal until Phase 2 lands.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/browser/mod/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
