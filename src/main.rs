extern crate kiss3d;
extern crate nalgebra as na;

use na::{Point3, Translation3};
use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use kiss3d::light::Light;

static VOX_WIDTH: f32 = 0.1; // voxel width
static VOX_DIST: f32 = 1.0; // voxel distance
static N: usize = 8;

fn main() {
    let mut window = Window::new_with_size("Kiss3d: cube", 1000, 1000);
    let mut voxels: Vec<Vec<Vec<kiss3d::scene::SceneNode>>> = Vec::new();

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

    let eye_dist: f32 = 1.5 * (N as f32);
    let eye = Point3::new(eye_dist, eye_dist, eye_dist);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let mut cam = ArcBall::new(eye, origin);

    while window.render_with_camera(&mut cam) {
        let curr = cam.yaw();
        cam.set_yaw(curr + 0.0014);
    }
}
