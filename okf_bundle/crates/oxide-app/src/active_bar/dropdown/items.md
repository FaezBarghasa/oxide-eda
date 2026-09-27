---
okf_version: "0.2"
type: Function
title: items
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/items
language: rust
---

# items

## Signature

```rust
fn items(entries: &[DropdownEntry<ActiveBarMsg>]) -> Vec<(String, bool)>
```

## Source
Lines 928–936 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| called_by | [disabled_of](/crates/oxide-app/src/active_bar/dropdown/disabled_of.md) |
| called_by | [labels](/crates/oxide-app/src/active_bar/dropdown/labels.md) |
