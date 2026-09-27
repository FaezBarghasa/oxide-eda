---
okf_version: "0.2"
type: Function
title: patch_bump
description: "Patch-bump a semver-style `X.Y.Z` string. Returns `<old>.1`"
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/patch_bump
language: rust
---

# patch_bump

Patch-bump a semver-style `X.Y.Z` string. Returns `<old>.1`

## Signature

```rust
pub fn patch_bump(version: &str) -> String
```

## Visibility

- `pub`

## Docstring

Patch-bump a semver-style `X.Y.Z` string. Returns `<old>.1`
when the input doesn't parse (e.g. legacy rows with
`version = "draft"` or empty strings) so the cascade always
produces *some* observable change on the row — the Library
Updates dialog needs that to detect drift.

Examples:

- `"1.2.3"` → `"1.2.4"`
- `"0.0.1"` → `"0.0.2"`
- `""` → `".1"`
- `"draft"` → `"draft.1"`

## Source
Lines 75–87 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| called_by | [apply_cascade_bump](/crates/oxide-library/src/cascade/apply_cascade_bump.md) |
