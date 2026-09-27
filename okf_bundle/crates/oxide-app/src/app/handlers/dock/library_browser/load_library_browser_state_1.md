---
okf_version: "0.2"
type: Function
title: load_library_browser_state
resource: crates/oxide-app/src/app/handlers/dock/library_browser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/library_browser/load_library_browser_state_1
language: rust
---

# load_library_browser_state

## Signature

```rust
fn load_library_browser_state(&mut self, selected_library: String) -> Result<()>
```

## Source
Lines 59–107 in `crates/oxide-app/src/app/handlers/dock/library_browser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser](/crates/oxide-app/src/app/handlers/dock/library_browser.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
