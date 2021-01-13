extern crate nalgebra as na;

use wasm_bindgen::prelude::*;
use na::Vector3;
use nphysics3d::world::*;

pub mod data;


#[wasm_bindgen]
pub struct World {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    pub(crate) world_data: data::WorldData,
}

#[wasm_bindgen]
impl World {
    // implement all of the functions
    fn new() -> World {
        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector3::new(0.0, -9.8, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            world_data: data::WorldData::new()
        }
    }

    /// Sets the gravity to a new value
    pub fn set_gravity(&mut self, x: f32, y: f32, z: f32) {
        self.mechanical_world.gravity = Vector3::new(x, y, z);
    }
}

pub fn setup() -> World{
    return World::new();
    // let mut mech_world = DefaultMechanicalWorld::new(Vector3(0.0, -9.8, 0.0));
    // let mut geo_world = DefaultGeometricalWorld::new();

    // let mut bodies = DefaultBodySet::new();
    // let mut colliders = DefaultColliderSet::new();
    // let mut joint_constraints = DefaultJointConstraintSet::new();
    // let mut force_generators = DefaultForceGeneratorSet::new();

    // now that the world variables have been configured we can step the simulation
}