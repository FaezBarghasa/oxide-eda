---
okf_version: "0.2"
type: Function
title: smart_paste_offset
resource: crates/oxide-app/src/app/handlers/clipboard_workflows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/clipboard_workflows/smart_paste_offset
language: rust
---

# smart_paste_offset

## Signature

```rust
fn smart_paste_offset(app: &Oxide) -> (f64, f64)
```

## Source
Lines 119–134 in `crates/oxide-app/src/app/handlers/clipboard_workflows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clipboard_workflows](/crates/oxide-app/src/app/handlers/clipboard_workflows.md) |
| calls | [clipboard_bounds](/crates/oxide-app/src/app/handlers/clipboard_workflows/clipboard_bounds.md) |
| called_by | [handle_clipboard_smart_paste_requested](/crates/oxide-app/src/app/handlers/clipboard_workflows/handle_clipboard_smart_paste_requested.md) |
