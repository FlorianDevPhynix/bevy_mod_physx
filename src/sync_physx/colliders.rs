use bevy::prelude::*;
use physx::{prelude::*, traits::Class};


use crate::sync_physx::materials::PxMaterial;
use crate::PxPlane;
use crate::PhysX;
use crate::PxRigidActorHandle;
use crate::trans_to_physx;


#[derive(Component)]
pub enum PxCollider {
    Box{size: Vec3},
    Sphere{radius: f32},
    Capsule{radius: f32, depth: f32},
}



pub fn new_collider(
    mut physx: ResMut<PhysX>,
    query: Query<(Entity, &PxCollider, Option<&PxMaterial>, Option<&PxRigidActorHandle>), (Added<PxCollider>, Without<PxPlane>)>,
    parent_q: Query<(&Parent, &Transform)>,
    actor_handle_q: Query<&PxRigidActorHandle>,
){ 

    unsafe {


        for (e, collider, opt_material, opt_handle) in query.iter() {
 
            //flag
            let flags = physx_sys::PxShapeFlags::SimulationShape | physx_sys::PxShapeFlags::SceneQueryShape; //todo

            //material
            let material = match opt_material{
                Some(mat) => mat.clone(),
                None => PxMaterial::default(),
            }; 
    
            let px_material = physx.foundation.physics_mut().create_material(material.static_friction, material.dynamic_friction, material.restitution, ()).unwrap();

            //offset 
            let mut opt_collider_offset = None;

            //actor
            let handle = match opt_handle {
                Some(some) => some, 
                None => { //child collider, todo: remove unwrap
                    let (parent, collider_offset) = parent_q.get(e).unwrap();
                    opt_collider_offset = Some(collider_offset);
                    let parent_handle = actor_handle_q.get(parent.get()).unwrap();
                    parent_handle
                },
            };

            let actor = *physx.handles.rigid_actors.get(handle.0).unwrap();
            

            match collider {
                PxCollider::Box{ size } => {

                    let geom = PxBoxGeometry::new(size.x / 2.0, size.y / 2.0, size.z / 2.0);

                    let shape = physx_sys::PxRigidActorExt_createExclusiveShape_1(actor, geom.as_ptr(), px_material.as_ptr(), flags);

                    if let Some(collider_offset) = opt_collider_offset {
                        physx_sys::PxShape_setLocalPose_mut(shape, trans_to_physx(*collider_offset).as_ptr());
                    }
                },
                PxCollider::Sphere { radius } => {

                    let geom = PxSphereGeometry::new(*radius);

                    let shape = physx_sys::PxRigidActorExt_createExclusiveShape_1(actor, geom.as_ptr(), px_material.as_ptr(), flags);

                    if let Some(collider_offset) = opt_collider_offset {
                        physx_sys::PxShape_setLocalPose_mut(shape, trans_to_physx(*collider_offset).as_ptr());
                    }
                },
                PxCollider::Capsule { radius, depth } => {
                        
                    let geom = PxCapsuleGeometry::new(*radius, *depth / 2.0);

                    let shape = physx_sys::PxRigidActorExt_createExclusiveShape_1(actor, geom.as_ptr(), px_material.as_ptr(), flags);

                    //rotate capsule upright to fit with bevy's coordinate system
                    let local_pose = Transform::from_rotation(Quat::from_rotation_z((90f32).to_radians()));

                    if let Some(collider_offset) = opt_collider_offset {
                        physx_sys::PxShape_setLocalPose_mut(shape, trans_to_physx(collider_offset.mul_transform(local_pose)).as_ptr());
                    } else {
                        physx_sys::PxShape_setLocalPose_mut(shape, trans_to_physx(local_pose).as_ptr());
                    }
                },
    
            }
        }
    }

}


