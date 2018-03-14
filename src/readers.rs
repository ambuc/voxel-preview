use na::{Point3, Vector3};
use palette::LinSrgba;
use rosc::OscType;
use std::error::Error;
use std::slice;

pub fn lin_srgba(it: &mut slice::Iter<'_, OscType>) -> Result<LinSrgba<f32>, Box<Error>> {
    let r: f32 = match it.by_ref().next() {
        Some(OscType::Float(r)) => *r,
        _ => {
            return Err(From::from("No match for <r> in LinSrgba".to_string()));
        }
    };
    let g: f32 = match it.by_ref().next() {
        Some(OscType::Float(g)) => *g,
        _ => {
            return Err(From::from("No match for <g> in LinSrgba".to_string()));
        }
    };
    let b: f32 = match it.by_ref().next() {
        Some(OscType::Float(b)) => *b,
        _ => {
            return Err(From::from("No match for <b> in LinSrgba".to_string()));
        }
    };
    let a: f32 = match it.by_ref().next() {
        Some(OscType::Float(a)) => *a,
        _ => {
            return Err(From::from("No match for <a> in LinSrgba".to_string()));
        }
    };
    Ok(LinSrgba::new(r, g, b, a))
}

pub fn dsc_point_3(it: &mut slice::Iter<'_, OscType>) -> Result<Point3<i32>, Box<Error>> {
    let x: i32 = match it.by_ref().next() {
        Some(OscType::Int(x)) => *x,
        _ => {
            return Err(From::from("No match for <x> in Point3".to_string()));
        }
    };
    let y: i32 = match it.by_ref().next() {
        Some(OscType::Int(y)) => *y,
        _ => {
            return Err(From::from("No match for <y> in Point3".to_string()));
        }
    };
    let z: i32 = match it.by_ref().next() {
        Some(OscType::Int(z)) => *z,
        _ => {
            return Err(From::from("No match for <z> in Point3".to_string()));
        }
    };
    Ok(Point3::new(x, y, z))
}

pub fn dsc_vector_3(it: &mut slice::Iter<'_, OscType>) -> Result<Vector3<i32>, Box<Error>> {
    let i: i32 = match it.by_ref().next() {
        Some(OscType::Int(i)) => *i,
        _ => {
            return Err(From::from("No match for <i> in Vector3".to_string()));
        }
    };
    let j: i32 = match it.by_ref().next() {
        Some(OscType::Int(j)) => *j,
        _ => {
            return Err(From::from("No match for <j> in Vector3".to_string()));
        }
    };
    let k: i32 = match it.by_ref().next() {
        Some(OscType::Int(k)) => *k,
        _ => {
            return Err(From::from("No match for <k> in Vector3".to_string()));
        }
    };
    Ok(Vector3::new(i, j, k))
}

pub fn int(it: &mut slice::Iter<'_, OscType>) -> Result<i32, Box<Error>> {
    let n: i32 = match it.by_ref().next() {
        Some(OscType::Int(n)) => *n,
        _ => {
            return Err(From::from("No match for <n> in Int".to_string()));
        }
    };
    Ok(n)
}
