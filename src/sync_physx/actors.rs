use bevy::prelude::*;
use physx::{prelude::*, scene::Scene, traits::Class};

use crate::helpers::physx_vec3;
use crate::{PhysXRes, trans_to_physx, PxRigidActorHandle};
use crate::sync_physx::materials::PxMaterial;


//dynamic
#[derive(Component)]
pub struct PxDynamicActor;


pub fn new_dyn_actor(
    mut commands: Commands,
    mut physx: ResMut<PhysXRes>,
    query: Query<(Entity, &Transform), Added<PxDynamicActor>>,
){ 

    for (e, trans) in query.iter() {

        let mut dyn_actor = physx.foundation.physics_mut().create_dynamic(&trans_to_physx(*trans), ()).unwrap();

        let handle = physx.handles.rigid_actors.insert(dyn_actor.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxRigidActorHandle(handle));

        physx.scene.add_dynamic_actor(dyn_actor);

    }

}



//static
#[derive(Component)]
pub struct PxStaticActor;


pub fn new_static_actor(
    mut commands: Commands,
    mut physx: ResMut<PhysXRes>,
    query: Query<(Entity, &Transform), Added<PxStaticActor>>,
){ 

    for (e, trans) in query.iter() {

        let mut static_actor = physx.foundation.physics_mut().create_static(trans_to_physx(*trans), ()).unwrap();

        let handle = physx.handles.rigid_actors.insert(static_actor.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxRigidActorHandle(handle));

        physx.scene.add_static_actor(static_actor);

    }

}



//static ground plane
#[derive(Component)]
pub struct PxPlane{
    normal: Vec3,
    offset: f32,
}

impl PxPlane{
    pub fn new(normal: Vec3, offset: f32) -> Self{
        Self{
            normal,
            offset,
        }
    }
}

impl Default for PxPlane{
    fn default() -> Self{
        Self{
            normal: Vec3::new(0.0, 1.0, 0.0),
            offset: 0.0,
        }
    }
}
    
    
pub fn new_ground_plane(
    mut commands: Commands,
    mut physx: ResMut<PhysXRes>,
    query: Query<(Entity, &PxPlane, Option<&PxMaterial>), Added<PxPlane>>,
){ 
 
    for (e, plane, opt_material) in query.iter() {

        let material = match opt_material{
            Some(mat) => mat.clone(),
            None => PxMaterial::default(),
        };

        let mut px_material = physx.foundation.physics_mut().create_material(material.static_friction, material.dynamic_friction, material.restitution, ()).unwrap();


        let mut ground_plane = physx.foundation.physics_mut()
            .create_plane(physx_vec3(plane.normal), plane.offset, px_material.as_mut(), ())
            .unwrap();
            

        let handle = physx.handles.rigid_actors.insert(ground_plane.as_mut().as_mut_ptr());
        commands.entity(e).insert(PxRigidActorHandle(handle));

        physx.scene.add_static_actor(ground_plane);

    }

}