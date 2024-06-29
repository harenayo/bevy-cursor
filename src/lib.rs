use bevy::{
    app::{
        App,
        Plugin,
        Update,
    },
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{
            QueryEntityError,
            QuerySingleError,
            With,
        },
        system::Query,
    },
    hierarchy::{
        Children,
        Parent,
    },
    log::error,
    math::f32::Vec3,
    render::camera::{
        Camera,
        RenderTarget,
    },
    transform::components::{
        GlobalTransform,
        Transform,
    },
    window::{
        PrimaryWindow,
        Window,
        WindowRef,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Component)]
pub struct CursorRay {
    pub target: Entity,
}

impl Default for CursorRay {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug, Bundle)]
pub struct CursorRayBundle {
    pub cursor_ray: CursorRay,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub fn update_cursor_rays(
    mut cursor_rays: Query<(&CursorRay, &mut Transform, Option<&Parent>)>,
    global_transforms: Query<&GlobalTransform, With<Children>>,
    windows: Query<&Window>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    enum Error<'a> {
        QueryGlobalTransform(QueryEntityError),
        QueryWindow(QueryEntityError),
        QueryPrimaryWindow(QuerySingleError),
        QueryCamera(QueryEntityError),
        CameraTarget(&'a RenderTarget),
        CameraRay,
    }

    cursor_rays
        .iter_mut()
        .map(
            |(cursor_ray, mut cursor_ray_transform, cursor_ray_parent)| -> Result<_, Error> {
                let (camera, camera_global_transform) =
                    cameras.get(cursor_ray.target).map_err(Error::QueryCamera)?;

                let Option::Some(cursor_position) = windows
                    .get(match &camera.target {
                        RenderTarget::Window(WindowRef::Primary) => primary_windows
                            .get_single()
                            .map_err(Error::QueryPrimaryWindow)?,
                        RenderTarget::Window(WindowRef::Entity(window)) => *window,
                        target => return Result::Err(Error::CameraTarget(target)),
                    })
                    .map_err(Error::QueryWindow)?
                    .cursor_position()
                else {
                    return Result::Ok(());
                };

                let camera_ray = camera
                    .viewport_to_world(camera_global_transform, cursor_position)
                    .ok_or(Error::CameraRay)?;

                *cursor_ray_transform = GlobalTransform::IDENTITY
                    .mul_transform(
                        Transform::from_translation(camera_ray.origin)
                            .looking_to(*camera_ray.direction, Vec3::NAN),
                    )
                    .reparented_to(match cursor_ray_parent {
                        Option::Some(parent) => global_transforms
                            .get(parent.get())
                            .map_err(Error::QueryGlobalTransform)?,
                        Option::None => &GlobalTransform::IDENTITY,
                    });

                Result::Ok(())
            },
        )
        .filter_map(Result::err)
        .for_each(|error| match error {
            Error::QueryGlobalTransform(error) => {
                error!("a global transform was not found: {error}")
            },
            Error::QueryCamera(error) => error!("a camera was not found: {error}"),
            Error::QueryWindow(error) => error!("a window was not found: {error}"),
            Error::QueryPrimaryWindow(error) => error!("a primary window was not found: {error}"),
            Error::CameraTarget(target) => error!("{target:?} is not a window"),
            Error::CameraRay => error!("cannot compute a camera ray"),
        });
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct CursorRayPlugin;

impl Plugin for CursorRayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_rays);
    }
}
