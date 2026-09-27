---
okf_version: "0.2"
type: Function
title: reported_failures
description: "Paths currently in the refused state, so the report below fires on"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/reported_failures
language: rust
---

# reported_failures

Paths currently in the refused state, so the report below fires on

## Signature

```rust
fn reported_failures() -> &'static Mutex<HashSet<PathBuf>>
```

## Docstring

Paths currently in the refused state, so the report below fires on
the transition into failure rather than on every attempt.
`write_component_filter` runs once per keystroke in the Components
panel filter box, so an unconditional `error!` would fill the
Messages panel with one copy of the same line per character typed.

## Source
Lines 122–125 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [latch](/crates/oxide-app/src/fonts/prefs_file/latch.md) |
