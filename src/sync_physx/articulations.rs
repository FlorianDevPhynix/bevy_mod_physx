use bevy::prelude::*;
use physx::{prelude::{ArticulationJointType, ArticulationAxis, ArticulationMotion, ArticulationDriveType}, traits::Class};
use physx_sys::{PxPhysics_createAggregate_mut, PxRigidActor, PxArticulationLink as PxArticulationLink_sys};
 
use crate::{PhysX, trans_to_physx, PxRigidActorHandle, PxArticulationHandle};



#[derive(Component)]
pub struct PxArticulation {
    links: Vec<(Entity, Option<Entity>)>, //link, parent
}


impl PxArticulation {

    pub fn new() -> Self {
        Self{
            links: Vec::new(),
        }
    }

    pub fn create_link(&mut self, commands: &mut Commands, parent: Option<Entity>, pose: Transform) -> Entity {

        let link = commands.spawn((
            TransformBundle::from_transform(pose),
            PxArticulationLink{ pose }
        )).id();

        self.links.push((link, parent));
        
        return link;
    }

    
}



#[derive(Component)]
pub struct PxArticulationLink{
    pub pose: Transform,
}

#[derive(Component)]
pub struct PxArticulationRootTag;


pub fn new_articulation(
    mut commands: Commands,
    mut physx: ResMut<PhysX>,
    query: Query<(Entity, &PxArticulation), Added<PxArticulation>>,
    link_q: Query<&PxArticulationLink>,
) {

    for (e, articulation) in query.iter() {

        unsafe {

            let px_articulation = physx_sys::PxPhysics_createArticulationReducedCoordinate_mut(physx.foundation.physics_mut().as_mut_ptr());// as *mut physx_sys::PxArticulationBase;
 
            // physx_sys::PxArticulationReducedCoordinate_setArticulationFlag_mut(px_articulation, physx_sys::PxArticulationFlag::FixBase, true);


            let handle = physx.handles.articulations.insert(px_articulation);
            commands.entity(e).insert(PxArticulationHandle(handle));

            let mut map = std::collections::HashMap::new();

            for link in articulation.links.iter() {

                let link_comp = link_q.get(link.0).unwrap();

                let parent = match link.1 {
                    Some(parent_e) => *map.get(&parent_e).unwrap(),
                    None => {
                        commands.entity(link.0).insert(PxArticulationRootTag); //mark root link
                        std::ptr::null_mut()
                    },
                };


                let pose = trans_to_physx(link_comp.pose);

                //link
                let px_link = physx_sys::PxArticulationReducedCoordinate_createLink_mut(px_articulation, parent, pose.as_ptr());

                //handle
                let handle = physx.insert_rigid_actor(link.0, px_link as *mut PxRigidActor);
                commands.entity(link.0).insert(handle);

                map.insert(link.0, px_link);


                if parent.is_null() {continue;}

            }
                
        }

    }

}


pub fn add_articulation_system(
    mut physx: ResMut<PhysX>,
    query: Query<&PxArticulationHandle, Added<PxArticulationHandle>>,
) {

    for handle in query.iter() {

        unsafe {

            let px_articulation = *physx.handles.articulations.get(handle.0).unwrap();

            let filter_hint = physx_sys::phys_PxGetAggregateFilterHint(physx_sys::PxAggregateType::Generic, false);
            let px_aggregate = PxPhysics_createAggregate_mut(physx.foundation.physics_mut().as_mut_ptr() as *mut physx_sys::PxPhysics, 50, 50, filter_hint);
            physx_sys::PxAggregate_addArticulation_mut(px_aggregate, px_articulation);

            physx_sys::PxScene_addAggregate_mut(physx.scene.as_mut_ptr(), px_aggregate);
            // physx_sys::PxScene_addArticulation_mut(physx.scene.as_mut_ptr(), px_articulation.clone());
        }

    }

}

//todo add more joint types
pub enum PxJointAxis {
    X = 0,
    Y = 1,
    Z = 2,
}


#[derive(Default)]
pub enum PxJointType {
    #[default]
    Fixed,
    Spherical,
}


#[derive(Default, Clone, Copy)]
pub struct PxJointLimit {
    pub min: f32,
    pub max: f32,
}

impl PxJointLimit {
    pub fn new (min: f32, max: f32) -> Self {
        Self {min,max,}
    }
}


#[derive(Default, Clone, Copy)]
pub enum PxJointMotion {
    #[default]
    Locked,
    Free,
    Limited(PxJointLimit),
}

#[derive(Default, Clone, Copy)]
pub struct PxJointDrive {
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub force_limit: f32,
}


#[derive(Component, Default)]
pub struct PxArticulationJoint {
    pub joint_type: PxJointType,
    pub parent_pose: Transform,
    pub child_pose: Transform,
    pub motions: [PxJointMotion; 3],
    pub drives: [Option<PxJointDrive>; 3],
}




impl PxArticulationJoint {
    pub fn new(joint_type: PxJointType) -> Self {
        Self {
            joint_type,
            ..Default::default()
        }
    }

