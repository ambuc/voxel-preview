#![feature(inclusive_range, inclusive_range_syntax)]
#![feature(match_default_bindings)]
#![feature(range_contains)]
#![feature(slice_patterns)]
#![feature(underscore_lifetimes)]

extern crate kiss3d;
extern crate nalgebra as na;
extern crate rosc;
#[macro_use]
extern crate simple_error;
mod bresenham3d;
mod converters;
mod kiss_setup;
mod paint;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rosc::{OscMessage, OscPacket, OscType};
use std::env;
use std::error::Error;
use std::io;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;

// kiss3d constants
static WINDOW_W: u32 = 888; // arbitrary
static WINDOW_H: u32 = 888;
static ROTATION_RAD: f32 = 0.001; // amount by which to rotate the camera yaw per frame
static CUBE_WIDTH: usize = 8; // this will probably always be 8, since it should be a good
                              // simulation of a real 3d LED cube one could buy.
static VOX_RADIUS: f32 = 0.1; // radius of a voxel 'sphere'. this needs to be small,
                              // since kiss3d officially doesn't support transparency
static EYE_OFFSET: f32 = 1.5; // arbitrary
static POLL_TIMEOUT: u64 = 10; // polling is efficient enough to support this

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} CLIENT_IP:CLIENT_PORT", &args[0]);
    if args.len() < 2 {
        panic!(usage);
    }
    let addr = SocketAddrV4::from_str(&args[1]).unwrap();
    let socket = UdpSocket::bind(addr).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_millis(POLL_TIMEOUT)))
        .unwrap();
    let mut buf = [0u8; rosc::decoder::MTU];
    let mut window = kiss_setup::make_window();
    let mut voxels = kiss_setup::make_cube_in_window(&mut window); // initial blank slate
    let mut cam = kiss_setup::make_camera();

    loop {
        {
            let _ = window.render_with_camera(&mut cam);
            let curr = cam.yaw();
            cam.set_yaw(curr + ROTATION_RAD);
        }
        match socket.recv_from(&mut buf) {
            Ok((size, addr_from)) => {
                println!("Received packet with size {} from: {}", size, addr_from);
                match rosc::decoder::decode(&buf[..size]) {
                    Ok(OscPacket::Message(OscMessage {
                        addr,
                        args: Some(args),
                    })) => apply_instructions(&mut window, &mut voxels, addr, &args).unwrap(),
                    Err(e) => println!("Couldn't decode message: {:?}", e),
                    _ => (),
                };
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    println!("Error receiving from socket: {:?}", e);
                }
            }
        }
    }
}

// parses $args and applies the mutation to $voxels
fn apply_instructions(
    mut window: &mut Window,
    mut voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    addr: String,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    Ok(if addr == "/all" {
        paint::paint_all(&mut window, &mut voxels, &args)?;
    } else if addr == "/pt" {
        paint::paint_pt(&mut window, &mut voxels, &args)?;
    } else if addr == "/line" {
        paint::paint_line(&mut window, &mut voxels, &args)?;
    })
}

// TODO
// /plane ox oy oz v1x v1y v1z v2x v2y v2z rgb
// /plane ox oy oz v1x v1y v1z v2x v2y v2z rgb1 rbg2 cx cy cz
// /face  n x1 y1 z1 ... xn yn zn rgb
// /face  n x1 y1 z1 ... xn yn zn rgb2 rgb2 cx cy cz
// /cubo  ox oy oz v1x v1y v1z v2x v2y v2z v3x v3y v3z rgb
// /cubo  ox oy oz v1x v1y v1z v2x v2y v2z v3x v3y v3z rgb1 r2 g2 b2 cx cy cz
// /free  n x1 y1 z1 ... xn yn zn rgb
// /free  n x1 y1 z1 ... xn yn zn rgb1 rgb2 cx cy cz
