use std::os::raw::{c_ulonglong, c_int, c_void};
use std::marker::PhantomData;
use std::{ptr, slice};
use skeleton::Skeleton;
use types::{Status, SkeletonState, WorldPoint, DepthPoint};
use openni2::{
    Frame,
    OniDepthPixel,
    frame_from_pointer as oni_frame_from_pointer,
};
use nite2_sys::*;

pub struct UserTracker<'a> {
    handle: NiteUserTrackerHandle,
    _device_lifetime: PhantomData<&'a ()>,
}

impl<'a> UserTracker<'a> {
    pub fn open_default() -> Result<UserTracker<'a>, Status> {
        let mut handle: NiteUserTrackerHandle = ptr::null_mut();
        let status = unsafe {
            niteInitializeUserTracker(&mut handle)
        }.into();
        match status {
            Status::Ok => Ok(UserTracker {
                handle,
                _device_lifetime: PhantomData,
            }),
            _ => Err(status),
        }
    }

    pub fn read_frame(&self) -> Result<UserTrackerFrame<'a>, Status> {
        let mut pointer = ptr::null_mut();
        let status = unsafe {
            niteReadUserTrackerFrame(self.handle, &mut pointer)
        }.into();
        match status {
            Status::Ok => Ok(frame_from_pointer(pointer, self.handle)),
            _ => Err(status),
        }
    }

    pub fn track_skeleton(&self, user: NiteUserId, setting: bool) -> Result<(), Status> {
        if setting {
            let status = unsafe { niteStartSkeletonTracking(self.handle, user) }.into();
            if let Status::Ok = status {
                Ok(())
            } else {
                Err(status)
            }
        } else {
            unsafe { niteStopSkeletonTracking(self.handle, user); }
            Ok(())
        }
    }

    pub fn tracking_skeleton(&self, user: NiteUserId) -> bool {
        unsafe { niteIsSkeletonTracking(self.handle, user) }
    }

    // niteSetSkeletonSmoothing
    // niteGetSkeletonSmoothing
    // niteStartPoseDetection
    // niteStopPoseDetection
    // niteStopAllPoseDetection

    pub fn register_next_frame_callback<F: FnMut(&UserTracker)>(&self, mut callback: F) -> Result<UserTrackerListener, Status> {
        extern "C" fn callback_wrapper(cookie: *mut c_void) {
            let closure: &mut dyn FnMut() = unsafe { &mut *(cookie as *mut Box<dyn FnMut()>) };
            closure();
        }

        let closure: Box<Box<dyn FnMut()>> = Box::new(Box::new(move || {
            callback(&self);
        }));

        let mut callback_struct = NiteUserTrackerCallbacks {
            readyForNextFrame: Some(callback_wrapper),
        };
        let status = unsafe {
            niteRegisterUserTrackerCallbacks(
                self.handle,
                &mut callback_struct,
                Box::into_raw(closure) as *mut _,
            )
        }.into();
        if let Status::Ok = status {
            Ok(UserTrackerListener {
                user_tracker_handle: &self.handle,
                callback_struct,
            })
        } else {
            Err(status)
        }
    }
}

impl<'a> Drop for UserTracker<'a> {
    fn drop(&mut self) {
        unsafe { niteShutdownUserTracker(self.handle); }
    }
}

#[derive(Debug)]
pub struct UserTrackerFrame<'a> {
    nite_frame: &'a NiteUserTrackerFrame,
    frame_pointer: *mut NiteUserTrackerFrame,
    user_tracker_handle: NiteUserTrackerHandle,
}

fn frame_from_pointer<'a>(frame_pointer: *mut NiteUserTrackerFrame, handle: NiteUserTrackerHandle) -> UserTrackerFrame<'a> {
    assert!(!frame_pointer.is_null(), "Creating UserTrackerFrame: *mut NiteUserTrackerFrame is null");
    let nite_frame: &NiteUserTrackerFrame = unsafe { &*frame_pointer };
    unsafe { niteUserTrackerFrameAddRef(handle, frame_pointer) };
    UserTrackerFrame {
        nite_frame,
        frame_pointer,
        user_tracker_handle: handle,
    }
}

impl<'a> UserTrackerFrame<'a> {
    pub fn timestamp(&self) -> c_ulonglong {
        self.nite_frame.timestamp
    }

    pub fn frame_index(&self) -> usize {
        self.nite_frame.frameIndex as usize
    }

    pub fn floor_confidence(&self) -> f32 {
        self.nite_frame.floorConfidence
    }

    pub fn floor(&self) -> NitePlane {
        self.nite_frame.floor
    }

    pub fn depth_frame(&self) -> Frame<OniDepthPixel> {
        unsafe { oni_frame_from_pointer(self.nite_frame.pDepthFrame) }
    }

    pub fn user_map(&self) -> UserMap {
        let pixels = self.nite_frame.userMap.pixels;
        assert!(!pixels.is_null(), "Creating UserMap: NiteUserMap.pixels is null");
        let width = self.nite_frame.userMap.width as usize;
        let height = self.nite_frame.userMap.height as usize;
        let stride = self.nite_frame.userMap.stride as usize;
        UserMap {
            pixels: unsafe { slice::from_raw_parts(pixels, width * height) },
            width,
            height,
            stride,
        }
    }

    pub fn users(&self) -> Vec<UserData> {
        if self.nite_frame.userCount == 0 {
            vec![]
        } else {
            assert!(!self.nite_frame.pUser.is_null(), "Creating NiteUserData slice: NiteUserTrackerFrame.pUser is null");
            unsafe { slice::from_raw_parts(self.nite_frame.pUser, self.user_count()) }.into_iter().map(|&user_data| {
                UserData(user_data)
            }).collect()
        }
    }

    #[inline]
    pub fn user_count(&self) -> usize {
        self.nite_frame.userCount as usize
    }
}

impl<'a> Drop for UserTrackerFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            niteUserTrackerFrameRelease(self.user_tracker_handle, self.frame_pointer)
        };
    }
}

pub struct UserMap<'a> {
    pub pixels: &'a [NiteUserId],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

pub struct UserTrackerListener<'tracker> {
    user_tracker_handle: &'tracker NiteUserTrackerHandle,
    callback_struct: NiteUserTrackerCallbacks,
}

impl<'tracker> Drop for UserTrackerListener<'tracker> {
    fn drop(&mut self) {
        unsafe {
            niteUnregisterUserTrackerCallbacks(*self.user_tracker_handle, &mut self.callback_struct);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserData(NiteUserData);

impl UserData {
    pub fn id(&self) -> NiteUserId {
        self.0.id
    }

    // TODO: used?
    pub fn state(&self) -> c_int {
        self.0.state
    }

    pub fn skeleton(&self) -> Result<Skeleton, SkeletonState> {
        match self.0.skeleton.state.into() {
            SkeletonState::Tracked => Ok(Skeleton(self.0.skeleton)),
            state => Err(state),
        }
    }

    pub fn center_of_mass(&self) -> WorldPoint {
        self.0.centerOfMass.into()
    }

    pub fn bounding_box(&self) -> (DepthPoint, DepthPoint) {
        (self.0.boundingBox.min.into(), self.0.boundingBox.max.into())
    }

    // pub fn poses
}
