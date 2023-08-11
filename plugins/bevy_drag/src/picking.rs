use bevy::prelude::*;
use bevy_mod_picking::prelude::{Move, Pointer};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

#[derive(Reflect, Clone)]
pub struct GizmoRaycastSet;

pub type GizmoPickSource = RaycastSource<GizmoRaycastSet>;
pub type PickableGizmo = RaycastMesh<GizmoRaycastSet>;

pub struct GizmoPickingPlugin;

impl GizmoPickingPlugin {
    // Update our `RaycastSource` with the current cursor position every frame.
    fn update_raycast_with_cursor(
        mut cursor: EventReader<Pointer<Move>>,
        mut query: Query<&mut RaycastSource<GizmoRaycastSet>>,
    ) {
        // Grab the most recent cursor event if it exists:
        let Some(Pointer::<Move> {pointer_location, .. }) = cursor.iter().last() else { return };
        for mut pick_source in &mut query {
            pick_source.cast_method = RaycastMethod::Screenspace(pointer_location.position);
        }
    }
}

impl Plugin for GizmoPickingPlugin {
    fn build(&self, app: &mut App) {
        app
            // The DefaultRaycastingPlugin bundles all the functionality you might need into a single
            // plugin. This includes building rays, casting them, and placing a debug cursor at the
            // intersection. For more advanced uses, you can compose the systems in this plugin however
            // you need. For example, you might exclude the debug cursor system.
            .add_plugins(DefaultRaycastingPlugin::<GizmoRaycastSet>::default())
            // You will need to pay attention to what order you add systems! Putting them in the wrong
            // order can result in multiple frames of latency. Ray casting should probably happen near
            // start of the frame. For example, we want to be sure this system runs before we construct
            // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
            // order your systems with the ones provided by the raycasting plugin.
            .add_systems(
                First,
                Self::update_raycast_with_cursor
                    .before(RaycastSystem::BuildRays::<GizmoRaycastSet>),
            );
    }
}
