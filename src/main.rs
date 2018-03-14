#![feature(inclusive_range, inclusive_range_syntax)]
#![feature(match_default_bindings)]
#![feature(range_contains)]
#![feature(slice_patterns)]
#![feature(underscore_lifetimes)]

extern crate kiss3d;
extern crate nalgebra as na;
extern crate palette;
extern crate rosc;
extern crate simple_error;
mod bresenham3d;
mod geometry;
mod kiss_setup;
mod paint;
mod readers;

use na::Point3;
use palette::LinSrgba;
use palette::gradient::Gradient;
use rosc::{OscMessage, OscPacket, OscType};
use std::env;
use std::error::Error;
use std::io;
use std::net::{SocketAddrV4, UdpSocket};
use std::slice;
use std::str::FromStr;
use std::time::Duration;

// kiss3d constants
static WINDOW_W: u32 = 888; // arbitrary
static WINDOW_H: u32 = 888;
static ROTATION_RAD: f32 = 0.001; // amount by which to rotate the camera yaw per frame
                                  //static ROTATION_RAD: f32 = 0.0; // amount by which to rotate the camera yaw per frame
static CUBE_WIDTH: i32 = 8; // this will probably always be 8, since it should be a good
                            // simulation of a real 3d LED cube one could buy.
static VOX_RADIUS: f32 = 0.05; // radius of a voxel 'sphere'. this needs to be small,
                               // since kiss3d officially doesn't support transparency
static EYE_OFFSET: f32 = 1.5; // arbitrary
static POLL_TIMEOUT: u64 = 10; // polling is efficient enough to support this

type Shape = Vec<Point3<i32>>;
type Shader = Fn(Point3<i32>) -> LinSrgba<f32>;

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

        kiss_setup::make_axes(&mut window);

        match socket.recv_from(&mut buf) {
            Ok((size, addr_from)) => {
                println!("Received packet with size {} from: {}", size, addr_from);
                match rosc::decoder::decode(&buf[..size]) {
                    Ok(OscPacket::Message(OscMessage {
                        addr,
                        args: Some(args),
                    })) => {
                        println!("{:?}\t{:?}", addr, args);
                        let (shape, shader): (Shape, Box<Shader>) =
                            match get_shape_and_shader(addr, args) {
                                Ok(load) => load,
                                Err(e) => {
                                    println!("{:?}", e);
                                    continue;
                                }
                            };
                        for cell in shape {
                            let _ = paint::paint(&mut window, &mut voxels, cell, shader(cell));
                        }
                    }
                    Ok(msg) => println!("Recieved other message: {:?}", msg),
                    Err(e) => println!("Couldn't decode message: {:?}", e),
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
fn get_shape_and_shader(
    addr: String,
    args: Vec<OscType>,
) -> Result<(Shape, Box<Shader>), Box<Error>> {
    let mut it: slice::Iter<'_, OscType> = args.iter();

    Ok(match addr.as_ref() {
        "/dsc/voxel" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = vec![pt];
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/line" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let dir = readers::dsc_vector_3(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::discrete_line(pt, dir);
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/line/grad" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let dir = readers::dsc_vector_3(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shader = move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            };

            (geometry::discrete_line(pt, dir), Box::new(shader))
        }

        "/dsc/plane" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shader = move |_cell: Point3<i32>| clr;

            (geometry::discrete_plane(pt, vec1, vec2), Box::new(shader))
        }

        "/dsc/plane/grad" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shader = move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            };

            (geometry::discrete_plane(pt, vec1, vec2), Box::new(shader))
        }

        "/dsc/frame" => {
            let anchor = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::discrete_frame(anchor, vec1, vec2);
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/frame/grad" => {
            let anchor = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shape = geometry::discrete_frame(anchor, vec1, vec2);
            let shader = Box::new(move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            });

            (shape, shader)
        }

        "/dsc/cuboid" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let vec3 = readers::dsc_vector_3(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::discrete_cuboid(pt, vec1, vec2, vec3);
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/cuboid/grad" => {
            let pt = readers::dsc_point_3(&mut it)?;
            let vec1 = readers::dsc_vector_3(&mut it)?;
            let vec2 = readers::dsc_vector_3(&mut it)?;
            let vec3 = readers::dsc_vector_3(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shape = geometry::discrete_cuboid(pt, vec1, vec2, vec3);
            let shader = Box::new(move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            });

            (shape, shader)
        }

        "/dsc/sphere" => {
            let center = readers::dsc_point_3(&mut it)?;
            let p = readers::int(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::discrete_sphere(center, p);
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/sphere/grad" => {
            let center = readers::dsc_point_3(&mut it)?;
            let p = readers::int(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shape = geometry::discrete_sphere(center, p);
            let shader = Box::new(move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            });

            (shape, shader)
        }

        "/dsc/shell" => {
            let center = readers::dsc_point_3(&mut it)?;
            let p = readers::int(&mut it)?;
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::discrete_shell(center, p);
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/dsc/shell/grad" => {
            let center = readers::dsc_point_3(&mut it)?;
            let p = readers::int(&mut it)?;
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shape = geometry::discrete_shell(center, p);
            let shader = Box::new(move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            });

            (shape, shader)
        }

        "/fill/solid" => {
            let clr = readers::lin_srgba(&mut it)?;

            let shape = geometry::all_cells();
            let shader = Box::new(move |_cell: Point3<i32>| clr);

            (shape, shader)
        }

        "/fill/solid/grad" => {
            let clr1 = readers::lin_srgba(&mut it)?;
            let clr2 = readers::lin_srgba(&mut it)?;
            let clr_pt = readers::dsc_point_3(&mut it)?;
            let clr_dir = readers::dsc_vector_3(&mut it)?;

            let shape = geometry::all_cells();
            let shader = Box::new(move |cell: Point3<i32>| {
                let proj: f32 = (cell - clr_pt).dot(&clr_dir) as f32 / clr_dir.dot(&clr_dir) as f32;
                let grad = Gradient::new(vec![clr1, clr2]);
                grad.get(proj)
            });

            (shape, shader)
        }

        _ => {
            return Err(From::from(format!(
                "no match for addr {:?} args {:?}",
                addr, args
            )));
        }
    })
}
