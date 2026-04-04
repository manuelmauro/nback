---
name: bevy-3d
description: Bevy 3D rendering, lighting, materials, cameras, shapes, PBR, scene composition, hierarchy, animation, and orbit camera. Use when building 3D games or rendering 3D scenes.
---

# Bevy 3D Rendering & Scenes

## Minimal 3D Scene

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane (circle rotated flat)
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

## 3D Shapes

All built-in 3D primitives:

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial::default());

    // Cuboid
    commands.spawn((Mesh3d(meshes.add(Cuboid::default())), MeshMaterial3d(material.clone())));

    // Sphere (icosphere with 5 subdivisions)
    commands.spawn((Mesh3d(meshes.add(Sphere::default().mesh().ico(5).unwrap())), MeshMaterial3d(material.clone())));

    // Sphere (UV sphere: 32 sectors x 18 stacks)
    commands.spawn((Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))), MeshMaterial3d(material.clone())));

    // Capsule
    commands.spawn((Mesh3d(meshes.add(Capsule3d::default())), MeshMaterial3d(material.clone())));

    // Cylinder
    commands.spawn((Mesh3d(meshes.add(Cylinder::default())), MeshMaterial3d(material.clone())));

    // Torus
    commands.spawn((Mesh3d(meshes.add(Torus::default())), MeshMaterial3d(material.clone())));

    // Cone
    commands.spawn((Mesh3d(meshes.add(Cone::default())), MeshMaterial3d(material.clone())));

    // Conical Frustum
    commands.spawn((Mesh3d(meshes.add(ConicalFrustum::default())), MeshMaterial3d(material.clone())));

    // Tetrahedron
    commands.spawn((Mesh3d(meshes.add(Tetrahedron::default())), MeshMaterial3d(material.clone())));

    // Plane with subdivisions
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(material.clone()),
    ));

    // Extrusion (2D shape extruded into 3D)
    commands.spawn((
        Mesh3d(meshes.add(Extrusion::new(RegularPolygon::default(), 1.0))),
        MeshMaterial3d(material.clone()),
    ));
}
```

## Lighting

### Point Light

```rust
commands.spawn((
    PointLight {
        intensity: 100_000.0,
        color: Color::srgb(1.0, 0.0, 0.0),
        shadows_enabled: true,
        range: 100.0,
        ..default()
    },
    Transform::from_xyz(4.0, 8.0, 4.0),
    // Attach a visible mesh as child to visualize the light source
    children![(
        Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            emissive: LinearRgba::new(4.0, 0.0, 0.0, 0.0),
            ..default()
        })),
    )],
));
```

### Directional Light (Sun)

```rust
use bevy::light::CascadeShadowConfigBuilder;

commands.spawn((
    DirectionalLight {
        illuminance: light_consts::lux::OVERCAST_DAY,
        shadows_enabled: true,
        ..default()
    },
    Transform::from_xyz(0.0, 2.0, 0.0)
        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
    // Tighter cascade config for small scenes
    CascadeShadowConfigBuilder {
        first_cascade_far_bound: 4.0,
        maximum_distance: 10.0,
        ..default()
    }
    .build(),
));
```

### Spot Light

```rust
commands.spawn((
    SpotLight {
        intensity: 1_000_000.0,
        color: Color::srgb(0.0, 1.0, 0.0),
        range: 30.0,
        inner_angle: 0.6,
        outer_angle: 0.8,
        shadows_enabled: true,
        ..default()
    },
    Transform::from_xyz(-1.0, 2.0, 0.0)
        .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
));
```

### Ambient Light

```rust
commands.insert_resource(GlobalAmbientLight {
    color: Color::srgb(1.0, 0.5, 0.0),
    brightness: 200.0,
    ..default()
});
```

### Animating Light Direction

```rust
fn animate_light(time: Res<Time>, mut query: Query<&mut Transform, With<DirectionalLight>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}
```

## PBR Materials (StandardMaterial)

```rust
fn setup_materials(mut materials: ResMut<Assets<StandardMaterial>>) {
    // Basic colored
    materials.add(Color::srgb(0.8, 0.7, 0.6));

    // Full PBR control
    materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffd891").unwrap().into(),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    // Emissive (glowing)
    materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.5, 1.0),
        emissive: LinearRgba::new(0.0, 2.0, 4.0, 0.0),
        ..default()
    });

    // Textured
    materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/stone.png")),
        perceptual_roughness: 1.0,
        ..default()
    });

    // Transparent
    materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Alpha mask (cutout)
    materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/foliage.png")),
        alpha_mode: AlphaMode::Mask(0.5),
        cull_mode: None, // render both sides
        ..default()
    });

    // Unlit (no lighting calculations)
    materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });
}
```

### Animating Materials

```rust
fn animate_materials(
    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for handle in &material_handles {
        if let Some(material) = materials.get_mut(handle)
            && let Color::Hsla(ref mut hsla) = material.base_color
        {
            *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
        }
    }
}
```

## Loading glTF Models

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load specific mesh primitive from glTF
    let mesh = asset_server.load(
        GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }
            .from_asset("models/character.gltf"),
    );

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
```

