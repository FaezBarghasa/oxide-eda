---
okf_version: "0.2"
type: Function
title: build_footer
description: "Dynamic footer: Save + Close (clean) or ⚠ + Discard + Save (dirty)."
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/build_footer
language: rust
---

# build_footer

Dynamic footer: Save + Close (clean) or ⚠ + Discard + Save (dirty).

## Signature

```rust
fn build_footer(dirty: bool) -> Option<Element<'a, PrefMsg>>
```

## Type Parameters

- `'a`

## Docstring

Dynamic footer: Save + Close (clean) or ⚠ + Discard + Save (dirty).
Footer is rendered only when dirty — the X button in the header is the
canonical close action; a redundant "Close" button at the bottom is
noise. Save / Discard appear when there are unsaved changes.

## Source
Lines 676–726 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
| called_by | [build_dialog](/crates/oxide-app/src/preferences/mod/build_dialog.md) |
