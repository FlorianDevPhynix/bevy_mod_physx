use std::collections::HashMap;

use bevy::prelude::*;
use physx::{prelude::*, physics::PhysicsFoundationBuilder, foundation::DefaultAllocator, traits::Class, scene::Scene};


mod helpers;
use helpers::*;

mod custom;
use custom::*;

mod sync_physx;
use sync_physx::*;

mod sync_bevy;
use sync_bevy::*;

mod handles;
use handles::*;

// mod debug_render;
// use debug_render::*;


pub mod prelude {
    pub use crate::PhysXPlugin;
    pub use crate::PhysX;
    pub use crate::RaycastHit;
    pub use crate::sync_bevy::*;
    pub use crate::sync_physx::*;
}


pub struct PhysXPlugin;

impl Plugin for PhysXPlugin {
    fn build(&self, app: &mut App) {
        app 
            //register
            .register_type::<PxPlane>()

            //setup physx
            .add_startup_system(setup_physx)

            //sync Physx
            .add_systems((
                new_ground_plane,
                new_static_actor, 
                new_dyn_actor, 
                new_articulation, //todo add articulation to scene after setting joints
            ).in_base_set(PhysXPipelineSet::BeforeFlush).chain())

            .add_system(apply_system_buffers.in_base_set(PhysXPipelineSet::Flush).after(PhysXPipelineSet::BeforeFlush)) //clear commands for new components

            .add_systems(( //after flush
                new_articulation_joint,
                new_collider,
                // update_changed_transform,
                update_mass_properties_system,
                update_damping_system,
                update_articulation_joint_drive,
                add_articulation_system,
                px_apply_forces, 
            ).in_base_set(PhysXPipelineSet::AfterFlush).after(PhysXPipelineSet::Flush).chain())

            //run physx
            .add_system(px_step_simulation.in_base_set(PhysXPipelineSet::RunPhysx).after(PhysXPipelineSet::AfterFlush) )

            //sync bevy
            .add_systems(( //todo: run if changed maybe
                sync_transforms, 
                sync_velocitys
            ).in_base_set(PhysXPipelineSet::SyncBevy).after(PhysXPipelineSet::RunPhysx))
            ;

    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum PhysXPipelineSet {
    BeforeFlush, //
    Flush,       //  Sync PhysX
    AfterFlush,  //
    RunPhysx,
    SyncBevy,
}


#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub entity: Entity,
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Resource)]
pub struct PhysX {
    foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    scene: Owner<PxScene>,
    actor_to_entity: HashMap<*mut physx_sys::PxRigidActor, Entity>,
    handles: Handels,
}
unsafe impl Send for PhysX {}
unsafe impl Sync for PhysX {}


impl Drop for PhysX {
    fn drop(&mut self) {
        unsafe {
            // physx_sys::phys_PxCloseExtensions();
            physx_sys::PxScene_release_mut(self.scene.as_mut_ptr());
            physx_sys::PxPhysics_release_mut(self.foundation.physics_mut().as_mut_ptr());
            physx_sys::PxFoundation_release_mut(self.foundation.foundation_mut().as_mut_ptr());
        }

    }
}


impl PhysX {

    fn insert_rigid_actor(&mut self, entity: Entity, actor: *mut physx_sys::PxRigidActor) -> PxRigidActorHandle {

        let handle = self.handles.rigid_actors.insert(actor);
        self.actor_to_entity.insert(actor, entity);

        return PxRigidActorHandle(handle);

    }
    
    /// Raycast from origin in direction, returns entity, distance, position, normal
    /// returns None if no hit
    /// returns Some((entity, distance, position, normal)) if hit
    pub fn raycast(&mut self, origin: Vec3, direction: Vec3, max_distance: f32) -> Option<RaycastHit> {

        unsafe {

            let filter_data = physx_sys::PxQueryFilterData_new();
            let mut hit = std::mem::MaybeUninit::uninit();
 
            if physx_sys::PxSceneQueryExt_raycastSingle(
                self.scene.as_mut_ptr(),
                vec3_to_physx(origin).as_ptr(),
                vec3_to_physx(direction).as_ptr(),
                max_distance,
                physx_sys::PxHitFlags::Default,
                hit.as_mut_ptr(),
                &filter_data,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) {

                let hit = hit.assume_init();

                match self.actor_to_entity.get(&hit.actor) {
                    Some(entity) => {
                        return Some(RaycastHit { 
                            entity: *entity, 
                            distance: hit.distance, 
                            position: vec3_from_physx(hit.position), 
                            normal: vec3_from_physx(hit.normal), 
                        });
                    }
                    None => {
                        panic!("Error: Raycast hit actor without entity");
                    }
                }

            } else {
                return None;
            }
 
        }

    }

}


const PHYSXSTEP: f32 = 1.0 / 60.0;

//run physx
fn px_step_simulation(   
    mut physx: ResMut<PhysX>,
    time: Res<Time>,
    mut accumilator: Local<f32>,
){

    *accumilator += time.delta_seconds();

    if *accumilator >= PHYSXSTEP {
        *accumilator -= PHYSXSTEP;

        physx.scene.simulate(PHYSXSTEP, None, None);

        physx.scene.fetch_results(true).expect("PhysX simulation failed");
    }
}



fn setup_physx(   
    mut commands: Commands,
){

    let mut foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>;

    let mut builder = PhysicsFoundationBuilder::<physx::foundation::DefaultAllocator>::new(DefaultAllocator);

    builder.enable_visual_debugger(true);
    builder.with_extensions(true);
    builder.set_pvd_host("127.0.0.1");


    //with or without PVD
    foundation = match builder.build::<PxShape>() {
        Some(found) => found,
        None => Default::default(),
    };


    let scene: Owner<PxScene> = foundation
        .create(SceneDescriptor {
            gravity: PxVec3::new(0.0, -9.81, 0.0),
            on_advance: Some(OnAdvance),
            simulation_filter_shader: FilterShaderDescriptor::Custom(costum_filter_shader),
            thread_count: std::thread::available_parallelism().unwrap().get() as u32,
            ..SceneDescriptor::new(()) 
        })
        .unwrap();

    // unsafe { physx_sys::PxScene_setVisualizationParameter_mut(scene.as_mut_ptr(), 0u32, 1.0); }

    let handles = Handels::default();

    commands.insert_resource(PhysX{ foundation, scene, handles, actor_to_entity: HashMap::new() });
}



