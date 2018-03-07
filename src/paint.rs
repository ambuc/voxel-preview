static COLOR_RANGE_L: f32 = 0f32;
static COLOR_RANGE_R: f32 = 1f32; // 0..1 incl
static IDX_RANGE_L: usize = 0usize;
use CUBE_WIDTH;
use VOX_RADIUS;
use bresenham3d;
use converters;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rosc::OscType;
use std::error::Error;

pub fn paint(
    window: &mut Window,
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    x: usize,
    y: usize,
    z: usize,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> Result<(), Box<Error>> {
    // test xyz range for pixel and rgb range for intented color.
    if vec![x, y, z]
        .iter()
        .all(|&idx| (IDX_RANGE_L..CUBE_WIDTH).contains(idx))
        && vec![r, g, b]
            .iter()
            .all(|&col| (COLOR_RANGE_L..=COLOR_RANGE_R).contains(col))
    {
        // update the voxel. to account for brightness we really just want to destroy and recreate it anew.
        window.remove(&mut voxels[x][y][z]);
        // kiss3d doesn't support opacity, so we just scale down the sphere a bit.
        voxels[x][y][z] = window.add_sphere(VOX_RADIUS * a);
        voxels[x][y][z].append_translation(&converters::translation_for(x, y, z));
        voxels[x][y][z].set_color(r, g, b);
    }

    Ok(())
}

// /all rgb
pub fn paint_all(
    mut window: &mut Window,
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    let mut it = args.iter();

    let (r, g, b, a) = converters::extract_rgb(it.next())?;

    println!("/all {}/{}/{}/{}", r, g, b, a);

    for i in 0..CUBE_WIDTH {
        for j in 0..CUBE_WIDTH {
            for k in 0..CUBE_WIDTH {
                paint(&mut window, voxels, i, j, k, r, g, b, a)?;
            }
        }
    }

    Ok(())
}

// /pt x y z rgb
pub fn paint_pt(
    mut window: &mut Window,
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    let mut it = args.iter();

    let (x, y, z) = converters::extract_xyz(&mut it)?;
    let (r, g, b, a) = converters::extract_rgb(it.next())?;

    println!("/pt {}x{}x{} {}/{}/{}/{}", x, y, z, r, g, b, a);

    paint(&mut window, voxels, x, y, z, r, g, b, a)
}

// /line x1 y1 z1 x2 y2 z2 rgb
pub fn paint_line(
    mut window: &mut Window,
    voxels: &mut Vec<Vec<Vec<SceneNode>>>,
    args: &Vec<OscType>,
) -> Result<(), Box<Error>> {
    let mut it = args.iter();

    let (x1, y1, z1) = converters::extract_xyz(&mut it)?;
    let (x2, y2, z2) = converters::extract_xyz(&mut it)?;
    let (r, g, b, a) = converters::extract_rgb(it.next())?;

    println!(
        "/line {}x{}x{} {}x{}x{} {}/{}/{}/{}",
        x1, y1, z1, x2, y2, z2, r, g, b, a
    );

    for (x, y, z) in bresenham3d::line(
        (x1 as i32, y1 as i32, z1 as i32),
        (x2 as i32, y2 as i32, z2 as i32),
    ) {
        paint(
            &mut window,
            voxels,
            x as usize,
            y as usize,
            z as usize,
            r,
            g,
            b,
            a,
        )?;
    }

    Ok(())
}
