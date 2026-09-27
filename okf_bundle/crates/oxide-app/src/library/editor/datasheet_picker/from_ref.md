---
okf_version: "0.2"
type: Function
title: from_ref
description: "Derive the display mode from the current `DatasheetRef`. None"
resource: crates/oxide-app/src/library/editor/datasheet_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/datasheet_picker/from_ref
language: rust
---

# from_ref

Derive the display mode from the current `DatasheetRef`. None

## Signature

```rust
impl DatasheetMode { pub fn from_ref(r: Option<&DatasheetRef>) -> Self }
```

## Visibility

- `pub`

## Docstring

Derive the display mode from the current `DatasheetRef`. None
of the options is "no datasheet" — that's the URL mode with an
empty buffer.

## Source
Lines 35–40 in `crates/oxide-app/src/library/editor/datasheet_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet_picker](/crates/oxide-app/src/library/editor/datasheet_picker.md) |