    pub fn parent_pose(mut self, parent_pose: Transform) -> Self {
        self.parent_pose = parent_pose;
        self
    }

    pub fn child_pose(mut self, child_pose: Transform) -> Self {
        self.child_pose = child_pose;
        self
    }

    pub fn motion(mut self, axis: PxJointAxis, motion: PxJointMotion) -> Self {
        self.motions[axis as usize] = motion;
        self
    }

    pub fn motions(mut self, motion: PxJointMotion) -> Self {
        self.motions = [motion; 3];
        self
    }

    pub fn drive(mut self, axis: PxJointAxis, drive: PxJointDrive) -> Self {
        self.drives[axis as usize] = Some(drive);
        self
    }

    pub fn drives(mut self, drive: PxJointDrive) -> Self {
        self.drives = [Some(drive); 3];
        self
    }

    pub fn set_drive_targets(&mut self, target: Vec3) -> &mut Self {

        for (i, drive) in self.drives.iter_mut().enumerate() {
            if let Some(drive) = drive {
                drive.target = target[i];
            }
        }

        self
    }

    pub fn set_drive_targets_quat(&mut self, target_rot: Quat) -> &mut Self {

        let (mut axis, angle) = target_rot.normalize().to_axis_angle(); //is normalize nessary?
        axis *= angle;

        for (i, drive) in self.drives.iter_mut().enumerate() {
            if let Some(drive) = drive {
                drive.target = axis[i];
            }
        }

        self
    }
}



pub fn new_articulation_joint(
    physx: ResMut<PhysX>,
    query: Query<(&PxRigidActorHandle, &PxArticulationJoint), (Added<PxArticulationJoint>, Without<PxArticulationRootTag>)>,
) {

    unsafe {

        for (handle, joint) in query.iter() {

            //get joint from link at set based on joint type
            let px_link = *physx.handles.rigid_actors.get(handle.0).unwrap();
            let px_joint = physx_sys::PxArticulationLink_getInboundJoint(px_link as *const PxArticulationLink_sys);

            //pose
            physx_sys::PxArticulationJointReducedCoordinate_setParentPose_mut(px_joint, trans_to_physx(joint.parent_pose).as_ptr());
            physx_sys::PxArticulationJointReducedCoordinate_setChildPose_mut(px_joint, trans_to_physx(joint.child_pose).as_ptr());

            //save
            let px_joint_reduced = px_joint as *mut physx::prelude::ArticulationJointReducedCoordinate;

            //type
            match joint.joint_type {
                PxJointType::Fixed => {(*px_joint_reduced).set_joint_type(ArticulationJointType::Fix);},
                PxJointType::Spherical => {(*px_joint_reduced).set_joint_type(ArticulationJointType::Spherical);},
            }

            //motion
            for (i, motion) in joint.motions.iter().enumerate() {
                match motion {
                    PxJointMotion::Locked => {(*px_joint_reduced).set_motion(to_axis(i), ArticulationMotion::Locked);},
                    PxJointMotion::Free => {(*px_joint_reduced).set_motion(to_axis(i), ArticulationMotion::Free);},
                    PxJointMotion::Limited(limit) => {
                        (*px_joint_reduced).set_motion(to_axis(i), ArticulationMotion::Limited);
                        (*px_joint_reduced).set_limit(to_axis(i), limit.min, limit.max);
                    },
                }
            }

            //drive
            for (i, drive) in joint.drives.iter().enumerate() {
                if let Some(drive) = drive {
                    (*px_joint_reduced).set_drive_target(drive.target, to_axis(i));
                    (*px_joint_reduced).set_drive(to_axis(i), drive.stiffness, drive.damping, drive.force_limit, ArticulationDriveType::Acceleration);
                }
            }

            
        }            


    }

}



pub fn update_articulation_joint_drive(
    physx: ResMut<PhysX>,
    query: Query<(&PxRigidActorHandle, &PxArticulationJoint), (Changed<PxArticulationJoint>, Without<PxArticulationRootTag>)>,
) {

    unsafe {

        for (handle, joint) in query.iter() {

            //get joint from link at set based on joint type
            let px_link = *physx.handles.rigid_actors.get(handle.0).unwrap();
            let px_joint = physx_sys::PxArticulationLink_getInboundJoint(px_link as *const PxArticulationLink_sys);

            let px_joint_reduced = px_joint as *mut physx::prelude::ArticulationJointReducedCoordinate;

            //drive
            for (i, drive) in joint.drives.iter().enumerate() {
                if let Some(drive) = drive {
                    (*px_joint_reduced).set_drive_target(drive.target, to_axis(i));
                    (*px_joint_reduced).set_drive(to_axis(i), drive.stiffness, drive.damping, drive.force_limit, ArticulationDriveType::Acceleration);
                }
            }

            
        }

    }

}



fn to_axis(index: usize) -> ArticulationAxis {
    match index {
        0 => ArticulationAxis::Twist,
        1 => ArticulationAxis::Swing1,
        2 => ArticulationAxis::Swing2,
        _ => panic!("Invalid index"),
    }
}

                
