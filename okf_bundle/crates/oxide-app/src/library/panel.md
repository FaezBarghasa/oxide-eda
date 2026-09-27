---
okf_version: "0.2"
type: Module
title: panel
description: Library left-dock panel — flat library list (post-WS-K refactor).
resource: crates/oxide-app/src/library/panel.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/panel
language: rust
---

# panel

Library left-dock panel — flat library list (post-WS-K refactor).

## Docstring

Library left-dock panel — flat library list (post-WS-K refactor).

The inline category-tree-with-row-grid the v0.9-refactor-2 plan §10
sketched is now superseded by the main-window Library Browser tab
(see [`crate::library::browser`]). This panel is reduced to a flat
list of open libraries, each with a single `[Open]` button that
opens the browser tab for that library.

Shape:

```text
┌──────────────────────────────────────────┐
│ [Search libraries…]                      │
├──────────────────────────────────────────┤
│ ▸ Loratis-SN-lib-2.snxlib    [Open]      │
│ ▸ Loratis-SN-lib-3.snxlib    [Open]      │
│ ─────────────────────────────────────────  │
│ [+ Open Library…]                        │
└──────────────────────────────────────────┘
```

Per-row click on `[Open]` fires `LibraryMessage::OpenLibraryBrowser`
which routes through the same handler the project-tree double-click
uses.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/panel/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
