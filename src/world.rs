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

    pub fn step(&mut self, delta: i32){
        self.world_data.world_step(
            &mut self.mechanical_world,
            &mut self.geometrical_world
        );
    }
}

pub fn setup() -> World{
    return World::new();

    // now that the world variables have been configured we can step the simulation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_world() {
        let mut world: World = World::new();
        world.set_gravity(0.0, -9.8, 0.0);
        world.step(13);
        assert!(world.world_data.get_body_count() >= 0)
    }
}