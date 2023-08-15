pub mod controller;
pub mod events;
pub mod gizmo;
pub mod math;
pub mod normalize;
pub mod picking;
pub mod popover;

use crate::{
    controller::{TransformController, TransformControllerSettings},
    events::{EntityPointerEvent, TransformEvent},
    math::world_position_view_plane_intersection_world,
};
use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_cameras::CameraMode;
pub use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_mod_picking::{events::PointerCancel, prelude::*};

#[derive(Default)]
pub struct TransformablePlugin<T: CameraMode>(pub T);

impl<T: CameraMode + Send + Sync + 'static> Plugin for TransformablePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<debug::DebugPickingPlugin>(),
        )
        .insert_resource::<TransformControllerSettings>(TransformControllerSettings::default())
        .insert_resource(T::default())
        .add_event::<EntityPointerEvent>()
        .add_event::<TransformEvent>()
        .add_systems(PostStartup, Self::setup_raycast_camera)
        .add_systems(
            Update,
            Self::emit_transform_events.run_if(on_event::<EntityPointerEvent>()),
        )
        .add_systems(
            PostUpdate,
            Self::consume_transform_events.run_if(Self::run_criteria),
        );
    }
}

impl<T: CameraMode + Send + Sync + 'static> TransformablePlugin<T> {
    fn run_criteria(
        _camera_mode: Res<T>,
        controller_settings: Res<TransformControllerSettings>,
    ) -> bool {
        controller_settings.enabled
    }

    pub fn setup_raycast_camera(
        mut commands: Commands,
        camera_query: Query<Entity, (With<Camera>, With<Camera3d>)>,
    ) {
        for entity in camera_query.iter() {
            commands.entity(entity).insert(RaycastPickCamera::default());
        }
    }

    fn emit_transform_events(
        mut drags_rdr: EventReader<EntityPointerEvent>,
        mut transformable_query: Query<TransformableQuery>,
        raycast_camera_query: Query<(&GlobalTransform, &Camera)>,
        mut transform_event_wtr: EventWriter<TransformEvent>,
        mut camera_controller: ResMut<T>,
    ) {
        for event in drags_rdr.iter() {
            use EntityPointerEvent::*;
            match event {
                DragStart {
                    entity,
                    pointer_id: _,
                    pointer_position: _,
                    data,
                } => {
                    camera_controller.lock();

                    if let Ok(TransformableQueryItem {
                        mut controller,
                        transform,
                        ..
                    }) = transformable_query.get_mut(*entity)
                    {
                        controller.drag_start_entity_position = Some(transform.translation);
                        controller.drag_start_pointer_position = data.hit.position;
                    }
                }
                Drag {
                    entity,
                    pointer_id: _,
                    pointer_position,
                    data: _,
                } => match transformable_query.get(*entity) {
                    Ok(TransformableQueryReadOnlyItem {
                        entity,
                        controller,
                        transform: _,
                        ..
                    }) => {
                        let TransformController {
                            enabled,
                            drag_start_entity_position,
                            drag_start_pointer_position,
                        } = controller;

                        if !enabled {
                            continue;
                        }

                        let (camera_transform, camera) = raycast_camera_query.single();
                        let logical_viewport_size = camera.logical_viewport_size().unwrap();
                        let camera_affine3A = camera_transform.affine();
                        let view_mat4 = Mat4::from(camera_affine3A);
                        let inverse_view_mat4 = view_mat4.inverse();
                        let proj_mat4 = Camera::projection_matrix(camera);
                        let inverse_proj_mat4: Mat4 = proj_mat4.inverse();

                        if let (
                            Some(drag_start_entity_position),
                            Some(drag_start_pointer_position),
                        ) = (*drag_start_entity_position, *drag_start_pointer_position)
                        {
                            let offset = drag_start_entity_position - drag_start_pointer_position;

                            let translation = world_position_view_plane_intersection_world(
                                drag_start_pointer_position,
                                *pointer_position,
                                logical_viewport_size,
                                view_mat4,
                                inverse_view_mat4,
                                inverse_proj_mat4,
                            ) + offset;

                            let event = TransformEvent::Translate((entity, translation));

                            transform_event_wtr.send(event)
                        }
                    }

                    Err(err) => {
                        info!("{:?}", err)
                    }
                },
                DragEnd {
                    entity: _,
                    pointer_id: _,
                    pointer_position: _,
                    data: _,
                } => {
                    camera_controller.unlock();
                }
                _ => {}
            }
        }
    }

    fn consume_transform_events(
        mut transform_evts_rdr: EventReader<TransformEvent>,
        mut transformable_query: Query<TransformableQuery>,
    ) {
        for evt in transform_evts_rdr.iter() {
            match evt {
                TransformEvent::Translate((entity, translation)) => {
                    match transformable_query.get_mut(*entity) {
                        Ok(TransformableQueryItem { mut transform, .. }) => {
                            transform.translation = *translation;
                        }
                        Err(_err) => {}
                    }
                }
                TransformEvent::Rotate((_entity, _rotation)) => {}
                TransformEvent::Scale((_entity, _scale)) => {}
            }
        }
    }
}

#[derive(Bundle)]
pub struct Transformable {
    pickable_bundle: PickableBundle,
    raycast_target: RaycastPickTarget, // <- Needed for the raycast backend.
    transform_controller: TransformController,
    drag: On<Pointer<Drag>>,
    dragstart: On<Pointer<DragStart>>,
    dragend: On<Pointer<DragEnd>>,
}

//note: the entity also need to have a mesh component
impl Default for Transformable {
    fn default() -> Transformable {
        Transformable {
            pickable_bundle: PickableBundle::default(),
            raycast_target: RaycastPickTarget::default(),
            transform_controller: TransformController::default(),
            drag: On::<Pointer<Drag>>::send_event::<EntityPointerEvent>(),
            dragstart: On::<Pointer<DragStart>>::send_event::<EntityPointerEvent>(),
            dragend: On::<Pointer<DragEnd>>::send_event::<EntityPointerEvent>(),
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TransformableQuery {
    entity: Entity,
    controller: &'static mut TransformController,
    interaction: &'static Interaction,
    raycast_target: &'static RaycastPickTarget,
    transform: &'static mut Transform,

    #[cfg(feature = "selection")]
    pub selection: &'static PickSelection,
    #[cfg(feature = "highlight")]
    pub highlight: &'static PickHighlight,
    _pickable: With<Pickable>,
    _drag: With<On<Pointer<Drag>>>,
    _dragstart: With<On<Pointer<DragStart>>>,
    _dragend: With<On<Pointer<DragEnd>>>,
}
