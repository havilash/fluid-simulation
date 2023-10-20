use crate::game::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Cursor {
    pub position: Vector,
    pub force_type: CursorForceType,
    pub radius: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CursorForceType {
    None,
    Attract,
    Repel,
}

impl Cursor {
    pub fn new(position: Vector, force_type: CursorForceType, radius: f32) -> Cursor {
        Cursor {
            position,
            force_type,
            radius,
        }
    }
}
