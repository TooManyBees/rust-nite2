extern crate openni2;
extern crate nite2;
extern crate piston_window;
extern crate image;

use std::{mem};
use piston_window::*;
use image::{ImageBuffer};
use openni2::OniDepthPixel;
use nite2::{Status, UserTrackerManager, DepthPoint};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const WHITE: [f32; 4] = [1., 1., 1., 1.];

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
            *px = 1.0f32 - (*px / count as f32);
        }
    }
}

struct Viewer {
    draw_skeleton: bool,
    draw_center_of_mass: bool,
    draw_status_label: bool,
    draw_bounding_box: bool,
    draw_background: bool,
    draw_depth: bool,
    draw_frame_id: bool,
}

fn main() -> Result<(), Status> {
    let mut viewer = Viewer {
        draw_skeleton: true,
        draw_center_of_mass: true,
        draw_status_label: true,
        draw_bounding_box: true,
        draw_background: true,
        draw_depth: true,
        draw_frame_id: true,
    };

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("NiTE2 Sample User Viewer", [WIDTH as u32, HEIGHT as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs = Glyphs::new("cour.ttf", window.factory.clone(), TextureSettings::new()).expect("font failed");
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

    let user_colors: [[f32; 3]; 6] = [
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        [1., 1., 0.],
        [1., 0., 1.],
        [0., 1., 1.],
    ];
    let mut histogram: [f32; 10000] = unsafe { mem::zeroed() };

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::S => viewer.draw_skeleton = !viewer.draw_skeleton,
                Key::L => viewer.draw_status_label = !viewer.draw_status_label,
                Key::C => viewer.draw_center_of_mass = !viewer.draw_center_of_mass,
                Key::X => viewer.draw_bounding_box = !viewer.draw_bounding_box,
                Key::B => viewer.draw_background = !viewer.draw_background,
                Key::D => viewer.draw_depth = !viewer.draw_depth,
                Key::F => viewer.draw_frame_id = !viewer.draw_frame_id,
                _ => {},
            }
        }
        if let Some(_) = e.render_args() {
            let user_frame = tracker.read_frame().expect("Couldn't read user tracker frame");

            let users = user_frame.users();

            let skeletons = users.iter()
                .filter_map(|user| user.skeleton().ok()?.into_depth(&depth_stream).ok());

            let depth_frame = user_frame.depth_frame();
            let depth_pixels = depth_frame.pixels();
            depth_histogram(&mut histogram, depth_pixels);
            let user_map = user_frame.user_map();
            assert_eq!(user_map.width, WIDTH);
            assert_eq!(user_map.height, HEIGHT);

            for ((&user, &depth), mut canvas_px) in user_map.pixels.iter().zip(depth_pixels).zip(canvas.pixels_mut()) {
                if user == 0 {
                    if viewer.draw_background {
                        let color = (histogram[depth as usize] * 256f32) as u8;
                        canvas_px.data = [color, color, color, 0xFF];
                    } else {
                        canvas_px.data = [0, 0, 0, 0xFF];
                    };
                } else {
                    let user_color = user_colors[(user as usize - 1) % user_colors.len()];
                    let multiplier = if viewer.draw_background { histogram[depth as usize] } else { 1. };
                    canvas_px.data = [
                        (user_color[0] * multiplier * 255.) as u8,
                        (user_color[1] * multiplier * 255.) as u8,
                        (user_color[2] * multiplier * 255.) as u8,
                        0xFF,
                    ];
                }
            }

            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                image(&texture, c.transform, g);

                if viewer.draw_skeleton {
                    for skeleton in skeletons {
                        for (j1, j2) in skeleton.limbs().into_iter() {
                            line(WHITE, 1.0, [j1.position.x as f64, j1.position.y as f64, j2.position.x as f64, j2.position.y as f64], c.transform, g);
                        }
                    }
                }
                for user in &users {
                    if viewer.draw_center_of_mass {
                        if let Ok(DepthPoint { x, y, .. }) = user.center_of_mass().into_depth(&depth_stream) {
                            ellipse(WHITE, [x as f64 - 4., y as f64 - 4., 8., 8.], c.transform, g);
                        }
                    }
                    if viewer.draw_bounding_box {
                        let (DepthPoint { x: xmin, y: ymin, .. }, DepthPoint { x: xmax, y: ymax, .. }) = user.bounding_box();
                        line(WHITE, 1.0, [xmin as f64, ymin as f64, xmax as f64, ymin as f64], c.transform, g);
                        line(WHITE, 1.0, [xmin as f64, ymax as f64, xmax as f64, ymax as f64], c.transform, g);
                        line(WHITE, 1.0, [xmin as f64, ymin as f64, xmin as f64, ymax as f64], c.transform, g);
                        line(WHITE, 1.0, [xmax as f64, ymin as f64, xmax as f64, ymax as f64], c.transform, g);
                    }
                    if viewer.draw_status_label {
                        // text(WHITE, 16, "string", &mut glyphs, c.transform.trans(0.0, 0.0), g);
                        // gotta keep track of user state in user manager (new, lost, calibrated, tracking, etc.)
                    }
                }

                if viewer.draw_frame_id {
                    let _ = text(WHITE, 14, &format!("{}", user_frame.frame_index()), &mut glyphs, c.transform.trans(20., 20.), g);
                }
            });
        }
    }


    Ok(())
}
