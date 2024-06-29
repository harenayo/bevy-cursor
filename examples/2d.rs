use {
    bevy::{
        app::{
            App,
            Startup,
            Update,
        },
        core_pipeline::core_2d::Camera2dBundle,
        ecs::{
            query::{
                With,
                Without,
            },
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
    cursor_rays: Query<&Transform, (With<CursorRay>, Without<Sprite>)>,
    mut sprites: Query<&mut Transform, (With<Sprite>, Without<CursorRay>)>,
) {
    sprites.single_mut().translation = Vec3 {
        z: 0.0,
        ..cursor_rays.single().translation
    };
}
