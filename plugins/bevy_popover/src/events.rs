use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::*;

#[derive(Event, Debug)]
pub enum EntityPointerEvent {
    Out {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Out,
    },
    Over {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Over,
    },
}

impl From<ListenerInput<Pointer<Out>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        EntityPointerEvent::Out {
            entity: event.target,
            data: Out { hit: event.hit },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}
impl From<ListenerInput<Pointer<Over>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        EntityPointerEvent::Over {
            entity: event.target,
            data: Over { hit: event.hit },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

#[derive(Event, Debug)]
pub enum TransformEvent {
    Translate((Entity, Vec3)),
    Rotate((Entity, Quat)),
    Scale((Entity, Vec3)),
}
