---
okf_version: "0.2"
type: Module
title: uf
description: "Iterative path-compression union-find, generic over the node key."
resource: crates/oxide-net/src/uf.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/uf
language: rust
---

# uf

Iterative path-compression union-find, generic over the node key.

## Docstring

Iterative path-compression union-find, generic over the node key.

Shared by `oxide-net`'s netlist builder / cross-sheet stitcher and
`oxide-erc`'s rules / context. Both used to keep their own recursive copy —
the recursive form was a real stack-overflow vector on degenerate wire
chains >10K segments (HI-17). This is the single canonical implementation.

Most callers union bucketed point keys ([`Key`]); the cross-sheet stitcher
unions richer nodes (an occurrence id plus a per-sheet root), so `find` /
`union` are generic over any `K: Eq + Hash + Copy`.

## Relationships

| Type | Target |
|------|--------|
| related | [find](/crates/oxide-net/src/uf/find.md) |
| related | [union](/crates/oxide-net/src/uf/union.md) |