## 3D Camera

### Perspective Camera (Default)

```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
));
```

### Orthographic Camera

```rust
use bevy::camera::ScalingMode;

commands.spawn((
    Camera3d::default(),
    Projection::from(OrthographicProjection {
        scale: 0.01,
        scaling_mode: ScalingMode::WindowSize,
        ..OrthographicProjection::default_3d()
    }),
    Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
));
```

### Orbit Camera

```rust
use std::f32::consts::FRAC_PI_2;
use bevy::input::mouse::AccumulatedMouseMotion;

#[derive(Resource)]
struct CameraSettings {
    orbit_distance: f32,
    pitch_speed: f32,
    yaw_speed: f32,
    pitch_range: std::ops::Range<f32>,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            yaw_speed: 0.004,
            pitch_range: -pitch_limit..pitch_limit,
        }
    }
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    settings: Res<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta.y * settings.pitch_speed)
        .clamp(settings.pitch_range.start, settings.pitch_range.end);
    let yaw = yaw + delta.x * settings.yaw_speed;

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    let target = Vec3::ZERO;
    camera.translation = target - camera.forward() * settings.orbit_distance;
}
```

## Parent-Child Hierarchy

Parent transforms propagate to children:

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let mat = materials.add(Color::srgb(0.8, 0.7, 0.6));

    // Parent with child using children! macro
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Rotator,
        children![(
            Mesh3d(cube),
            MeshMaterial3d(mat),
            Transform::from_xyz(0.0, 0.0, 3.0), // offset from parent
        )],
    ));
}

#[derive(Component)]
struct Rotator;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in &mut query {
        transform.rotate_x(3.0 * time.delta_secs());
    }
}
```

## Rotation Helpers

```rust
fn rotate_entity(time: Res<Time>, mut query: Query<&mut Transform, With<MyMarker>>) {
    for mut transform in &mut query {
        // Rotate around Y axis
        transform.rotate_y(time.delta_secs() / 2.0);

        // Rotate around local X axis
        transform.rotate_x(3.0 * time.delta_secs());

        // Rotate around arbitrary axis
        transform.rotate(Quat::from_axis_angle(Vec3::Y, 0.5 * time.delta_secs()));
    }
}
```

## Viewport to World (3D Ray Casting)

```rust
fn click_to_world(
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, transform) = *camera;
    if let Ok(window) = windows.single() {
        if let Some(ray) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(transform, cursor).ok())
        {
            info!("Ray origin: {}, direction: {}", ray.origin, ray.direction);
        }
    }
}
```

## Environment Map Lighting

```rust
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.0, 8.0),
    EnvironmentMapLight {
        diffuse_map: asset_server.load("environment_maps/diffuse.ktx2"),
        specular_map: asset_server.load("environment_maps/specular.ktx2"),
        intensity: 2_000.0,
        ..default()
    },
));
```

## Camera Exposure

```rust
use bevy::camera::{Exposure, PhysicalCameraParameters};

let params = PhysicalCameraParameters {
    aperture_f_stops: 1.0,
    shutter_speed_s: 1.0 / 125.0,
    sensitivity_iso: 100.0,
    sensor_height: 0.01866,
};

commands.spawn((
    Camera3d::default(),
    Exposure::from_physical_camera(params),
));
```
