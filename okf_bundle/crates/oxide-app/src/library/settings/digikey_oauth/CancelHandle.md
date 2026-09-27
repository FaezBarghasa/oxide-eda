---
okf_version: "0.2"
type: Class
title: CancelHandle
description: Hands the caller a way to cancel the in-flight handshake. Cloning
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/CancelHandle
language: rust
---

# CancelHandle

Hands the caller a way to cancel the in-flight handshake. Cloning

## Signature

```rust
pub struct CancelHandle
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Hands the caller a way to cancel the in-flight handshake. Cloning
is cheap and lets the iced `Cancel` button dispatch a cancel from
any thread.
[derive(Debug, Clone)]

## Methods

- `flag`

## Source
Lines 67–69 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
