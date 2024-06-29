use {
    bevy::{
        app::{
            App,
            Startup,
            Update,
        },
        asset::{
            Assets,
            Handle,
        },
        core_pipeline::core_3d::Camera3dBundle,
        ecs::{
            query::{
                With,
                Without,
            },
            system::{
                Commands,
                Query,
                ResMut,
            },
        },
        math::{
            f32::{
                Vec2,
                Vec3,
            },
            primitives::{
                Cuboid,
                Plane3d,
            },
            Ray3d,
        },
        pbr::{
            DirectionalLightBundle,
            PbrBundle,
            StandardMaterial,
        },
        render::{
            color::Color,
            mesh::Mesh,
        },
        transform::components::Transform,
        DefaultPlugins,
    },
    bevy_cursor::{
        CursorRay,
        CursorRayBundle,
        CursorRayPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorRayPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec2::splat(15.0).extend(5.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..Camera3dBundle::default()
        })
        .id();

    commands.spawn(CursorRayBundle {
        cursor_ray: CursorRay {
            target: camera,
        },
        ..CursorRayBundle::default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_scale(Vec2::ZERO.extend(1.0)).looking_at(Vec3::ZERO, Vec3::Z),
        ..DirectionalLightBundle::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(Color::WHITE),
        transform: Transform::from_scale(Vec3::splat(0.4)),
        ..PbrBundle::default()
    });
}

fn update(
    cursor_rays: Query<&Transform, (With<CursorRay>, Without<Handle<Mesh>>)>,
    mut meshes: Query<&mut Transform, (With<Handle<Mesh>>, Without<CursorRay>)>,
) {
    let cursor_ray = cursor_rays.single();

    let ray = Ray3d {
        origin: cursor_ray.translation,
        direction: cursor_ray.forward(),
    };

    if let Option::Some(distance) = ray.intersect_plane(Vec3::ZERO, Plane3d::new(Vec3::Z)) {
        meshes.single_mut().translation = ray.get_point(distance);
    }
}
