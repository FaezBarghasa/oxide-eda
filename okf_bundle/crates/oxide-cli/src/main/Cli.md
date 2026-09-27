---
okf_version: "0.2"
type: Class
title: Cli
description: "[derive(Parser)]"
resource: crates/oxide-cli/src/main.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-cli"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:24:52Z"
concept_id: crates/oxide-cli/src/main/Cli
language: rust
---

# Cli

[derive(Parser)]

## Signature

```rust
struct Cli
```

## Decorators

- `derive(Parser)`
- `command(name = "oxide")`
- `command(about = "Headless CLI tool for Oxide EDA: DRC, ERC, CAM and Release automation", long_about = None)`

## Docstring

[derive(Parser)]
[command(name = "oxide")]
[command(about = "Headless CLI tool for Oxide EDA: DRC, ERC, CAM and Release automation", long_about = None)]

## Methods

- `command`

## Source
Lines 12–15 in `crates/oxide-cli/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/crates/oxide-cli/src/main.md) |
