use CUBE_WIDTH;
use EYE_OFFSET;
use VOX_RADIUS;
use WINDOW_H;
use WINDOW_W;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::Translation3;
use na::{Point3, Vector3};

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
pub fn make_camera() -> ArcBall {
    let offset: f32 = EYE_OFFSET * (CUBE_WIDTH as f32);
    let origin = Point3::new(
        1.0 * CUBE_WIDTH as f32 / 2.0,
        1.0 * CUBE_WIDTH as f32 / 2.0,
        -1.0 * CUBE_WIDTH as f32 / 2.0,
    );
    ArcBall::new(origin + Vector3::new(offset, offset, offset), origin)
}

// creates CUBE_WIDTH x CUBE_WIDTH x CUBE_WIDTH array of voxels attached to window,
// and returns the 3d array of scenenodes for later mutation
pub fn make_cube_in_window(window: &mut Window) -> Vec<Vec<Vec<SceneNode>>> {
    let mut voxels = Vec::new();
    for i in 0..CUBE_WIDTH {
        voxels.push(Vec::new());
        for j in 0..CUBE_WIDTH {
            voxels[i as usize].push(Vec::new());
            for k in 0..CUBE_WIDTH {
                let mut vox = window.add_sphere(VOX_RADIUS);

                vox.append_translation(&Translation3::new(
                    1.0 * (i as f32),
                    1.0 * (k as f32),
                    (-1.0) * (j as f32),
                ));

                // default rainbow coloring
                vox.set_color(
                    (i as f32 / CUBE_WIDTH as f32),
                    (j as f32 / CUBE_WIDTH as f32),
                    (k as f32 / CUBE_WIDTH as f32),
                );

                voxels[i as usize][j as usize].push(vox);
            }
        }
    }
    voxels
}

pub fn make_axes(window: &mut Window) {
    let _ = window.draw_line(
        &Point3::origin(),
        &Point3::new(1.0, 0.0, 0.0),
        &Point3::new(1.0, 0.0, 0.0),
    );
    let _ = window.draw_line(
        &Point3::origin(),
        &Point3::new(0.0, 0.0, -1.0),
        &Point3::new(0.0, 1.0, 0.0),
    );
    let _ = window.draw_line(
        &Point3::origin(),
        &Point3::new(0.0, 1.0, 0.0),
        &Point3::new(0.0, 0.0, 1.0),
    );
}
