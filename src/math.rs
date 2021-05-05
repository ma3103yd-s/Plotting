use nalgebra::geometry::{Point3, Perspective3};
use nalgebra::base::{Matrix3, DMatrix,Matrix, DVector};
use nalgebra::base::Vector3;
use std::ops::{Add, Sub};
use crate::plot::*;

pub fn create_camera(pos: &Vector3<f32>, target: &Vector3<f32>) -> Matrix3<f32> {
    let dir: Vector3<f32> = (target-pos).normalize();
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let x: Vector3<f32> = dir.cross(&up).normalize();
    let z: Vector3<f32> = x.cross(&dir).normalize();

    let mut camera_matrix  = Matrix3::from_columns(&[x, z, dir]);
    camera_matrix

}

pub fn project(camera: &Matrix3<f32>, point: &Vector3<f32>) -> [f32;2] {
    let hpoint = point;
    let point_3d = camera*point;
    let point_2d = [point_3d[0], point_3d[2]];
    point_2d
    
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
    let coords = Vector3::new(x,y,z);
    coords
}

pub fn cartesian_to_polar(points: &Vector3<f32>) -> (f32,f32,f32) {
    let r = points.norm();
    let theta = (points[2]/r).acos();
    let phi = (points[1]).atan2(points[0]);
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



