use bevy::prelude::*;
use slotmap::{SlotMap, new_key_type};


#[derive(Default)]
pub struct Handels {
    pub rigid_actors: SlotMap<RigidActorHandle, *mut physx_sys::PxRigidActor>, //static, dynamic, articulation link
    pub articulations: SlotMap<ArticulationHandle, *mut physx_sys::PxArticulationBase>,//reduced coordinate, maximum coordinate
    //Joints

}


new_key_type! {
    pub struct RigidActorHandle;
    pub struct ArticulationHandle;

}

#[derive(Component)]
pub struct PxRigidActorHandle(pub RigidActorHandle);
#[derive(Component)]
pub struct PxArticulationHandle(pub ArticulationHandle);

