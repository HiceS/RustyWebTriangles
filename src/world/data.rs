extern crate nalgebra as na;

use na::Vector3;

use nphysics3d::object::{DefaultBodyHandle, RigidBody, BodySet, BodyHandle, BodyPartHandle, ColliderSet};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use crate::world::body::SimBody;

/// All of the data in the World that has to do with bodies
pub struct WorldData {
    bodies:     DefaultBodySet<f32>,
    colliders:  DefaultColliderSet<f32, DefaultBodyHandle>,
    joints:     DefaultJointConstraintSet<f32, DefaultBodyHandle>,
    forces:     DefaultForceGeneratorSet<f32, DefaultBodyHandle>,
    mechanical_world:   DefaultMechanicalWorld<f32>,
    geometrical_world:  DefaultGeometricalWorld<f32>,
}

pub trait WorldInteraction {
    fn set_gravity(&mut self, x: f32, y: f32, z: f32);

    /// Adds a body, returns a i32 to represent the ID value of the body
    fn add_body(&mut self) -> i32;

    /// Adds a clone of the previous body by ID
    fn clone_body(&mut self, previous_body: i32) -> i32;

    /// Removes a body by ID
    fn remove_body(&mut self, body_id: i32);

}

/// Test Cases not exposed to WASM
impl WorldData {
    pub fn create_test_body(&mut self, sim_body: &SimBody) -> bool {
        let handle = self.bodies.insert(sim_body.build_rb());
        let col_handle = self.colliders.insert(sim_body.build_collider(handle));
        return self.bodies.contains(handle) && self.colliders.contains(col_handle);
    }

    fn remove_test_body(mut self, handle: DefaultBodyHandle) -> bool {
        unimplemented!();
    }
}

/// WorldData exists to implement many world data builder patterns
/// Such that a World object is obfuscated for the wasm bindings
impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            bodies:     DefaultBodySet::new(),
            colliders:  DefaultColliderSet::new(),
            joints:     DefaultJointConstraintSet::new(),
            forces:     DefaultForceGeneratorSet::new(),
            mechanical_world:   DefaultMechanicalWorld::new(Vector3::new(0.0, -9.8, 0.0)),
            geometrical_world:  DefaultGeometricalWorld::new(),
        }
    }

    pub fn get_body_count(&self) -> usize {
        return self.bodies.iter().size_hint().0;
    }

    pub fn set_gravity(&mut self, x: f32, y: f32, z: f32) {
        self.mechanical_world.gravity = Vector3::new(x, y, z);
    }

    pub fn get_gravity(self) -> Vector3<f32>{
        self.mechanical_world.gravity
    }

    /// Simulates the stepping of the physics world
    pub fn world_step(&mut self, delta: i32){
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.forces
        );
    }
}