---
okf_version: "0.2"
type: Function
title: classify_bump
description: "Classify a `(current, latest)` semver-style version pair into a"
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/classify_bump
language: rust
---

# classify_bump

Classify a `(current, latest)` semver-style version pair into a

## Signature

```rust
pub fn classify_bump(current: &str, latest: &str) -> BumpKind
```

## Visibility

- `pub`

## Docstring

Classify a `(current, latest)` semver-style version pair into a
[`BumpKind`]. Both inputs are opaque-string-treated semver: the
rule splits on `.` and compares the leading two numeric segments.
Any parse failure or unequal-major pair upgrades to [`BumpKind::Major`].

## Source
Lines 97–112 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
| calls | [parts](/crates/oxide-app/src/library/updates_dialog/parts.md) |
| called_by | [scan_library_updates_for_open_schematic](/crates/oxide-app/src/app/dispatch/library/updates/scan_library_updates_for_open_schematic.md) |
