---
okf_version: "0.2"
type: Function
title: render
description: Rasterize oxide-mark.svg to a PNG at size×size. Returns PNG bytes.
resource: tools/build_icons.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:build_icons.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/build_icons/render
language: python
---

# render

Rasterize oxide-mark.svg to a PNG at size×size. Returns PNG bytes.

## Signature

```python
def render(size: int) -> bytes
```

## Docstring

Rasterize oxide-mark.svg to a PNG at size×size. Returns PNG bytes.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `size` | `int` | `—` |

## Returns
`bytes`

## Source
Lines 39–43 in `tools/build_icons.py`

## Relationships

| Type | Target |
|------|--------|
| related | [build_icons](/tools/build_icons.md) |
| called_by | [main](/tools/build_icons/main.md) |
