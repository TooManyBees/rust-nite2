extern crate openni2;
extern crate nite2;
extern crate nite2_sys;

use openni2::Status;
use nite2::{UserTracker};

fn main() -> Result<(), Status> {
    openni2::init()?;
    nite2::init()?;

    let u = UserTracker::open_default()?;

    let f = u.read_frame()?;
    let d = f.depth_frame();

    println!("{:?}\n{:?}", f, d);

    Ok(())
}
