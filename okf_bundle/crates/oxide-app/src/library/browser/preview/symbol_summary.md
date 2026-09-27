---
okf_version: "0.2"
type: Function
title: symbol_summary
resource: crates/oxide-app/src/library/browser/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/preview/symbol_summary
language: rust
---

# symbol_summary

## Signature

```rust
fn symbol_summary(sym: Option<&oxide_library::Symbol>) -> String
```

## Decorators

- `expect(
    dead_code,
    reason = "F15 removed the preview pane; the builders stay until the Properties panel absorbs them"
)`

## Source
Lines 272–300 in `crates/oxide-app/src/library/browser/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/browser/preview.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
