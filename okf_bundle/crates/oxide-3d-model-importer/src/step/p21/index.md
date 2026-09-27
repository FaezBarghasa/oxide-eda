# p21

## Classs

- [Entity](Entity.md) — [derive(Clone, Debug)]
- [ParseError](ParseError.md) — [derive(Debug, thiserror::Error)]
- [StepMeshResult](StepMeshResult.md) — [derive(Debug)]

## Functions

- [extract_data_section](extract_data_section.md)
- [first_ref](first_ref.md)
- [parse_cartesian_point](parse_cartesian_point.md)
- [parse_entities](parse_entities.md)
- [parse_entity_statement](parse_entity_statement.md)
- [parse_minimal_poly_loop_face](parse_minimal_poly_loop_face.md) — [test]
- [parse_missing_data_section_fails](parse_missing_data_section_fails.md) — [test]
- [parse_numbers](parse_numbers.md)
- [parse_refs](parse_refs.md)
- [parse_to_meshes](parse_to_meshes.md) — Parse a STEP ISO-10303-21 DATA section into tessellated triangle meshes.
- [resolve_edge_loop_points](resolve_edge_loop_points.md)
- [resolve_poly_loop_points](resolve_poly_loop_points.md)
- [triangulate_polygon](triangulate_polygon.md)
