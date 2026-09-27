---
okf_version: "0.2"
type: Function
title: view_drc
resource: crates/oxide-app/src/panels/drc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:13:22Z"
concept_id: crates/oxide-app/src/panels/drc/view_drc
language: rust
---

# view_drc

## Signature

```rust
pub fn view_drc(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 19–82 in `crates/oxide-app/src/panels/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-app/src/panels/drc.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
