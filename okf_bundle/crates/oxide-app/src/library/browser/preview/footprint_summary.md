---
okf_version: "0.2"
type: Function
title: footprint_summary
resource: crates/oxide-app/src/library/browser/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/preview/footprint_summary
language: rust
---

# footprint_summary

## Signature

```rust
fn footprint_summary(fp: Option<&oxide_library::Footprint>) -> String
```

## Decorators

- `expect(
    dead_code,
    reason = "F15 removed the preview pane; the builders stay until the Properties panel absorbs them"
)`

## Source
Lines 306–336 in `crates/oxide-app/src/library/browser/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/browser/preview.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
