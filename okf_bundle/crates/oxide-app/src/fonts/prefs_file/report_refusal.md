---
okf_version: "0.2"
type: Function
title: report_refusal
description: "Report a refused write once, at `Error` level — the default filter"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/report_refusal
language: rust
---

# report_refusal

Report a refused write once, at `Error` level — the default filter

## Signature

```rust
fn report_refusal(path: &Path, context: &str, error: &PrefsLoadError)
```

## Docstring

Report a refused write once, at `Error` level — the default filter
is `LevelFilter::Info` (`crate::diagnostics`), so `debug!` / `warn!`
would never reach the user's Messages panel, and this is one step
away from losing every preference they have.

## Source
Lines 141–154 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [latch](/crates/oxide-app/src/fonts/prefs_file/latch.md) |
| called_by | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
