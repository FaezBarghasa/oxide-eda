---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate/new
language: rust
---

# new

## Signature

```rust
impl Translate<'a, Message, Theme, Renderer> { pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        offset: (f32, f32),
    ) -> Self }
```

## Type Parameters

- `'a`
- `Message`
- `Theme`
- `Renderer`

## Visibility

- `pub`

## Source
Lines 30–38 in `crates/oxide-app/src/app/view/translate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [translate](/crates/oxide-app/src/app/view/translate.md) |
