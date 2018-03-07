// adapted from https://gist.github.com/yamamushi/5823518.
pub fn line((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut voxels: Vec<(i32, i32, i32)> = vec![];

    let dx: i32 = x2 - x1;
    let dy: i32 = y2 - y1;
    let dz: i32 = z2 - z1;

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

    let mut curr_x: i32 = x1;
    let mut curr_y: i32 = y1;
    let mut curr_z: i32 = z1;

    if (l >= m) && (l >= n) {
        err_1 = dy2 - l;
        err_2 = dz2 - l;
        for _ in 0..l {
            voxels.push((curr_x, curr_y, curr_z));
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
            voxels.push((curr_x, curr_y, curr_z));
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
            voxels.push((curr_x, curr_y, curr_z));
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

    voxels.push((curr_x, curr_y, curr_z));

    voxels
}
