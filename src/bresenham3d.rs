use na::Point3;

// adapted from https://gist.github.com/yamamushi/5823518.
pub fn line(p: Point3<i32>, q: Point3<i32>) -> Vec<Point3<i32>> {
    let mut voxels: Vec<Point3<i32>> = vec![];

    let dx: i32 = q.x - p.x;
    let dy: i32 = q.y - p.y;
    let dz: i32 = q.z - p.z;

    let l: i32 = dx.abs();
    let m: i32 = dy.abs();
    let n: i32 = dz.abs();

    let x_inc: i32 = if dx < 0 { -1 } else { 1 };
    let y_inc: i32 = if dy < 0 { -1 } else { 1 };
    let z_inc: i32 = if dz < 0 { -1 } else { 1 };

    let dx2: i32 = l << 1;
    let dy2: i32 = m << 1;
    let dz2: i32 = n << 1;

    let mut err_1: i32;
    let mut err_2: i32;

    let mut curr_x: i32 = p.x;
    let mut curr_y: i32 = p.y;
    let mut curr_z: i32 = p.z;

    if (l >= m) && (l >= n) {
        err_1 = dy2 - l;
        err_2 = dz2 - l;
        for _ in 0..l {
            voxels.push(Point3::<i32>::new(curr_x, curr_y, curr_z));
            if err_1 > 0 {
                curr_y += y_inc;
                err_1 -= dx2;
            }
            if err_2 > 0 {
                curr_z += z_inc;
                err_2 -= dx2;
            }
            err_1 += dy2;
            err_2 += dz2;
            curr_x += x_inc;
        }
    } else if (m >= l) && (m >= n) {
        err_1 = dx2 - m;
        err_2 = dz2 - m;
        for _ in 0..m {
            voxels.push(Point3::<i32>::new(curr_x, curr_y, curr_z));
            if err_1 > 0 {
                curr_x += x_inc;
                err_1 -= dy2;
            }
            if err_2 > 0 {
                curr_z += z_inc;
                err_2 -= dy2;
            }
            err_1 += dx2;
            err_2 += dz2;
            curr_y += y_inc;
        }
    } else {
        err_1 = dy2 - n;
        err_2 = dx2 - n;
        for _ in 0..n {
            voxels.push(Point3::<i32>::new(curr_x, curr_y, curr_z));
            if err_1 > 0 {
                curr_y += y_inc;
                err_1 -= dz2;
            }
            if err_2 > 0 {
                curr_x += x_inc;
                err_2 -= dz2;
            }
            err_1 += dy2;
            err_2 += dx2;
            curr_z += z_inc;
        }
    }

    voxels.push(Point3::<i32>::new(curr_x, curr_y, curr_z));

    voxels
}
