---
okf_version: "0.2"
type: Function
title: labeled_field
resource: crates/oxide-app/src/library/editor/footprint/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d/labeled_field
language: rust
---

# labeled_field

## Signature

```rust
fn labeled_field(
    label: &'static str,
    field: impl Into<Element<'a, LibraryMessage>>,
    muted: iced::Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 136–148 in `crates/oxide-app/src/library/editor/footprint/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-app/src/library/editor/footprint/body3d.md) |
