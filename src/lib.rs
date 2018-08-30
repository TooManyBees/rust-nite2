extern crate nite2_sys;
extern crate openni2;

mod types;
mod user_tracker;
mod user_tracker_manager;
mod skeleton;

use nite2_sys::*;
pub use nite2_sys::{
    NiteVersion,
    NitePlane,
    NiteBoundingBox,
    NitePoint3f,
    NiteQuaternion,
    NiteUserId,
};

pub use types::{
    Status,
    JointType,
    SkeletonState,
    PoseType,
    GestureType,
};

pub use user_tracker::{
    UserTracker,
    UserTrackerFrame,
    UserMap,
};

pub use user_tracker_manager::UserTrackerManager;

pub fn init() -> Result<(), Status> {
    let status = unsafe { niteInitialize() }.into();
    match status {
        Status::Ok => Ok(()),
        _ => Err(status),
    }
}

pub fn shutdown() {
    unsafe { niteShutdown(); }
}

pub fn version() -> NiteVersion {
    unsafe { niteGetVersion() }
}
