pub enum Handedness {
    Left,
    Right,
}
pub enum UpDirection {
    X,
    Y,
    Z,
}

pub struct CoordinateSystem {
    handedness: Handedness,
    up_direction: UpDirection,
}
