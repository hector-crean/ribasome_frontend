use crate::{
    controller::{TransformController, TransformControllerSettings},
    events::{EntityPointerEvent, TransformEvent},
    math::world_position_view_plane_intersection_world,
};
use bevy::{ecs::query::WorldQuery, prelude::*};
use bevy_cameras::CameraMode;
pub use bevy_mod_picking::prelude::*;
use bevy_mod_picking::{
    backends::raycast::RaycastPickingSet, events::PointerCancel, prelude::RaycastPickTarget,
};
use bevy_mod_raycast::RaycastMesh;

// Must insert this bundle into any entity, which we want to make draggeable
#[derive(Bundle, Default)]
pub struct PopoverBundle {
    pickable_bundle: PickableBundle,
    raycast_target: RaycastMesh<RaycastPickingSet>, // <- Needed for the raycast backend.
    popover_controller: PopoverController,
}

#[derive(Resource)]
pub struct PopoverControllerSettings {
    pub enabled: bool,
}
impl Default for PopoverControllerSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Component)]
pub struct PopoverController {
    pub enabled: bool,
}
impl Default for PopoverController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Default)]
pub struct PopoverPlugin;

impl Plugin for PopoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<debug::DebugPickingPlugin>(),
        )
        .insert_resource::<PopoverControllerSettings>(PopoverControllerSettings::default())
        .add_event::<EntityPointerEvent>()
        .add_systems(
            PostUpdate,
            Self::consume_pointer_events.run_if(Self::run_criteria),
        );
    }
}

impl PopoverPlugin {
    fn run_criteria(controller_settings: Res<PopoverControllerSettings>) -> bool {
        controller_settings.enabled
    }

    fn consume_pointer_events(
        mut pointer_evts_rdr: EventReader<EntityPointerEvent>,
        mut popover_on_hover_query: Query<PopupOnHoverQuery>,
    ) {
        for evt in pointer_evts_rdr.iter() {
            match evt {
                EntityPointerEvent::Click {
                    entity,
                    pointer_id,
                    pointer_position,
                    data,
                } => {
                    let query_item = match popover_on_hover_query.get_mut(*entity) {
                        Ok(query_item) => query_item,
                        Err(_) => continue,
                    };
                }
                _ => {}
            }
        }
    }
}

pub struct PopupOnHover {
    pickable_bundle: PickableBundle,
    raycast_target: RaycastPickTarget, // <- Needed for the raycast backend.
    pointer_over: On<Pointer<Over>>,
    pointer_out: On<Pointer<Out>>,
    pointer_down: On<Pointer<Down>>,
    pointer_click: On<Pointer<Click>>,
    pointer_up: On<Pointer<Up>>,
}

impl Default for PopupOnHover {
    fn default() -> Self {
        Self {
            pickable_bundle: PickableBundle::default(),
            raycast_target: RaycastPickTarget::default(),
            pointer_over: On::<Pointer<Over>>::send_event::<EntityPointerEvent>(),
            pointer_out: On::<Pointer<Out>>::send_event::<EntityPointerEvent>(),
            pointer_down: On::<Pointer<Down>>::send_event::<EntityPointerEvent>(),
            pointer_click: On::<Pointer<Click>>::send_event::<EntityPointerEvent>(),
            pointer_up: On::<Pointer<Up>>::send_event::<EntityPointerEvent>(),
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PopupOnHoverQuery {
    entity: Entity,
    interaction: &'static Interaction,
    raycast_target: &'static RaycastPickTarget,

    #[cfg(feature = "selection")]
    pub selection: &'static PickSelection,
    #[cfg(feature = "highlight")]
    pub highlight: &'static PickHighlight,
    _pickable: With<Pickable>,
    _pointer_over: With<On<Pointer<Over>>>,
    _pointer_out: With<On<Pointer<Out>>>,
    _pointer_down: With<On<Pointer<Down>>>,
    _pointer_click: With<On<Pointer<Click>>>,
    _pointer_up: With<On<Pointer<Up>>>,
}
