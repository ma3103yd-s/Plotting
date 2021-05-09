use iced::{Application,Settings, window};
use plotting::plot::Plot2D;
use plotting::plot::Linspace;
use plotting::plot::Line2D;
use plotting::plot::Color;
use plotting::plot::Plot3D;
use plotting::plot::Surface3D;
use plotting::window_3d::Window3D;
use plotting::math::*;
use nalgebra::base::Matrix;
use std::f32::consts::PI;




pub fn main() -> iced::Result {

    let x: Vec<f64> = Linspace::linspace(-10.0, 10.0, 100);
    let y: Vec<f64> = x.iter().map(|&x| x*x*x).collect();
    //let mut plot = Plot2D::plot((&x, &y).into()).grid("none");
    let y2: Vec<f64> = x.iter().map(|&x| x*x).collect();
    let line = Line2D::new(&x,&y2).color(Color::RED).linestyle(".");
    //plot.add_line(line);
    //Plot2D::plot(line).show();
    //println!("lines are {:?}", plot.get_lines());
    //
    //
    //
    let x = Linspace::linspace_f32(-5.0, 5.0, 20);
    let y = Linspace::linspace_f32(-5.0, 5.0, 20);
    let (X, Y) = meshgrid(&x,&y);
    //let Z = X.component_mul(&X)+Y.component_mul(&Y);
    let Z = X.map(|x| x.sin())+Y.map(|y| y.cos());
    //println!("Z is {}", Z);
    
    let s = Surface3D::new(X,Y,Z);
    let plot = Plot3D::plot(s).colormap("hot");

    //Plot3D::show(plot)

    Window3D::run(Settings{
        window: window::Settings::default(),
        flags: plot,
        default_font: None,
        default_text_size: 20,
        antialiasing: true,
    })
    //plot.show();

    
    
}
