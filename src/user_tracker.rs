use std::os::raw::c_ulonglong;
use std::marker::PhantomData;
use std::{ptr, slice};
use types::Status;
use openni2::{
    Device,
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

    pub fn open_with_device(device: &'a Device) -> Result<UserTracker<'a>, Status> {
        let mut handle: NiteUserTrackerHandle = ptr::null_mut();
        let status = unsafe {
            niteInitializeUserTrackerByDevice(device.handle(), &mut handle)
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
            Status::Ok => Ok(frame_from_pointer(pointer)),
            _ => Err(status),
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
}

fn frame_from_pointer<'a>(ptr: *mut NiteUserTrackerFrame) -> UserTrackerFrame<'a> {
    assert!(!ptr.is_null(), "Creating UserTrackerFrame: *mut NiteUserTrackerFrame is null");
    let nite_frame: &NiteUserTrackerFrame = unsafe { &*ptr };
    UserTrackerFrame {
        nite_frame
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
        assert!(!pixels.is_null() "Creating UserMap: NiteUserMap.pixels is null");
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

    pub fn users(&self) -> &'a [NiteUserData] {
        assert!(!self.nite_frame.pUser.is_null(), "Creating NiteUserData slice: NiteUserTrackerFrame.pUser is null");
        unsafe {
            slice::from_raw_parts(self.nite_frame.pUser, self.user_count())
        }
    }

    pub fn user_count(&self) -> usize {
        self.nite_frame.userCount as usize
    }
}

pub struct UserMap<'a> {
    pub pixels: &'a [NiteUserId],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}
