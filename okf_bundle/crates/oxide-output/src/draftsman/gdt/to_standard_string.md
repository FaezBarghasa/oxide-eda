---
okf_version: "0.2"
type: Function
title: to_standard_string
description: "Generates a standardized textual representation e.g. \"| [Position] | Ø0.05 (M) | A | B (M) | C |\""
resource: crates/oxide-output/src/draftsman/gdt.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:19:20Z"
concept_id: crates/oxide-output/src/draftsman/gdt/to_standard_string
language: rust
---

# to_standard_string

Generates a standardized textual representation e.g. "| [Position] | Ø0.05 (M) | A | B (M) | C |"

## Signature

```rust
impl FeatureControlFrame { pub fn to_standard_string(&self) -> String }
```

## Visibility

- `pub`

## Docstring

Generates a standardized textual representation e.g. "| [Position] | Ø0.05 (M) | A | B (M) | C |"

## Source
Lines 76–115 in `crates/oxide-output/src/draftsman/gdt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gdt](/crates/oxide-output/src/draftsman/gdt.md) |
