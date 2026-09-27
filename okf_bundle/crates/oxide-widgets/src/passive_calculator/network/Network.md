---
okf_version: "0.2"
type: Class
title: Network
description: "[derive(Debug, Clone, PartialEq)]"
resource: crates/oxide-widgets/src/passive_calculator/network.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/network/Network
language: rust
---

# Network

[derive(Debug, Clone, PartialEq)]

## Signature

```rust
pub enum Network
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, PartialEq)]

## Methods

- `condition`
- `component`
- `tolerance`
- `connection`
- `left`
- `right`

## Source
Lines 16–29 in `crates/oxide-widgets/src/passive_calculator/network.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network](/crates/oxide-widgets/src/passive_calculator/network.md) |
| called_by | [search_by_keyword](/crates/oxide-library/src/distributors/digikey/search_by_keyword.md) |
| called_by | [http_post_json](/crates/oxide-library/src/distributors/jlcpcb/http_post_json.md) |
| called_by | [http_get_json](/crates/oxide-library/src/distributors/lcsc/http_get_json.md) |
| called_by | [search_by_keyword](/crates/oxide-library/src/distributors/mouser/search_by_keyword.md) |
