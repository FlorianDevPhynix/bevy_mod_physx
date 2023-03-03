use bevy::prelude::*;
// use physx::traits::Class;
// use physx::prelude::*;

use crate::{PhysXPipelineSet};





pub struct PhysXDebugRenderPlugin;

impl Plugin for PhysXDebugRenderPlugin {

    fn build(&self, app: &mut App) {
        app 
            .add_system(physx_debug_render.after(PhysXPipelineSet::RunPhysx))
            ;
    }
}

//todo: dosent work
pub fn physx_debug_render(
    // mut physx: ResMut<PhysXRes>
) {
    // unsafe {


    //     let scene = physx.scene.as_mut_ptr();

    //     let render_buffer = physx_sys::PxScene_getRenderBuffer_mut(scene);




    //     //points
    //     while physx_sys::PxRenderBuffer_getNbPoints(render_buffer) > 0 {
    //         let point = physx_sys::PxRenderBuffer_getPoints(render_buffer).read();

    //         println!("{}", Vec3::new(point.pos.x, point.pos.y, point.pos.z));
    //     }

    //     //lines
    //     while physx_sys::PxRenderBuffer_getNbLines(render_buffer) > 0 {
    //         let line = physx_sys::PxRenderBuffer_getLines(render_buffer).read();

            
    //         let start = Vec3::new(line.pos0.x, line.pos0.y, line.pos0.z);
    //         let end = Vec3::new(line.pos1.x, line.pos1.y, line.pos1.z);

    //         println!("{} {}", start, end);
    //     }

    //     //triangles
    //     while physx_sys::PxRenderBuffer_getNbTriangles(render_buffer) > 0 {
    //         let _triangle = physx_sys::PxRenderBuffer_getTriangles(render_buffer).read();

    //         // let start = Vec3::new(line.pos0.x, line.pos0.y, line.pos0.z);
    //         // let end = Vec3::new(line.pos1.x, line.pos1.y, line.pos1.z);

    //         println!("triangle");
    //     }


    //     //texts

    // }

}