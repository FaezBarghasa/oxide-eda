---
okf_version: "0.2"
type: Function
title: rail_tab
description: Rail tab button (collapsed dock) — rounded corners.
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/rail_tab
language: rust
---

# rail_tab

Rail tab button (collapsed dock) — rounded corners.

## Signature

```rust
pub fn rail_tab(
    tokens: &ThemeTokens,
    is_active: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style + 'static
```

## Visibility

- `pub`

## Docstring

Rail tab button (collapsed dock) — rounded corners.

## Source
Lines 358–380 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_rail](/crates/oxide-app/src/dock/view/view_rail.md) |
