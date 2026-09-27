# mod

## Classs

- [Node](Node.md) — Minimal scene-graph node types we care about.
- [ParseError](ParseError.md) — [derive(Debug, thiserror::Error)]
- [Transform](Transform.md) — [derive(Debug, Clone)]
- [VrmlMesh](VrmlMesh.md) — A flat triangle mesh produced from a VRML IndexedFaceSet.

## Functions

- [apply_point](apply_point.md)
- [apply_point](apply_point_1.md)
- [build_mesh](build_mesh.md) — Convert an IndexedFaceSet (positions + face-separated indices) to a triangle mesh.
- [collect_meshes](collect_meshes.md) — ─── Mesh collection ──────────────────────────────────────────────────────────
- [compose_translation](compose_translation.md)
- [compose_translation](compose_translation_1.md)
- [identity](identity.md)
- [identity](identity_1.md)
- [parse](parse.md) — Parse a VRML97 source into a flat list of meshes.
- [parse_diffuse_color](parse_diffuse_color.md) — [test]
- [parse_empty_is_empty](parse_empty_is_empty.md) — [test]
- [parse_single_triangle](parse_single_triangle.md) — [test]
- [parse_src](parse_src.md)
- [parse_transform_applies_translation](parse_transform_applies_translation.md) — [test]
