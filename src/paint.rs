use CUBE_WIDTH;
use VOX_RADIUS;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::Point3;
use na::Translation3;
use palette::LinSrgba;
use std::error::Error;

pub fn paint(
    window: &mut Window,
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    pt: Point3<i32>,
    clr: LinSrgba<f32>,
) -> Result<(), Box<Error>> {
    // test xyz range for pixel and rgb range for intented color.
    if vec![pt.x, pt.y, pt.z]
        .iter()
        .all(|&idx| (0..CUBE_WIDTH).contains(idx))
    {
        // update the voxel. to account for brightness we really just want to destroy and recreate it anew.
        let (ix, iy, iz): (usize, usize, usize) = (pt.x as usize, pt.y as usize, pt.z as usize);
        window.remove(&mut voxels[ix][iy][iz]);

        // kiss3d doesn't support opacity, so we just scale down the sphere a bit.
        voxels[ix][iy][iz] = window.add_sphere(VOX_RADIUS * clr.alpha);

        // there's some nonsense here. Because drawing system coordinates hang from a
        // ceiling and are right-handed, where normal x/y/z coordinates rise from the
        // origin plane and are right-handed, we end up having to flip and rotate them
        // so that they match our intuition. TODO: represent this in a neat way with a
        // nalgebra transformation.
        voxels[ix][iy][iz].append_translation(&Translation3::new(
            1.0 * (pt.x as f32),
            1.0 * (pt.z as f32),
            (-1.0) * (pt.y as f32),
        ));

        voxels[ix][iy][iz].set_color(clr.red, clr.green, clr.blue);
    }
    Ok(())
}
