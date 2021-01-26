extern crate nalgebra as na;

use wasm_bindgen::prelude::*;
use na::Vector3;
use nphysics3d::world::*;
use self::na::RealField;

pub mod data;
pub mod body;


#[wasm_bindgen]
pub struct World {
    pub(crate) world_data: data::WorldData,
}

#[wasm_bindgen]
impl World {
    fn new() -> World {
        World {
            world_data: data::WorldData::new()
        }
    }

    pub fn create_test_body(&mut self) -> bool{
        let sim_body = body::SimBody::new("Test".to_string(), 12.32);
        return self.world_data.create_test_body(&sim_body);
    }

    pub fn set_gravity(&mut self, x: f32, y: f32, z: f32){
        self.world_data.set_gravity(x, y, z)
    }

    pub fn step(&mut self, delta: i32){
        self.world_data.world_step(delta)
    }

}

pub fn setup() -> World{
    return World::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_world() {
        let mut world: World = World::new();
        world.set_gravity(0.0, -9.8, 0.0);
        world.step(13);
        println!("{:?}", world.world_data.get_body_count());
        assert!(world.world_data.get_body_count() >= 0)
    }

    #[test]
    fn add_body() {
        let mut world: World = World::new();
        world.set_gravity(0.0, -9.8, 0.0);

        world.create_test_body();
        assert!(world.create_test_body());

        world.step(13);
        println!("{:?}", world.world_data.get_body_count());
        assert!(world.world_data.get_body_count() >= 0)
    }
}