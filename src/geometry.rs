use CUBE_WIDTH;
use bresenham3d;
use na::{distance, Point3, Vector3};

pub fn discrete_line(pt: Point3<i32>, dir: Vector3<i32>) -> Vec<Point3<i32>> {
    bresenham3d::line(pt, pt + dir)
}

pub fn discrete_plane(
    pt: Point3<i32>,
    u_hat: Vector3<i32>,
    v_hat: Vector3<i32>,
) -> Vec<Point3<i32>> {
    let mut cells = vec![];
    for a_pt in bresenham3d::line(pt, pt + u_hat) {
        for b_pt in bresenham3d::line(a_pt, a_pt + v_hat) {
            cells.push(b_pt);
        }
    }
    cells
}

pub fn discrete_frame(
    pt: Point3<i32>,
    u_hat: Vector3<i32>,
    v_hat: Vector3<i32>,
) -> Vec<Point3<i32>> {
    let mut cells = vec![];
    cells.append(&mut bresenham3d::line(pt, pt + u_hat));
    cells.append(&mut bresenham3d::line(pt, pt + v_hat));
    cells.append(&mut bresenham3d::line(pt + u_hat, pt + u_hat + v_hat));
    cells.append(&mut bresenham3d::line(pt + v_hat, pt + u_hat + v_hat));
    cells
}

pub fn discrete_cuboid(
    pt: Point3<i32>,
    u_hat: Vector3<i32>,
    v_hat: Vector3<i32>,
    w_hat: Vector3<i32>,
) -> Vec<Point3<i32>> {
    let mut cells = vec![];
    for a_pt in bresenham3d::line(pt, pt + u_hat) {
        for b_pt in bresenham3d::line(a_pt, a_pt + v_hat) {
            for c_pt in bresenham3d::line(b_pt, b_pt + w_hat) {
                cells.push(c_pt);
            }
        }
    }
    cells
}

pub fn discrete_sphere(center: Point3<i32>, p: i32) -> Vec<Point3<i32>> {
    let mut cells = vec![];
    for i in (center.x - p)..(center.x + p) {
        for j in (center.y - p)..(center.y + p) {
            for k in (center.z - p)..(center.z + p) {
                let dist: f32 = distance(
                    &Point3::new(i as f32, j as f32, k as f32),
                    &Point3::new(center.x as f32, center.y as f32, center.z as f32),
                );
                if dist <= p as f32 {
                    cells.push(Point3::new(i, j, k));
                }
            }
        }
    }
    cells
}

pub fn discrete_shell(center: Point3<i32>, p: i32) -> Vec<Point3<i32>> {
    let mut cells = vec![];
    for i in (center.x - p)..(center.x + p) {
        for j in (center.y - p)..(center.y + p) {
            for k in (center.z - p)..(center.z + p) {
                let dist: f32 = distance(
                    &Point3::<f32>::new(i as f32, j as f32, k as f32),
                    &Point3::<f32>::new(center.x as f32, center.y as f32, center.z as f32),
                );
                if (dist - (p as f32)).abs() <= 0.5 {
                    cells.push(Point3::<i32>::new(i, j, k));
                }
            }
        }
    }
    cells
}

pub fn all_cells() -> Vec<Point3<i32>> {
    let mut cells = vec![];
    for i in 0..CUBE_WIDTH {
        for j in 0..CUBE_WIDTH {
            for k in 0..CUBE_WIDTH {
                cells.push(Point3::new(i, j, k));
            }
        }
    }
    cells
}
