use CUBE_WIDTH;
use kiss3d::scene::SceneNode;
use na::Point3;
use palette::LinSrgba;
use palette::Blend;
use std::error::Error;

pub fn paint(
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    pt: Point3<i32>,
    clr_incoming: LinSrgba<f32>,
) -> Result<(), Box<Error>> {
    // test xyz range for pixel
    if !(0..CUBE_WIDTH).contains(pt.x) {
        Err(From::from(format!(
            "x coordinate {} not in range 0..{}",
            pt.x, CUBE_WIDTH
        )))
    } else if !(0..CUBE_WIDTH).contains(pt.y) {
        Err(From::from(format!(
            "y coordinate {} not in range 0..{}",
            pt.y, CUBE_WIDTH
        )))
    } else if !(0..CUBE_WIDTH).contains(pt.z) {
        Err(From::from(format!(
            "z coordinate {} not in range 0..{}",
            pt.z, CUBE_WIDTH
        )))
    } else {
        let vox = &mut voxels[pt.x as usize][pt.y as usize][pt.z as usize];

        let old_color_point: Point3<f32> = *vox.data().get_object().data().color();

        let clr_already =
            LinSrgba::new(old_color_point.x, old_color_point.y, old_color_point.z, 1.0);

        let clr = clr_already.overlay(clr_incoming);

        vox.set_color(clr.red, clr.green, clr.blue);

        Ok(())
    }
}
