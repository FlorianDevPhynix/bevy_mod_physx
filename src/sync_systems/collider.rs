use bevy::prelude::*;
use physx::{prelude::*, traits::Class};


use crate::PhysxRes;
use crate::PxDynamicRigidBodyHandle;

#[derive(Component)]
pub enum Collider {
    Box{half_extents: Vec3},
    Sphere{radius: f32},
    Capsule{radius: f32, half_height: f32},
}



pub fn new_collider(
    mut physx: ResMut<PhysxRes>,
    query: Query<(&Collider, &PxDynamicRigidBodyHandle), Added<Collider>>,
){ 

    unsafe {

        for (collider, handle) in query.iter() {

            match collider {
                Collider::Box{ half_extents }=> {

                    let geom = PxBoxGeometry::new(half_extents.x, half_extents.y, half_extents.z);
                    let px_material = physx_sys::PxPhysics_createMaterial_mut(physx.foundation.physics_mut().as_mut_ptr(),0.6, 0.6, 0.4);
                    
                    let actor = physx.handles.dynamic_actors.get(handle.0).unwrap();

                    physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(*actor as *mut physx_sys::PxRigidActor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                },
                Collider::Sphere { radius } => {

                    let geom = PxSphereGeometry::new(*radius);
                    let px_material = physx_sys::PxPhysics_createMaterial_mut(physx.foundation.physics_mut().as_mut_ptr(),0.6, 0.6, 0.4);
                    
                    let actor = physx.handles.dynamic_actors.get(handle.0).unwrap();

                    physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(*actor as *mut physx_sys::PxRigidActor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                },
                Collider::Capsule { radius, half_height } => {
                        
                    let geom = PxCapsuleGeometry::new(*radius, *half_height);
                    let px_material = physx_sys::PxPhysics_createMaterial_mut(physx.foundation.physics_mut().as_mut_ptr(),0.6, 0.6, 0.4);
                    
                    let actor = physx.handles.dynamic_actors.get(handle.0).unwrap();

                    physx_sys::PxRigidActorExt_createExclusiveShape_mut_1(*actor as *mut physx_sys::PxRigidActor, geom.as_ptr(), px_material, physx_sys::PxShapeFlags{ mBits: 1u64 as u8 });

                },

            }
                
        }
        


    }



}