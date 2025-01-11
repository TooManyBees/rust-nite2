use openni2::Stream;
use nite2_sys::{NiteSkeleton, NiteSkeletonJoint, /* NiteQuaternion */};
use types::{JointType, Status};

#[derive(Clone, Copy, Debug)]
pub struct Skeleton(pub(crate) NiteSkeleton);

impl Skeleton {
    pub fn joint(&self, joint_type: JointType) -> &NiteSkeletonJoint {
        &self.0.joints[joint_type as usize]
    }

    pub fn joints(&self) -> &[NiteSkeletonJoint] {
        &self.0.joints
    }

    pub fn limbs(&self) -> Vec<(&NiteSkeletonJoint, &NiteSkeletonJoint)> {
        let head = self.joint(JointType::Head);
        let neck = self.joint(JointType::Neck);
        let left_shoulder = self.joint(JointType::LeftShoulder);
        let right_shoulder = self.joint(JointType::RightShoulder);
        let left_elbow = self.joint(JointType::LeftElbow);
        let right_elbow = self.joint(JointType::RightElbow);
        let left_hand = self.joint(JointType::LeftHand);
        let right_hand = self.joint(JointType::RightHand);
        let torso = self.joint(JointType::Torso);
        let left_hip = self.joint(JointType::LeftHip);
        let right_hip = self.joint(JointType::RightHip);
        let left_knee = self.joint(JointType::LeftKnee);
        let right_knee = self.joint(JointType::RightKnee);
        let left_foot = self.joint(JointType::LeftFoot);
        let right_foot = self.joint(JointType::RightFoot);

        [
            (head, neck),
            (left_shoulder, left_elbow),
            (left_elbow, left_hand),
            (right_shoulder, right_elbow),
            (right_elbow, right_hand),
            (left_shoulder, right_shoulder),
            (left_shoulder, torso),
            (right_shoulder, torso),
            (torso, left_hip),
            (torso, right_hip),
            (left_hip, right_hip),
            (left_hip, left_knee),
            (left_knee, left_foot),
            (right_hip, right_knee),
            (right_knee, right_foot),
        ].iter().filter_map(|&joints| {
            if joints.0.positionConfidence >= 1.0 && joints.1.positionConfidence >= 1.0 {
                Some(joints)
            } else {
                None
            }
        }).collect()
    }

    pub fn into_depth(self, depth_stream: &Stream) -> Result<Skeleton, Status> {
        let mut joints = self.0.joints;
        for joint in joints.iter_mut() {
            let (x, y, z) = depth_stream.world_to_depth((joint.position.x, joint.position.y, joint.position.z))?;
            joint.position.x = x;
            joint.position.y = y;
            joint.position.z = z;
        }
        Ok(Skeleton(NiteSkeleton {
            joints,
            ..self.0
        }))
    }

    pub fn into_world(self, depth_stream: &Stream) -> Result<Skeleton, Status> {
        let mut joints = self.0.joints;
        for joint in joints.iter_mut() {
            let (x, y, z) = depth_stream.depth_to_world((joint.position.x, joint.position.y, joint.position.z))?;
            joint.position.x = x;
            joint.position.y = y;
            joint.position.z = z;
        }
        Ok(Skeleton(NiteSkeleton {
            joints,
            ..self.0
        }))
    }
}
