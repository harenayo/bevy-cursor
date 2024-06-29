use {
    bevy::{
        app::{
            App,
            Startup,
            Update,
        },
        core_pipeline::core_2d::Camera2dBundle,
        ecs::{
            query::With,
            system::{
                Commands,
                Query,
            },
        },
        math::f32::Vec3,
        render::color::Color,
        sprite::{
            Sprite,
            SpriteBundle,
        },
        transform::components::{
            GlobalTransform,
            Transform,
        },
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

fn startup(mut commands: Commands) {
    let camera = commands.spawn(Camera2dBundle::default()).id();

    commands.spawn(CursorRayBundle {
        cursor_ray: CursorRay {
            target: camera,
        },
        ..CursorRayBundle::default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..Sprite::default()
        },
        transform: Transform::from_scale(Vec3::splat(20.0)),
        ..SpriteBundle::default()
    });
}

fn update(
    cursor_rays: Query<&GlobalTransform, With<CursorRay>>,
    mut sprites: Query<&mut Transform, With<Sprite>>,
) {
    let cursor_ray = cursor_rays.single();
    let mut sprite = sprites.single_mut();

    sprite.translation = Vec3 {
        z: 0.0,
        ..cursor_ray.translation()
    };
}
