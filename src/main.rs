#![feature(slice_patterns)]
#![feature(match_default_bindings)]
#![feature(range_contains)]
#![feature(inclusive_range, inclusive_range_syntax)]

extern crate kiss3d;
extern crate mio;
extern crate nalgebra as na;
extern crate rosc;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
//use kiss3d::camera::Camera;
use mio::net::UdpSocket;
use mio::{Events, Poll, PollOpt, Ready, Token};
use na::{Point3, Translation3, Vector3};
use rosc::{OscMessage, OscPacket, OscType};
use std::error::Error;
//use std::f32::consts::PI;
use std::time::Duration;

static N: usize = 8;
static ROT: f32 = 0.001; // amount to rotate by
static VOX_WIDTH: f32 = 0.1; // voxel width

const TOKEN: Token = Token(1);

fn main() {
    run_sim().unwrap()
}

fn run_sim() -> Result<(), Box<Error>> {
    let socket = UdpSocket::bind(&"127.0.0.1:1234".parse()?)?;
    let poll = Poll::new()?;
    poll.register(&socket, TOKEN, Ready::readable(), PollOpt::edge())?;

    let mut events = Events::with_capacity(128);
    let mut buf = [0u8; rosc::decoder::MTU];

    let mut window = make_window();
    let mut voxels: Vec<Vec<Vec<SceneNode>>> = Vec::new();

    //let mut vox_colors: Vec<Vec<Vec<(f32, f32, f32)>>> = vec![vec![vec![(0.0, 0.0, 0.0); N]; N]; N];
    //let mut vox_refs: Vec<Vec<Vec<Option<SceneNode>>>> = vec![vec![vec![None; N]; N]; N];

    for i in 0..N {
        let mut plane: Vec<Vec<SceneNode>> = Vec::new();
        for j in 0..N {
            let mut row: Vec<SceneNode> = Vec::new();
            for k in 0..N {
                let mut vox = window.add_sphere(VOX_WIDTH);
                vox.set_color(0.0, 0.0, 0.0);
                vox.append_translation(&Translation3::new(i as f32, j as f32, k as f32));
                row.push(vox);
            }
            plane.push(row);
        }
        voxels.push(plane);
    }

    //cube
    //for (i, plane) in vox_colors.iter().enumerate() {
    //    for (j, line) in plane.iter().enumerate() {
    //        for (k, &(r, g, b)) in line.iter().enumerate() {
    //            let mut vox = window.add_sphere(VOX_WIDTH);
    //            vox.set_color(r, g, b);
    //            vox.append_translation(&Translation3::new(i as f32, j as f32, k as f32));
    //            vox_refs[i][j][k] = Some(vox);
    //        }
    //    }
    //}

    let mut cam = make_camera();

    Ok(while window.render_with_camera(&mut cam) {
        let curr = cam.yaw();
        cam.set_yaw(curr + ROT);
        poll.poll(&mut events, Some(Duration::from_millis(1)))?;
        for _event in events.iter() {
            match socket.recv_from(&mut buf) {
                Ok((size, _)) => match rosc::decoder::decode(&buf[..size]).unwrap() {
                    OscPacket::Message(OscMessage {
                        addr,
                        args: Some(args),
                    }) => {
                        mutate(&mut voxels, addr, &args).unwrap();
                    }
                    _ => (),
                },
                Err(_) => (),
            }
        }
    })
}

pub fn mutate(
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    addr: String,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    let (r, g, b): (f32, f32, f32) = match args.as_slice() {
        [OscType::Float(r), OscType::Float(g), OscType::Float(b)] => (*r, *g, *b),
        _ => return Ok(()),
    };
    let color_range = 0f32..=1f32;
    let idx_range = 0usize..N;
    match addr.split('/').skip(1).next() {
        Some("px") => {
            let coords = addr.split('/').skip(2).collect::<Vec<&str>>();
            let x = coords[0].parse::<usize>()?;
            let y = coords[1].parse::<usize>()?;
            let z = coords[2].parse::<usize>()?;
            if vec![x, y, z].iter().all(|&idx| idx_range.contains(idx))
                && vec![r, g, b].iter().all(|&col| color_range.contains(col))
            {
                voxels[x][y][z].set_color(r, g, b);
                //println!("{} {} {}\t{} {} {}", r, g, b, x, y, z);
            }
        }
        _ => (),
    }
    Ok(())

    //voxels[0][0][0].set_color(1.0, 1.0, 1.0);
}

pub fn make_window() -> Window {
    let mut window = Window::new_with_size(&format!("{}x{}x{}", N, N, N), 888, 888);
    window.set_light(Light::StickToCamera);
    window
}

fn make_camera() -> ArcBall {
    let eye_dist: f32 = 1.5 * (N as f32);
    let origin = Point3::new(N as f32 / 2.0, N as f32 / 2.0, N as f32 / 2.0);
    let eye = Vector3::new(eye_dist, eye_dist, eye_dist);
    ArcBall::new(origin + eye, origin)
}

// phone flat on its back = [ 0, 0,-1]
// phone tilted full fwd  = [ 1, 0, 0]
// phone tilted full back = [-1, 0, 0]
// phone tilted rightward = [ 0,-1, 0]
// phone tilted leftwards = [ 0, 1, 0]
