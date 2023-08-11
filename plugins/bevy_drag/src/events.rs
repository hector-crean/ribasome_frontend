use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::*;

#[derive(Event, Debug)]
pub enum EntityDragEvent {
    Dragging {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Drag,
    },
    DragStart {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragStart,
    },
    DragEnd {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragEnd,
    },
}

impl From<ListenerInput<Pointer<Drag>>> for EntityDragEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        EntityDragEvent::Dragging {
            entity: event.target,
            data: Drag {
                button: event.button,
                distance: event.distance,
                delta: event.delta,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}
impl From<ListenerInput<Pointer<DragStart>>> for EntityDragEvent {
    fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
        EntityDragEvent::DragStart {
            entity: event.target,
            data: DragStart {
                hit: event.hit,
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}
impl From<ListenerInput<Pointer<DragEnd>>> for EntityDragEvent {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        EntityDragEvent::DragEnd {
            entity: event.target,
            data: DragEnd {
                distance: event.distance,
                button: event.button,
            },
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
