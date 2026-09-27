---
okf_version: "0.2"
type: Function
title: item
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/item
language: rust
---

# item

## Signature

```rust
impl EntrySpec { fn item(
        icon: fn(ThemeId) -> svg::Handle,
        label: &'static str,
        action: ActiveBarAction,
    ) -> Self }
```

## Source
Lines 127–137 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
