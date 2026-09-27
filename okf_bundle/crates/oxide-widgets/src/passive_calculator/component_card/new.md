---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-widgets/src/passive_calculator/component_card.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/component_card/new
language: rust
---

# new

## Signature

```rust
impl ComponentCard<'a> { pub fn new(
        index: usize,
        kind: ComponentKind,
        component: PreferredComponent,
        tolerance: Tolerance,
        tokens: &'a ThemeTokens,
    ) -> Self }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 25–39 in `crates/oxide-widgets/src/passive_calculator/component_card.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component_card](/crates/oxide-widgets/src/passive_calculator/component_card.md) |
