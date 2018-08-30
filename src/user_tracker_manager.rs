use nite2_sys::NiteUserId;
use types::{Status};
use user_tracker::{UserTracker, UserData, UserTrackerFrame};

pub struct UserTrackerManager<'a> {
    user_tracker: UserTracker<'a>,
    users: Vec<UserData>,
    seen_user_ids: Vec<NiteUserId>,
    track_skeletons: bool,
}

impl<'a> UserTrackerManager<'a> {

    pub fn create() -> Result<UserTrackerManager<'a>, Status> {
        let manager = UserTrackerManager {
            user_tracker: UserTracker::open_default()?,
            users: Vec::with_capacity(10),
            seen_user_ids: Vec::with_capacity(10),
            track_skeletons: true,
        };
        Ok(manager)
    }

    pub fn track_skeletons(&mut self, enable: bool) -> Result<(), Status> {
        for id in &self.seen_user_ids {
            self.user_tracker.track_skeleton(*id, enable)?;
        }
        self.track_skeletons = enable;
        Ok(())
    }

    pub fn read_frame(&mut self) -> Result<UserTrackerFrame, Status> {
        let frame = self.user_tracker.read_frame()?;
        let users = frame.users();
        if self.track_skeletons {
            for user in &users {
                let user_id = user.id();
                if !self.users.iter().any(|u| u.id() == user_id) {
                    match self.user_tracker.track_skeleton(user_id, true) {
                        Ok(_) => println!("Tracking skeleton for user {}", user_id),
                        Err(e) => println!("Couldn't track skeleton for user {}: {}", user_id, e),
                    }
                }
            }
        }
        self.users = users;

        Ok(frame)
    }

    pub fn users(&self) -> &[UserData] {
        &self.users
    }
}
