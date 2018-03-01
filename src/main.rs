#![feature(slice_patterns)]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate rosc;

static VOX_WIDTH: f32 = 0.1; // voxel width
static VOX_DIST: f32 = 1.0; // voxel distance
static N: usize = 8;

// rosc
use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use rosc::{OscMessage, OscPacket, OscType};
use std::f32::consts::PI;
// kiss3d
use na::{Point3, Translation3};
use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

fn main() {
    let addr = match SocketAddrV4::from_str("192.168.0.105:1234") {
        Ok(addr) => addr,
        Err(_) => panic!("oh no"),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    let mut buf = [0u8; rosc::decoder::MTU];

    let eye_dist: f32 = 1.5 * (N as f32);
    let eye = Point3::new(eye_dist, eye_dist, eye_dist);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let mut cam = ArcBall::new(eye, origin);
    let mut window = make_window();

    while window.render_with_camera(&mut cam) {
        let curr = cam.yaw();
        cam.set_yaw(curr + 0.0005);
        println!("Listening to {}", addr);
        println!("{:?}", sock);
        // match sock.recv_from(&mut buf) {
        //     Ok((size, _)) => {
        //         let packet = rosc::decoder::decode(&buf[..size]).unwrap();
        //         match packet {
        //             OscPacket::Message(OscMessage { addr, args }) => {
        //                 println!("{:?}\t{:?}", addr, args);
        //                 // phone flat on its back = [ 0, 0,-1]
        //                 // phone tilted full fwd  = [ 1, 0, 0]
        //                 // phone tilted full back = [-1, 0, 0]
        //                 // phone tilted rightward = [ 0,-1, 0]
        //                 // phone tilted leftwards = [ 0, 1, 0]
        //             }
        //             _ => (),
        //         }
        //     }
        //     Err(e) => (),
        // }
    }
}

pub fn make_window() -> Window {
    let mut window = Window::new_with_size("Kiss3d: cube", 888, 888);
    let mut voxels: Vec<Vec<Vec<SceneNode>>> = Vec::new();

    //         c          //      c
    //   0   1 : 2   3    //  0   1   2
    //   |   | : |   |    //  |   |   |
    //         :          //      :
    //   <-----> offset   //  <---> offset
    let offset: f32 = 0.0 - ((((N as f32) - 1.0) * VOX_DIST) / 2.0);

    let mut i_coord = offset; //init i_coord
    for i_idx in 0..N {
        let mut i_row = Vec::new();
        let mut j_coord = offset; // init j_coord
        for j_idx in 0..N {
            let mut j_row = Vec::new();
            let mut k_coord = offset; // init k_coord
            for k_idx in 0..N {
                let mut vox = window.add_sphere(VOX_WIDTH);
                vox.set_color(
                    1.0 / (N as f32) * (i_idx as f32),
                    1.0 / (N as f32) * (j_idx as f32),
                    1.0 / (N as f32) * (k_idx as f32),
                );
                vox.append_translation(&Translation3::new(i_coord, j_coord, k_coord));
                j_row.push(vox);
                k_coord += VOX_DIST;
            }
            i_row.push(j_row);
            j_coord += VOX_DIST;
        }
        voxels.push(i_row);
        i_coord += VOX_DIST;
    }

    window.set_light(Light::StickToCamera);
    window
}
