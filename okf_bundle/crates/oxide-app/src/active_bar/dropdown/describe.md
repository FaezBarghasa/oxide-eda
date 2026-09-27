---
okf_version: "0.2"
type: Function
title: describe
description: "Stable serializer for `DropdownEntry` rows, used by the"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/describe
language: rust
---

# describe

Stable serializer for `DropdownEntry` rows, used by the

## Signature

```rust
fn describe(entries: &[DropdownEntry<ActiveBarMsg>]) -> Vec<String>
```

## Docstring

Stable serializer for `DropdownEntry` rows, used by the
behaviour-proof golden test below (#457) to prove the
data-table refactor is byte-for-byte output identical to the
pre-refactor per-menu builder functions.

## Source
Lines 1043–1060 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| called_by | [dropdown_entries_match_pre_refactor_golden](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries_match_pre_refactor_golden.md) |
