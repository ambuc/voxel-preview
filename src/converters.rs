use na::Translation3;
use rosc::{OscColor, OscType};
use std::error::Error;
use std::slice;

// there's some nonsense here. Because drawing system coordinates hang from a
// ceiling and are right-handed, where normal x/y/z coordinates rise from the
// origin plane and are right-handed, we end up having to flip and rotate them
// so that they match our intuition. TODO: represent this in a neat way with a
// nalgebra transformation.
pub fn translation_for(i: usize, j: usize, k: usize) -> Translation3<f32> {
    Translation3::new((1.0) * (i as f32), (1.0) * (k as f32), (-1.0) * (j as f32))
}

// graceful error handling is undesirable. we want to know if the controller is malformed.
pub fn osc_int_to_usize(input: Option<&OscType>) -> Result<usize, Box<Error>> {
    match input {
        Some(OscType::Int(x)) => Ok(*x as usize),
        _ => bail!("couldn't find osctype::int"),
    }
}

// expects (r,g,b,a) as a native OSC color type
pub fn extract_rgb(input: Option<&OscType>) -> Result<(f32, f32, f32, f32), Box<Error>> {
    match input {
        Some(OscType::Color(OscColor {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        })) => Ok((
            (*r as f32) / 255.0,
            (*g as f32) / 255.0,
            (*b as f32) / 255.0,
            (*a as f32) / 255.0,
        )),
        _ => bail!("couldn't find osctype::color"),
    }
}

// expects an iterator, and pops the next three off it
pub fn extract_xyz(it: &mut slice::Iter<'_, OscType>) -> Result<(usize, usize, usize), Box<Error>> {
    let x: usize = osc_int_to_usize(it.next())?;
    let y: usize = osc_int_to_usize(it.next())?;
    let z: usize = osc_int_to_usize(it.next())?;
    Ok((x, y, z))
}
