use bevy::prelude::*;


#[derive(Component, Debug, Clone, Copy)]
pub struct PxMaterial {
    pub static_friction: f32,
    pub dynamic_friction: f32,
    pub restitution: f32,
}

impl Default for PxMaterial {
    fn default() -> Self {
        Self {
            static_friction: 0.5,
            dynamic_friction: 0.5,
            restitution: 0.1,
        }
    }
}