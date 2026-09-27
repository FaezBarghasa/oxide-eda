---
okf_version: "0.2"
type: Class
title: FileMsg
description: File / save message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/FileMsg
language: rust
---

# FileMsg

File / save message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum FileMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

File / save message family (ADR-0001 D3). Namespaced under
`Message::File` and routed to `dispatch_file_message`.
[derive(Debug, Clone)]

## Methods

- `path`
- `title`
- `result`
- `path`
- `title`
- `result`
- `from_path`
- `to_path`

## Source
Lines 245–286 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
