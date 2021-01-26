use wasm_bindgen::prelude::*;

extern crate nalgebra as na;
use na::Vector3;

extern crate uuid;
use uuid::Uuid;

use nphysics3d::object::{DefaultBodyHandle, ColliderDesc, RigidBodyDesc, BodyStatus, RigidBody, BodySet, Collider, BodyPartHandle};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics3d::material::{BasicMaterial, MaterialHandle};
use nalgebra::Point3;
use ncollide3d::shape::{ShapeHandle, TriMesh, Ball};

/// This struct holds the data needed to construct a BodyHandle
/// Also needs to be able to construct a collider DESC
#[wasm_bindgen]
pub struct SimBody {
    /// Name of the simulation body
    name: String,
    /// ID value of the body in the simulation - uuid4
    id: Uuid,
    /// Margin around the body for collision detection
    margin: f32,
    /// Does this body generate only proximity events (no-collide)
    /// by Default this is false
    sensor: bool,
    /// Mass of the body (units tbd)
    mass: f32
}

/// Internal implementation
/// Once added to world - world can call these functions
impl SimBody {
    /// Builds a RigidBody
    /// To be supplied to the geo world
    /// This is the definition of the physical attributes of the body
    /// Ideally should return a Tuple of the RB and CollisionData
    pub fn build_rb(&self) -> RigidBody<f32>{
        /// default example of a built body
        /// this builder can actually build many bodies by continuously constructing build
        let mut rb_desc = RigidBodyDesc::new()
            .gravity_enabled(true)
            .status(BodyStatus::Dynamic)
            .linear_damping(2.0)
            .angular_damping(2.0)
            .max_linear_velocity(10.0)
            .max_angular_velocity(10.0)
            .mass(self.mass)
            .linear_motion_interpolation_enabled(true)
            .sleep_threshold(None)
            .local_center_of_mass(Point3::new( 0.0, 0.0, 0.0));

        // This body should not sleep in this case
        // However this shouldn't always be the case
        rb_desc.set_sleep_threshold(None);

        // create the bodyset
        // let mut bs = DefaultBodySet::new();

        // This handle stores a reference to the body in the set
        // call bs.rigid_body(handle)
        // let handle = bs.insert(rb_desc.build());

        // this needs to return a body set? is there a nesting relationship in body sets here ?
        return rb_desc.build();
    }

    /// Builds a collider and attaches it to a given object handle
    pub fn build_collider(&self, body_handle: DefaultBodyHandle) -> Collider<f32, DefaultBodyHandle> {
        // The triangle mesh shape ideally
        // Sample ball for now
        let ball_shape = ShapeHandle::new(Ball::new(1.5));

        let coll_desc = ColliderDesc::new(ball_shape)
            .sensor(self.sensor)
            .density(1.3)
            .material(MaterialHandle::new(
                BasicMaterial::new(0.3, 0.8)
            ))
            .margin(self.margin)
            .build(BodyPartHandle(body_handle, 0));

        return coll_desc;
    }
}

#[wasm_bindgen]
impl SimBody {
    /// Create a new SimBody by specifying the data
    pub fn new(name: String, mass: f32) -> SimBody{
        SimBody {
            name,
            id: Uuid::new_v4(),
            margin: 0.01,
            sensor: false,
            mass
        }
    }

    pub fn name(self) -> String {
        self.name
    }
}

/// Material Structure
/// Contains name and basic physical data
/// I expect there will be multiple implementations that carry additional data
#[wasm_bindgen]
pub struct Material {
    name: String,
    /// Bounciness kinda - default 0.0
    restitution: f32,
    /// Resistance to movement - default 0.5
    friction: f32
}

/// Hidden implementations for Material (from WASM)
impl Material {
    /// Constructs a basic material with the given data
    pub fn basic(&self) ->  MaterialHandle<f32>{
        MaterialHandle::new(BasicMaterial::new(self.restitution, self.friction))
    }

    /// just create a generic default material
    pub fn default() -> MaterialHandle<f32>{
        MaterialHandle::new(BasicMaterial::default())
    }
}

/// The WASM function implementations for Material
#[wasm_bindgen]
impl Material {
    /// new material with option of specifying friction and restitution
    pub fn new(name: String) -> Material {
        Material {
            name,
            restitution: 0.0,
            friction: 0.5
        }
    }

    pub fn name(self) -> String {
        self.name
    }
}