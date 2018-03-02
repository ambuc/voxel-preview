#![feature(inclusive_range, inclusive_range_syntax)]
#![feature(match_default_bindings)]
#![feature(range_contains)]
#![feature(slice_patterns)]

extern crate kiss3d;
extern crate mio;
extern crate nalgebra as na;
extern crate rosc;

use std::error::Error;
use std::time::Duration;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Point3, Translation3, Vector3};

use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use rosc::{OscMessage, OscPacket, OscType};

// kiss3d constants
static WINDOW_W: u32 = 888; // arbitrary
static WINDOW_H: u32 = 888;
static CUBE_WIDTH: usize = 8; // this will probably always be 8, since it should be a good
                              // simulation of a real 3d LED cube one could buy.
static ROTATION_RAD: f32 = 0.001; // amount by which to rotate the camera yaw per frame
static VOX_RADIUS: f32 = 0.1; // radius of a voxel 'sphere'. this needs to be small,
                              // since kiss3d officially doesn't support transparency
static EYE_OFFSET: f32 = 1.5; // arbitrary

// mio constants
static IP: &str = "127.0.0.1:1234"; // where 1234 is the default voxel-env port
static EVENTS_CAP: usize = 128;
static TOKEN: Token = Token(0);
static POLL_TIMEOUT: u64 = 1; // mio polling is efficient enough to support this

fn main() {
    run_sim().unwrap()
}

// creates the kiss3d scene, renders the voxel structure, and listens for UDP OSC updates on $IP
fn run_sim() -> Result<(), Box<Error>> {
    let socket = UdpSocket::bind(&IP.parse()?)?;
    let poll = Poll::new()?;
    poll.register(&socket, TOKEN, Ready::readable(), PollOpt::edge())?;
    let mut events = Events::with_capacity(EVENTS_CAP);
    let mut buf = [0u8; rosc::decoder::MTU];

    let mut window = make_window();
    let mut voxels = make_cube_in_window(&mut window); // initial blank slate
    let mut cam = make_camera();

    Ok(while window.render_with_camera(&mut cam) {
        {
            // tick yaw
            let curr = cam.yaw();
            cam.set_yaw(curr + ROTATION_RAD);
        }
        poll.poll(&mut events, Some(Duration::from_millis(POLL_TIMEOUT)))?;
        for _ in events.iter() {
            match socket.recv_from(&mut buf) {
                Ok((size, _)) => match rosc::decoder::decode(&buf[..size]).unwrap() {
                    OscPacket::Message(OscMessage {
                        addr,
                        args: Some(args),
                    }) => {
                        apply_instruction(&mut voxels, addr, &args).unwrap();
                    }
                    _ => (),
                },
                Err(_) => (),
            }
        }
    })
}

// utility function to accept [OscType::Float(r), OscType::Float(g), OscType::Float(b)]
// and return Some((r,g,b)). because we can't 'unwrap' enum types.
fn unwrap_args_as_rgb(args: &Vec<OscType>) -> Option<(f32, f32, f32)> {
    match args.as_slice() {
        [OscType::Float(r), OscType::Float(g), OscType::Float(b)] => Some((*r, *g, *b)),
        _ => None,
    }
}

// parses $args and applies the mutation to $voxels
fn apply_instruction(
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    addr: String,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    let color_range = 0f32..=1f32; // 0..1 incl
    let idx_range = 0usize..CUBE_WIDTH; // 0..8 excl

    // make r g b
    let (r, g, b): (f32, f32, f32) = match unwrap_args_as_rgb(args) {
        Some(rgb) => rgb,
        None => return Ok(()),
    };
    // make x y z
    let mut coords = addr.split('/').skip(1).map(|s| s.parse::<usize>());
    let x = coords.next().unwrap()?;
    let y = coords.next().unwrap()?;
    let z = coords.next().unwrap()?;

    // if xyz in 0..8 range (excl) and rgb in 0..1 range (incl)
    if vec![x, y, z].iter().all(|&idx| idx_range.contains(idx))
        && vec![r, g, b].iter().all(|&col| color_range.contains(col))
    {
        // update the voxel
        voxels[x][y][z].set_color(r, g, b);
    }
    Ok(())
}

// creates a window in userland with default lighting
pub fn make_window() -> Window {
    let mut window = Window::new_with_size(
        &format!("{}x{}x{}", CUBE_WIDTH, CUBE_WIDTH, CUBE_WIDTH),
        WINDOW_W,
        WINDOW_H,
    );
    window.set_light(Light::StickToCamera);
    window
}

// creates a camera fixed on the center of the voxel structure,  with some
fn make_camera() -> ArcBall {
    let offset: f32 = EYE_OFFSET * (CUBE_WIDTH as f32);
    let origin = Point3::new(
        CUBE_WIDTH as f32 / 2.0,
        CUBE_WIDTH as f32 / 2.0,
        CUBE_WIDTH as f32 / 2.0,
    );
    ArcBall::new(origin + Vector3::new(offset, offset, offset), origin)
}

// creates CUBE_WIDTH x CUBE_WIDTH x CUBE_WIDTH array of voxels attached to window,
// and returns the 3d array of scenenodes for later mutation
fn make_cube_in_window(window: &mut Window) -> Vec<Vec<Vec<SceneNode>>> {
    let mut voxels = Vec::new();
    for i in 0..CUBE_WIDTH {
        voxels.push(Vec::new());
        for j in 0..CUBE_WIDTH {
            voxels[i].push(Vec::new());
            for k in 0..CUBE_WIDTH {
                let mut vox = window.add_sphere(VOX_RADIUS);

                vox.append_translation(&Translation3::new(i as f32, j as f32, k as f32));

                // default rainbow coloring
                vox.set_color(
                    (i as f32 / CUBE_WIDTH as f32),
                    (j as f32 / CUBE_WIDTH as f32),
                    (k as f32 / CUBE_WIDTH as f32),
                );

                voxels[i][j].push(vox);
            }
        }
    }
    voxels
}
