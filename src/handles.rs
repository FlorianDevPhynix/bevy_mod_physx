use bevy::prelude::*;
use slotmap::{SlotMap, new_key_type};


#[derive(Default)]
pub struct Handels {
    pub static_actors: SlotMap<RigidStaticHandle, *mut physx_sys::PxRigidStatic>,
    pub dynamic_actors: SlotMap<RigidDynamicHandle, *mut physx_sys::PxRigidDynamic>,
    pub articulation_links: SlotMap<ArticulationLinkHandle, *mut physx_sys::PxArticulationLink>,
}


new_key_type! {
    pub struct RigidStaticHandle;
    pub struct RigidDynamicHandle;
    pub struct ArticulationLinkHandle;
}


#[derive(Component)]
pub struct PxStaticRigidBodyHandle(pub RigidStaticHandle);
#[derive(Component)]
pub struct PxDynamicRigidBodyHandle(pub RigidDynamicHandle);
#[derive(Component)]
pub struct PxArticulationLinkHandle(pub ArticulationLinkHandle);