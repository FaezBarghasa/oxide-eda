# scene_shader

## Classs

- [OtherSurface](OtherSurface.md) — [derive(Debug, Clone, Copy)]
- [PcbSurface](PcbSurface.md) — The PCB editor's scene surface.
- [ScenePipeline](ScenePipeline.md) — The set of `oxide_gfx` pipelines plus the camera, created once by iced and
- [ScenePrimitive](ScenePrimitive.md) — One frame's worth of scene geometry handed to the GPU. Cheap to build each
- [SceneShaderProgram](SceneShaderProgram.md) — The `shader::Program` mounted when a view routes its `Scene` through the
- [SceneSurface](SceneSurface.md) — One editor surface that draws its `Scene` on the GPU.

## Functions

- [draw](draw.md)
- [draw](draw_1.md)
- [draw](draw_2.md)
- [draw](draw_3.md)
- [each_surface_gets_its_own_primitive_type](each_surface_gets_its_own_primitive_type.md) — The whole point of the [`SceneSurface`] parameter: iced keys a stored
- [log_text_error_once](log_text_error_once.md) — Report the first glyph-atlas failure of each kind and stay silent after.
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md) — Build from an already-tessellated `Scene` and the current screen-space
- [new](new_3.md) — Build from an already-tessellated `Scene` and the current screen-space
- [pipeline_impl_block](pipeline_impl_block.md) — The `impl shader::Pipeline for ScenePipeline` block alone.
- [prepare](prepare.md)
- [prepare](prepare_1.md)
- [primitive_carries_the_scene_and_camera](primitive_carries_the_scene_and_camera.md) — [test]
- [the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover](the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover.md) — `PrepareError::AtlasFull` is the only thing `upload` can fail
- [trim](trim.md)
- [trim](trim_1.md)
- [world_origin_handles_degenerate_scale](world_origin_handles_degenerate_scale.md) — [test]
- [world_origin_is_negative_offset_over_scale](world_origin_is_negative_offset_over_scale.md) — [test]
- [world_origin_matches_the_screen_transform_mapping](world_origin_matches_the_screen_transform_mapping.md) — The mapping this mirrors is `ScreenTransform::world_to_screen`
- [world_origin_mm](world_origin_mm.md) — World coordinate (mm) at the render pass origin (top-left).
