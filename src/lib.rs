use bevy::prelude::*;

use physx::prelude::*;
use physx::scene::Scene;
use physx::{physics::PhysicsFoundationBuilder, foundation::DefaultAllocator};

mod helpers;
use helpers::*;

pub mod custom;
pub use custom::*;

pub mod sync_physx;
pub use sync_physx::*;

pub mod sync_bevy;
pub use sync_bevy::*;

pub mod handles;
pub use handles::*;




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
                new_articulation_joint,
                new_collider,
                px_apply_forces, 
                //px_set_joints,
                // set_changed_transform,
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
    pub foundation: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    pub scene: Owner<PxScene>,
    pub handles: Handels,
}
unsafe impl Send for PhysXRes {}
unsafe impl Sync for PhysXRes {}



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



    let handles = Handels::default();

    commands.insert_resource(PhysXRes{ foundation, scene, handles });
}



