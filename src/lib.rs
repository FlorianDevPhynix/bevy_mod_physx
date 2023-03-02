use std::collections::HashMap;

use bevy::prelude::*;
 
use physx::prelude::*;
use physx::scene::Scene;
use physx::traits::Class;
use physx::{physics::PhysicsFoundationBuilder, foundation::DefaultAllocator};

mod helpers;
use helpers::*;

pub mod custom;
pub use custom::*;
use custom::PxScene;

pub mod sync_physx;
pub use sync_physx::*;

pub mod sync_bevy;
pub use sync_bevy::*;

pub mod handles;
pub use handles::*;

pub mod debug_render;
pub use debug_render::*;




pub struct PhysXPlugin;

impl Plugin for PhysXPlugin {
    fn build(&self, app: &mut App) {
        app 
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

            .add_systems((
                // set_changed_transform,
                new_articulation_joint,
                new_collider,
                px_apply_forces, 
                //px_set_joints,
                add_articulation_system,
            ).in_base_set(PhysXPipelineSet::AfterFlush).after(PhysXPipelineSet::Flush).chain())

            //run physx
            .add_system(px_step_simulation.in_base_set(PhysXPipelineSet::RunPhysx).after(PhysXPipelineSet::AfterFlush) )

            //sync bevy
            .add_systems(( //todo: run if changed
                sync_bevy::transform::px_sync_transforms, 
                sync_bevy::velocity::px_write_velocitys
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


#[derive(Resource)]
pub struct PhysXRes {
    foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    scene: Owner<PxScene>,
    actor_to_entity: HashMap<*mut physx_sys::PxRigidActor, Entity>,
    handles: Handels,
}
unsafe impl Send for PhysXRes {}
unsafe impl Sync for PhysXRes {}


#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub entity: Entity,
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

impl PhysXRes {

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
                physx_vec3(origin).as_ptr(),
                physx_vec3(direction).as_ptr(),
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
                            position: vec3_from_pxvec3(hit.position), 
                            normal: vec3_from_pxvec3(hit.normal), 
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
    mut physx: ResMut<PhysXRes>,
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

    commands.insert_resource(PhysXRes{ foundation, scene, handles, actor_to_entity: HashMap::new() });
}



