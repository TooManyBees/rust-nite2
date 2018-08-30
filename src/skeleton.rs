use nite2_sys::{NiteSkeleton, NiteSkeletonJoint};
use types::{JointType};

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
        ].into_iter().filter_map(|&joints| {
            if joints.0.positionConfidence >= 1.0 && joints.1.positionConfidence >= 1.0 {
                Some(joints)
            } else {
                None
            }
        }).collect()
    }

    // TODO: gtfo of here, joints_mut....
    // the only reason this exists is to change position from world to depth;
    // we need a method on joint to do this instead
    pub fn joints_mut(&mut self) -> &mut [NiteSkeletonJoint] {
        &mut self.0.joints
    }
}
