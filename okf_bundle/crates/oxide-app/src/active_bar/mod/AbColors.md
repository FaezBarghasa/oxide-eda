---
okf_version: "0.2"
type: Class
title: AbColors
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/AbColors
language: rust
---

# AbColors

## Signature

```rust
struct AbColors
```

## Decorators

- `derive(Clone, Copy)`
- `expect(
    dead_code,
    reason = "palette carried on the struct so the dropdown helpers need not rebuild it"
)`

## Methods

- `text`
- `bar_bg`
- `bar_border`
- `drop_bg`
- `drop_border`
- `sep`
- `hover`

## Source
Lines 93–101 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
