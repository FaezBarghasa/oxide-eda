---
okf_version: "0.2"
type: Function
title: history_pane
description: "Build a column-of-cards rendering of `entries` for the SCH"
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane/history_pane
language: rust
---

# history_pane

Build a column-of-cards rendering of `entries` for the SCH

## Signature

```rust
pub fn history_pane(
    entries: &[HistoryEntry],
    now: DateTime<Utc>,
    tokens: &ThemeTokens,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M`

## Visibility

- `pub`

## Docstring

Build a column-of-cards rendering of `entries` for the SCH
Library / Footprint / Sim editor's right-side history pane.

Layout per row (Stage 17 — text only):

```text
┌───────────────────────────────┐
│ alpCaner            12m ago   │  ← author + relative time
│ fix pin 4 typo                │  ← subject
│ a3bbcc6                       │  ← short SHA (muted)
└───────────────────────────────┘
```

Empty `entries` renders a single muted "No history yet." card so
fresh libraries don't show an empty void. `now` is parameterised
to keep the relative-time math testable; production callers pass
`Utc::now()`.

## Source
Lines 72–124 in `crates/oxide-widgets/src/history_pane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_pane](/crates/oxide-widgets/src/history_pane.md) |
| calls | [empty_pane](/crates/oxide-widgets/src/history_pane/empty_pane.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [panel_bg](/crates/oxide-widgets/src/theme_ext/panel_bg.md) |
