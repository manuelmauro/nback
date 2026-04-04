---
name: bevy-shaders
description: 'Bevy custom shaders: Material trait, AsBindGroup, WGSL shader files, extended materials, 2D materials, shader defs, animated shaders, storage buffers, fullscreen post-processing, and compute shaders. Use when writing custom visual effects, materials, or GPU compute logic.'
---

# Bevy Shaders

## Overview

Bevy shaders are written in WGSL and loaded from your assets directory. Custom materials bridge Rust data to the GPU via the `Material` trait and `AsBindGroup` derive.

## Custom 3D Material

### Rust Side

Register the material plugin, define a struct with `AsBindGroup`, implement `Material`:

```rust
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

const SHADER_ASSET_PATH: &str = "shaders/custom_material.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::BLUE,
            color_texture: Some(asset_server.load("textures/icon.png")),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

### WGSL Side (custom_material.wgsl)

```wgsl
#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color
        * textureSample(material_color_texture, material_color_sampler, mesh.uv);
}
```

Key points:
- `#{MATERIAL_BIND_GROUP}` is auto-replaced by Bevy with the correct bind group index
- `VertexOutput` provides `uv`, `world_position`, `world_normal`, etc.
- Binding indices must match `AsBindGroup` attributes

## Custom 2D Material

Use `Material2d` and `Material2dPlugin` for 2D:

```rust
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<Custom2dMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Custom2dMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for Custom2dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_2d.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Custom2dMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Custom2dMaterial {
            color: LinearRgba::BLUE,
            color_texture: Some(asset_server.load("textures/icon.png")),
        })),
        Transform::default().with_scale(Vec3::splat(128.0)),
    ));
}
```

### WGSL (custom_material_2d.wgsl)

```wgsl
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color
        * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}
```

## Animated Shader (Time-Based)

Access `globals.time` from `mesh_view_bindings`:

### Rust

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AnimatedMaterial {}

impl Material for AnimatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
```

### WGSL (animate_shader.wgsl)

```wgsl
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 2.0;
    let t = sin(globals.time * speed) * 0.5 + 0.5;
    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * 1.4;

    let color_a = vec3<f32>(1.0, 0.0, 0.0); // red
    let color_b = vec3<f32>(0.0, 0.0, 1.0); // blue
    let mixed = mix(color_a, color_b, t * distance_to_center);

    return vec4<f32>(mixed, 1.0);
}
```

## Extended Material (Modify PBR)

Extend `StandardMaterial` without rewriting the entire PBR pipeline:

### Rust

```rust
use bevy::pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
struct MyExtension {
    // Start from binding 100 to avoid conflicts with StandardMaterial (0-99)
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                opaque_render_method: OpaqueRendererMethod::Auto,
                ..default()
            },
            extension: MyExtension { quantize_steps: 4 },
        })),
    ));
}
```

### WGSL (extended_material.wgsl)

```wgsl
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct MyExtendedMaterial {
    quantize_steps: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> my_extended_material: MyExtendedMaterial;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Modify PBR input before lighting
    pbr_input.material.base_color.b = pbr_input.material.base_color.r;
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material, pbr_input.material.base_color
    );

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

    // Post-lighting effect: color quantization
    let steps = f32(my_extended_material.quantize_steps);
    out.color = vec4<f32>(vec4<u32>(out.color * steps)) / steps;

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
```

## Shader Defs (Conditional Compilation)

Toggle shader features per-material instance:

```rust
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[bind_group_data(CustomMaterialKey)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    is_red: bool,
}

#[repr(C)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct CustomMaterialKey {
    is_red: bool,
}

impl From<&CustomMaterial> for CustomMaterialKey {
    fn from(material: &CustomMaterial) -> Self {
        Self { is_red: material.is_red }
    }
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader_defs.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if key.bind_group_data.is_red {
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader_defs.push("IS_RED".into());
        }
        Ok(())
    }
}
```

### WGSL

```wgsl
#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
#ifdef IS_RED
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
#else
    return material_color;
#endif
}
```

## Storage Buffer Binding

Pass dynamic GPU data via storage buffers:

```rust
use bevy::render::storage::ShaderStorageBuffer;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[storage(0, read_only)]
    colors: Handle<ShaderStorageBuffer>,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef { "shaders/storage_buffer.wgsl".into() }
    fn fragment_shader() -> ShaderRef { "shaders/storage_buffer.wgsl".into() }
}

fn setup(
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let color_data: Vec<[f32; 4]> = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];
    let colors = buffers.add(ShaderStorageBuffer::from(color_data));
    materials.add(CustomMaterial { colors });
}

// Update buffer at runtime
fn update(
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    handle: Res<MaterialHandle>,
    time: Res<Time>,
) {
    let material = materials.get_mut(&handle.0).unwrap();
    let buffer = buffers.get_mut(&material.colors).unwrap();
    buffer.set_data(/* new data */);
}
```

## Fullscreen Post-Processing

Apply a fullscreen shader effect to the rendered scene using `FullscreenMaterial`:

```rust
use bevy::core_pipeline::{
    core_3d::graph::Node3d,
    fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
};
use bevy::render::{
    extract_component::ExtractComponent,
    render_graph::{InternedRenderLabel, RenderLabel},
    render_resource::ShaderType,
};

// Attach to camera as a component
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
struct MyEffect { intensity: f32 }

impl FullscreenMaterial for MyEffect {
    fn fragment_shader() -> ShaderRef { "shaders/fullscreen_effect.wgsl".into() }
    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}

// Register plugin and spawn camera with effect
App::new()
    .add_plugins((DefaultPlugins, FullscreenMaterialPlugin::<MyEffect>::default()))
    // ...
commands.spawn((Camera3d::default(), MyEffect { intensity: 0.005 }));
```

## AsBindGroup Attribute Reference

| Attribute                  | WGSL Type            | Example                                                     |
|----------------------------|----------------------|-------------------------------------------------------------|
| `#[uniform(N)]`            | `var<uniform>`       | `#[uniform(0)] color: LinearRgba`                           |
| `#[texture(N)]`            | `texture_2d<f32>`    | `#[texture(1)] tex: Option<Handle<Image>>`                  |
| `#[sampler(N)]`            | `sampler`            | `#[sampler(2)] tex: Option<Handle<Image>>`                  |
| `#[storage(N)]`            | `var<storage>`       | `#[storage(0, read_only)] buf: Handle<ShaderStorageBuffer>` |
| `#[storage(N, read_only)]` | `var<storage, read>` | Read-only storage buffer                                    |

## Common WGSL Imports

```wgsl
// 3D PBR vertex output (uv, world_position, world_normal, etc.)
#import bevy_pbr::forward_io::VertexOutput

// 2D mesh vertex output
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Global time, resolution, etc.
#import bevy_pbr::mesh_view_bindings::globals
// Access: globals.time, globals.delta_time, globals.frame_count

// Full PBR pipeline (for ExtendedMaterial)
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material
#import bevy_pbr::pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, alpha_discard}

// Shader module imports from assets folder
#import "shaders/my_module.wgsl"::MY_CONSTANT
```

## Material Trait Methods

The `Material` trait has these overridable methods:
- `fragment_shader()` — required, returns `ShaderRef`
- `vertex_shader()` — optional custom vertex shader
- `alpha_mode(&self)` — optional, default `AlphaMode::Opaque`
- `specialize()` — optional, inject shader defs or modify pipeline descriptor
