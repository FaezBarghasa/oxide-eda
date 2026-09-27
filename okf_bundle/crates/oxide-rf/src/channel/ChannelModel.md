---
okf_version: "0.2"
type: Class
title: ChannelModel
description: RF Channel Configuration.
resource: crates/oxide-rf/src/channel.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:27:05Z"
concept_id: crates/oxide-rf/src/channel/ChannelModel
language: rust
---

# ChannelModel

RF Channel Configuration.

## Signature

```rust
pub struct ChannelModel
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

RF Channel Configuration.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `snr_db`
- `path_loss_db`
- `phase_offset_deg`
- `frequency_offset_hz`

## Source
Lines 9–14 in `crates/oxide-rf/src/channel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [channel](/crates/oxide-rf/src/channel.md) |
