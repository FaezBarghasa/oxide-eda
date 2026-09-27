---
okf_version: "0.2"
type: Function
title: point_is_connected
description: "True when a wire endpoint, junction, label, or no-connect sits at `pos`,"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/point_is_connected
language: rust
---

# point_is_connected

True when a wire endpoint, junction, label, or no-connect sits at `pos`,

## Signature

```rust
fn point_is_connected(
    pos: &Point,
    wires: &[ErcWire],
    junctions: &[ErcJunction],
    labels: &[ErcLabel],
    no_connects: &[ErcNoConnect],
) -> bool
```

## Docstring

True when a wire endpoint, junction, label, or no-connect sits at `pos`,
compared in the 1 µm key space — the same "same point" definition the
union-find uses, so the gate and the net partition never disagree (D5.5).
Junctions count: a pin may tap a wire mid-span where a junction sits (D5.3).
Buses do not: a bundle is never unioned, so gating on a bus endpoint minted
a one-terminal phantom net and a spurious unconnected-pin warning (D5.4).

## Source
Lines 379–393 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
| calls | [pt_key](/crates/oxide-erc/src/context/pt_key.md) |
| called_by | [project](/crates/oxide-erc/src/context/project.md) |
