---
okf_version: "0.2"
type: Class
title: EditorMsg
description: Component Preview inner messages. The surface is preview-only
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/EditorMsg
language: rust
---

# EditorMsg

Component Preview inner messages. The surface is preview-only

## Signature

```rust
pub enum EditorMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Component Preview inner messages. The surface is preview-only
for Symbol/Footprint; the canvas messages stay defined here so
the standalone `.snxsym` / `.snxfpt` document tabs can reuse
them, but they no longer dispatch through the Component Preview
tab.
[derive(Debug, Clone)]

## Methods

- `pin`
- `value`
- `pin`
- `pad`
- `pin`
- `idx`
- `value`
- `idx`
- `value`
- `idx`
- `value`
- `idx`
- `value`
- `idx`
- `idx`
- `value`
- `idx`
- `value`
- `idx`
- `value`
- `idx`
- `name`
- `value`
- `name`
- `buf`
- `name`
- `name`
- `buf`
- `name`
- `unit`
- `name`
- `value`
- `name`
- `name`
- `kind`
- `pin_number`
- `value`

## Source
Lines 46–277 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
