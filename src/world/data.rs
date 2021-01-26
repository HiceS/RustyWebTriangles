extern crate nalgebra as na;

use na::Vector3;

use nphysics3d::object::DefaultBodyHandle;
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

/// All of the data in the World that has to do with bodies
pub struct WorldData {
    bodies:     DefaultBodySet<f32>,
    colliders:  DefaultColliderSet<f32, DefaultBodyHandle>,
    joints:     DefaultJointConstraintSet<f32, DefaultBodyHandle>,
    forces:     DefaultForceGeneratorSet<f32, DefaultBodyHandle>
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            bodies:     DefaultBodySet::new(),
            colliders:  DefaultColliderSet::new(),
            joints:     DefaultJointConstraintSet::new() ,
            forces:     DefaultForceGeneratorSet::new()
        }
    }

    pub fn get_body_count(&self) -> usize {
        return self.bodies.iter().size_hint().0;
    }

    // Takes in a mech world and a geo world and steps the simulation
    pub fn world_step(&mut self, mech_world: & mut DefaultMechanicalWorld<f32>, geo_world: &mut DefaultGeometricalWorld<f32>){
        mech_world.step(
            geo_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.forces
        );
    }


}