extern crate minifb;
extern crate openni2;
extern crate nite2;
use minifb::{ Window, Key, WindowOptions, Scale };
use std::{mem, process};
use openni2::OniDepthPixel;
use nite2::{Status, UserTracker};

pub fn depth_histogram(hist: &mut [f32], pixels: &[OniDepthPixel]) {
    let mut count = 0usize;
    for h in hist.iter_mut() {
        *h = 0f32;
    }

    for px in pixels {
        if *px != 0 {
            hist[*px as usize] += 1.0;
            count += 1;
        }
    }

    for i in 1..hist.len() {
        hist[i] += hist[i-1];
    }
    if count > 0 {
        for px in hist.iter_mut().skip(1) {
            *px = 256f32 * (1.0f32 - (*px / count as f32));
        }
    }
}

fn main() -> Result<(), Status> {
    openni2::init()?;
    nite2::init()?;

    let tracker = UserTracker::open_default()?;

    let mut window = match Window::new("NiTE2 Silhouette Viewer", 320, 240, WindowOptions {
        resize: false,
        scale: Scale::X2,
        ..Default::default()
    }) {
        Ok(window) => window,
        Err(_) => process::exit(1),
    };

    let user_colors: [u32; 6] = [
        0xFF0000,
        0x00FF00,
        0x0000FF,
        0xFFFF00,
        0xFF00FF,
        0x00FFFF,
    ];
    let mut buffer: [u32; 320 * 240] = unsafe { mem::zeroed() };
    let mut histogram: [f32; 10000] = unsafe { mem::zeroed() };
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let user_frame = tracker.read_frame().expect("Couldn't read user tracker frame");
        let depth_frame = user_frame.depth_frame();
        let depth_pixels = depth_frame.pixels();
        depth_histogram(&mut histogram, depth_pixels);
        let user_map = user_frame.user_map();
        assert_eq!(user_map.width, 320);
        assert_eq!(user_map.height, 240);
        for (i, (&user, &depth)) in user_map.pixels.iter().zip(depth_pixels).enumerate() {
            if user == 0 {
                buffer[i] = histogram[depth as usize] as u32;
            } else {
                buffer[i] = user_colors[user as usize % user_colors.len()];
            }
        }
        window.update_with_buffer(&buffer).expect("Couldn't write to minifb");
    }

    Ok(())
}
