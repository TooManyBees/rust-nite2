extern crate openni2;
extern crate nite2;
extern crate piston_window;
extern crate image;

use std::{mem};
use piston_window::*;
use image::{ImageBuffer};
use openni2::OniDepthPixel;
use nite2::{Status, UserTrackerManager};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn depth_histogram(hist: &mut [f32], pixels: &[OniDepthPixel]) {
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

// struct UserViewer {
//     users: Vec<User>,

// }

fn main() -> Result<(), Status> {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("NiTE2 Sample User Viewer", [WIDTH as u32, HEIGHT as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let mut texture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    openni2::init()?;
    let default_device = openni2::Device::open_default()?;
    let depth_stream = default_device.create_stream(openni2::SensorType::DEPTH)?;
    nite2::init()?;

    let mut tracker = UserTrackerManager::create()?;

    let user_colors: [[u8; 4]; 6] = [
        [0xFF, 0x00, 0x00, 0xFF],
        [0x00, 0xFF, 0x00, 0xFF],
        [0x00, 0x00, 0xFF, 0xFF],
        [0xFF, 0xFF, 0x00, 0xFF],
        [0xFF, 0x00, 0xFF, 0xFF],
        [0x00, 0xFF, 0xFF, 0xFF],
    ];
    let mut histogram: [f32; 10000] = unsafe { mem::zeroed() };

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            let user_frame = tracker.read_frame().expect("Couldn't read user tracker frame");

            let skeletons = user_frame.users()
                .into_iter()
                .filter_map(|user| user.skeleton().ok().map(|mut skeleton| {
                    for mut joint in skeleton.joints_mut().iter_mut() {
                        let (x, y, z) = depth_stream.world_to_depth((joint.position.x, joint.position.y,  joint.position.z)).expect("Couldn't translate depth to world coordinates!");
                        joint.position.x = x;
                        joint.position.y = y;
                        joint.position.z = z;
                    }
                    skeleton 
                }));
                // .fold(Vec::with_capacity(user_frame.user_count()), |mut acc, user| {
                //     if let Ok(mut skeleton) = user.skeleton() {
                //         for mut joint in skeleton.joints_mut().iter_mut() {
                //             let (x, y, z) = depth_stream.world_to_depth((joint.position.x, joint.position.y,  joint.position.z)).expect("Couldn't translate depth to world coordinates!");
                //             joint.position.x = x;
                //             joint.position.y = y;
                //             joint.position.z = z;
                //         }
                //         acc.push(skeleton);
                //     }
                //     acc
                // });

            let depth_frame = user_frame.depth_frame();
            let depth_pixels = depth_frame.pixels();
            depth_histogram(&mut histogram, depth_pixels);
            let user_map = user_frame.user_map();
            assert_eq!(user_map.width, WIDTH);
            assert_eq!(user_map.height, HEIGHT);
            for ((&user, &depth), mut canvas_px) in user_map.pixels.iter().zip(depth_pixels).zip(canvas.pixels_mut()) {
                if user == 0 {
                    let color = histogram[depth as usize] as u8;
                    canvas_px.data = [color, color, color, 0xFF];
                } else {
                    let color = user_colors[(user as usize - 1) % user_colors.len()];
                    canvas_px.data = color;
                }
            }

            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                image(&texture, c.transform, g);
                for skeleton in skeletons {
                    for (j1, j2) in skeleton.limbs().into_iter() {
                        line([1.0, 1.0, 1.0, 1.0], 1.0, [j1.position.x as f64, j1.position.y as f64, j2.position.x as f64, j2.position.y as f64], c.transform, g);
                    }
                }
            });
        }
    }


    Ok(())
}
