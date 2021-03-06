use std::fmt;
use nite2_sys::*;
use openni2::{Stream, Status as OpenNI2Status};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Status {
    Ok,
    Error(String),
    BadUserId,
    OutOfFlow,
}

impl Status {
    fn from_int(value: NiteStatus) -> Self {
        match value {
            NITE_STATUS_OK => Status::Ok,
            NITE_STATUS_ERROR => Status::Error(String::from("Generic NiTE error")),
            NITE_STATUS_BAD_USER_ID => Status::BadUserId,
            NITE_STATUS_OUT_OF_FLOW => Status::OutOfFlow,
            _ => panic!("Unknown NiTE status {}", value),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_string = match self {
            Status::Ok => "Ok",
            Status::Error(s) => &s,
            Status::BadUserId => "Bad user id",
            Status::OutOfFlow => "Out of flow",
        };
        write!(f, "NiTE2 error: {}", error_string)
    }
}

impl From<NiteStatus> for Status {
    fn from(i: NiteStatus) -> Self {
        Status::from_int(i)
    }
}

impl From<Status> for OpenNI2Status {
    fn from(status: Status) -> OpenNI2Status {
        match status {
            Status::Ok => OpenNI2Status::Ok,
            Status::Error(s) => OpenNI2Status::Error(s),
            Status::BadUserId => OpenNI2Status::BadParameter,
            Status::OutOfFlow => OpenNI2Status::OutOfFlow,
        }
    }
}

impl From<OpenNI2Status> for Status {
    fn from(status: OpenNI2Status) -> Status {
        match status {
            OpenNI2Status::Ok => Status::Ok,
            OpenNI2Status::OutOfFlow => Status::OutOfFlow,
            e @ _ => Status::Error(format!("OpenNI2 error: {}", e))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum JointType {
    Head = NITE_JOINT_HEAD,
    Neck = NITE_JOINT_NECK,
    LeftShoulder = NITE_JOINT_LEFT_SHOULDER,
    RightShoulder = NITE_JOINT_RIGHT_SHOULDER,
    LeftElbow = NITE_JOINT_LEFT_ELBOW,
    RightElbow = NITE_JOINT_RIGHT_ELBOW,
    LeftHand = NITE_JOINT_LEFT_HAND,
    RightHand = NITE_JOINT_RIGHT_HAND,
    Torso = NITE_JOINT_TORSO,
    LeftHip = NITE_JOINT_LEFT_HIP,
    RightHip = NITE_JOINT_RIGHT_HIP,
    LeftKnee = NITE_JOINT_LEFT_KNEE,
    RightKnee = NITE_JOINT_RIGHT_KNEE,
    LeftFoot = NITE_JOINT_LEFT_FOOT,
    RightFoot = NITE_JOINT_RIGHT_FOOT,
}

impl JointType {
    pub fn from_int(i: NiteJointType) -> JointType {
        match i {
            NITE_JOINT_HEAD => JointType::Head,
            NITE_JOINT_NECK => JointType::Neck,
            NITE_JOINT_LEFT_SHOULDER => JointType::LeftShoulder,
            NITE_JOINT_RIGHT_SHOULDER => JointType::RightShoulder,
            NITE_JOINT_LEFT_ELBOW => JointType::LeftElbow,
            NITE_JOINT_RIGHT_ELBOW => JointType::RightElbow,
            NITE_JOINT_LEFT_HAND => JointType::LeftHand,
            NITE_JOINT_RIGHT_HAND => JointType::RightHand,
            NITE_JOINT_TORSO => JointType::Torso,
            NITE_JOINT_LEFT_HIP => JointType::LeftHip,
            NITE_JOINT_RIGHT_HIP => JointType::RightHip,
            NITE_JOINT_LEFT_KNEE => JointType::LeftKnee,
            NITE_JOINT_RIGHT_KNEE => JointType::RightKnee,
            NITE_JOINT_LEFT_FOOT => JointType::LeftFoot,
            NITE_JOINT_RIGHT_FOOT => JointType::RightFoot,
            _ => panic!("Unknown joint type {}", i),
        }
    }
}

impl From<NiteJointType> for JointType {
    fn from(i: NiteJointType) -> JointType {
        JointType::from_int(i)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum SkeletonState {
    None = NITE_SKELETON_NONE,
    Calibrating = NITE_SKELETON_CALIBRATING,
    Tracked = NITE_SKELETON_TRACKED,
    CalibrationErrorNotInPose = NITE_SKELETON_CALIBRATION_ERROR_NOT_IN_POSE,
    CalibrationErrorHands = NITE_SKELETON_CALIBRATION_ERROR_HANDS,
    CalibrationErrorHead = NITE_SKELETON_CALIBRATION_ERROR_HEAD,
    CalibrationErrorLegs = NITE_SKELETON_CALIBRATION_ERROR_LEGS,
    CalibrationErrorTorso = NITE_SKELETON_CALIBRATION_ERROR_TORSO,
}

impl SkeletonState {
    pub fn from_int(i: NiteSkeletonState) -> SkeletonState {
        match i {
            NITE_SKELETON_NONE => SkeletonState::None,
            NITE_SKELETON_CALIBRATING => SkeletonState::Calibrating,
            NITE_SKELETON_TRACKED => SkeletonState::Tracked,
            NITE_SKELETON_CALIBRATION_ERROR_NOT_IN_POSE => SkeletonState::CalibrationErrorNotInPose,
            NITE_SKELETON_CALIBRATION_ERROR_HANDS => SkeletonState::CalibrationErrorHands,
            NITE_SKELETON_CALIBRATION_ERROR_HEAD => SkeletonState::CalibrationErrorHead,
            NITE_SKELETON_CALIBRATION_ERROR_LEGS => SkeletonState::CalibrationErrorLegs,
            NITE_SKELETON_CALIBRATION_ERROR_TORSO => SkeletonState::CalibrationErrorTorso,
            _ => panic!("Unknown skeleton state {}", i),
        }
    }
}

impl From<NiteSkeletonState> for SkeletonState {
    fn from(i: NiteSkeletonState) -> SkeletonState {
        SkeletonState::from_int(i)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PoseType {
    Psi = NITE_POSE_PSI,
    CrossedHands = NITE_POSE_CROSSED_HANDS,
}

impl PoseType {
    pub fn from_int(i: NitePoseType) -> PoseType {
        match i {
            NITE_POSE_PSI => PoseType::Psi,
            NITE_POSE_CROSSED_HANDS => PoseType::CrossedHands,
            _ => panic!("Unknown pose type {}", i),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum GestureType {
    Wave = NITE_GESTURE_WAVE,
    Click = NITE_GESTURE_CLICK,
    HandRaise = NITE_GESTURE_HAND_RAISE,
}

impl GestureType {
    pub fn from_int(i: NiteGestureType) -> GestureType {
        match i {
            NITE_GESTURE_WAVE => GestureType::Wave,
            NITE_GESTURE_CLICK => GestureType::Click,
            NITE_GESTURE_HAND_RAISE => GestureType::HandRaise,
            _ => panic!("Unknown gesture type {}", i),
        }
    }
}

impl From<NiteGestureType> for GestureType {
    fn from(i: NiteGestureType) -> GestureType {
        GestureType::from_int(i)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPoint {
    pub fn into_depth(self, stream: &Stream) -> Result<DepthPoint, Status> {
        let (x, y, z) = stream.world_to_depth((self.x, self.y, self.z))?;
        Ok(DepthPoint { x, y, z })
    }
}

impl Into<WorldPoint> for NitePoint3f {
    fn into(self) -> WorldPoint {
        WorldPoint { x: self.x, y: self.y, z: self.z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DepthPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl DepthPoint {
    pub fn into_world(self, stream: &Stream) -> Result<WorldPoint, Status> {
        let (x, y, z) = stream.depth_to_world((self.x, self.y, self.z))?;
        Ok(WorldPoint { x, y, z })
    }
}

impl Into<DepthPoint> for NitePoint3f {
    fn into(self) -> DepthPoint {
        DepthPoint { x: self.x, y: self.y, z: self.z }
    }
}
