use bevy::prelude::*;
use physx::traits::Class;
use crate::{prelude::*, handles::PxRigidActorHandle, helpers::physx_vec3};


#[derive(Component)]
pub enum PxMassProperties { //todo: center of mass
    Mass(f32),
    Density(f32),
}



pub fn update_mass_properties_system(
    mut physx: ResMut<PhysX>,
    query: Query<(&PxMassProperties, &PxRigidActorHandle), (Changed<PxMassProperties>, Without<PxStaticActor>)>,
) {

    unsafe {

        for (mass_properties, handle) in query.iter() {

            let actor = *physx.handles.rigid_actors.get_mut(handle.0).unwrap();

            match mass_properties {
                PxMassProperties::Mass(mass) => {
                    physx_sys::PxRigidBody_setMass_mut(actor as *mut physx_sys::PxRigidBody, *mass);
                },
                PxMassProperties::Density(density) => {
                    physx_sys::PxRigidBodyExt_updateMassAndInertia_1(actor as *mut physx_sys::PxRigidBody, *density, physx_vec3(Vec3::ZERO).as_ptr(), false);
                },
            }

        }

    }
}