---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill/new
language: rust
---

# new

## Signature

```rust
impl TabPill<'a, Message, Theme, Renderer> { pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: TabPillStyle,
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
Lines 78–86 in `crates/oxide-widgets/src/tab_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_pill](/crates/oxide-widgets/src/tab_pill.md) |
