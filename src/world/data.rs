extern crate nalgebra as na;

use nphysics3d::object::*;

/// All of the data in the World that has to do with bodies
pub struct WorldData {
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32, DefaultBodyHandle>
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
        }
    }
}