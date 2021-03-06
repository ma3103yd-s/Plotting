use nalgebra::geometry::{Point3, Perspective3};
use nalgebra::base::{Matrix4, DMatrix,Matrix3, DVector, RowVector3};
use nalgebra::base::{Vector3, Vector4};
use std::ops::{Add, Sub};
use crate::plot::*;

pub fn create_camera(pos: &Vector3<f32>, target: &Vector3<f32>) -> Matrix4<f32> {
    let mut dir: Vector3<f32> = (pos-target).normalize();
    let mut up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let mut x: Vector3<f32> = up.cross(&dir).normalize();
    let mut z: Vector3<f32> = dir.cross(&x).normalize();
    let t: Vector4<f32> = Vector4::new(0.0,0.0,0.0,1.0);

    let mut camera_matrix  = Matrix4::from_columns(&[x.to_homogeneous(), z.to_homogeneous(),
    dir.to_homogeneous(),t]).transpose()
        *Matrix4::from_columns(&[Vector4::new(1.0,0.0,0.0,0.0),
        Vector4::new(0.0,1.0,0.0,0.0),Vector4::new(0.0,0.0,1.0,0.0),
        Vector4::new(-pos[0], -pos[1], -pos[2],0.0)]);
    camera_matrix

}

pub fn project(camera: &Matrix4<f32>, point: &Vector3<f32>) -> [f32;2] {
    //let hpoint = point;
    let point_3d = camera*point.to_homogeneous();
    let point_2d = [point_3d[0], point_3d[2]];
    point_2d
    
}

pub fn project_line(camera: &Matrix4<f32>, point_1: &Vector3<f32>, point_2: &Vector3<f32>) -> [(f32, f32);2] {
    let p1 = project(&camera, &point_1);
    let p2 = project(&camera, &point_2);
    [(p1[0], p1[1]), (p2[0], p2[1])]
}

// Todo. Implement method. Map points to a certain interval
pub fn map_points<T: Add + Sub + Copy, U: Add + Sub + Copy+Into<f64>>(points: &[T], interval: (U, U)) -> Vec<(T,f64)>{
    let diff = interval.1-interval.0;
    let nbr_of_points = points.len();
    let mut new_points = Linspace::linspace(interval.0.into(), interval.1.into(), nbr_of_points);
    let map: Vec<(T, f64)> = points.iter().zip(new_points.iter()).map(|(x, y)| (*x, *y)).collect();
    map
}

pub fn map_points_f32<T: Add + Sub + Copy, U: Add + Sub + Copy+Into<f32>>(points: &[T], interval: (U, U)) -> Vec<(T,f32)>{
    let diff = interval.1-interval.0;
    let nbr_of_points = points.len();
    let mut new_points = Linspace::linspace_f32(interval.0.into(), interval.1.into(), nbr_of_points);
    let map: Vec<(T, f32)> = points.iter().zip(new_points.iter()).map(|(x, y)| (*x, *y)).collect();
    map
}




pub fn polar_to_cartesian(r: f32, theta: f32, phi: f32) -> Vector3<f32> {
    let x = r*theta.sin()*phi.cos();
    let y = r*theta.sin()*phi.sin();
    let z = r*theta.cos();
    let coords = Vector3::new(x,z,-y);
    coords
}

pub fn cartesian_to_polar(points: &Vector3<f32>) -> (f32,f32,f32) {
    let r = points.norm();
    let mut theta = (points[1]/r).acos();
    let mut phi = (-points[2]).atan2(points[0]);
    if(theta>std::f32::consts::PI) {
        theta = theta-std::f32::consts::PI;
        phi = phi+std::f32::consts::PI;
    } else if theta <0_f32 {
        theta = theta.abs();
        phi = phi+std::f32::consts::PI;
    }

    (r, theta, phi)
}


pub fn meshgrid(x: &[f32], y: &[f32]) -> (DMatrix<f32>, DMatrix<f32>) {
    let rows = y.len();
    let cols = x.len();
    let x_iter = x.iter().copied().cycle().take(rows*cols);
    let y_iter = y.iter().copied().cycle().take(rows*cols);
    //println!("Test");
    let mut X = DMatrix::from_iterator(rows, cols, x_iter).transpose();
    let mut Y = DMatrix::from_iterator(rows, cols, y_iter);
    //println!("X is {}", X);
    //println!("Y is {}", Y);


    (X, Y)
    
}



