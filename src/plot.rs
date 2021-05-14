use crate::window::{Window};
use crate::window_3d::Window3D;
use crate::window::Message;
use crate::math::*;
use iced::Settings;
use iced::window;
use iced::Application;
use nalgebra::base::{Matrix, DMatrix};



#[derive(Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);


impl Color {

    pub const BLACK: Color = Color{0: 0.0, 1: 0.0, 2: 0.0, 3: 1.0};
    pub const RED: Color = Color(1.0, 0.0, 0.0, 1.0);
    pub const BLUE: Color = Color(0.0, 0.0, 1.0, 1.0);

}

pub fn min<T: Into<f64>+Copy>(vals: &[T]) -> (f64, usize) {
    let mut min = std::f64::MAX;
    let mut pos: usize = 0;
    for (i, &val) in vals.iter().enumerate() {
        let val:f64 = val.into();
        if val < min {
            min = val;
            pos = i;
        }


    }
    return (min, pos)
}

pub fn double_min<T: Into<(f64, f64)>+Copy>(vals: &[T]) -> (f64,f64) {
    let mut min_1 = std::f64::MAX;
    let mut min_2 = min_1;
    for (&x) in vals.iter() {
        let (first, second) = x.into();
        if first < min_1 {
            min_1 = first;
        }
        if second < min_2 {
            min_2 = second;
        }

    }

    return (min_1, min_2)

}

pub fn double_max<T: Into<(f64, f64)>+Copy>(vals: &[T]) -> (f64,f64) {
    let mut max_1 = std::f64::MIN;
    let mut max_2 = max_1;
    for (&x) in vals.iter() {
        let (first, second) = x.into();
        if first > max_1 {
            max_1 = first;
        }
        if second > max_2 {
            max_2 = second;
        }

    }

    return (max_1, max_2)

}

pub struct Axes2D {
    xlim: [f64;2],
    ylim: [f64;2],
    scale: f64
}

impl Axes2D {
    pub fn new() -> Self {
        Self {
            xlim: [0.0, 1.0],
            ylim: [0.0, 1.0],
            scale: 1.0,
        }
    }

    pub fn get_xaxes(&self) -> [f64;2] {
        self.xlim
    }
    pub fn get_yaxes(&self) -> [f64;2] {
        self.ylim
    }
    pub fn axes(mut self, xlim: &[f64;2], ylim: &[f64;2]) -> Self {
        self.xlim = xlim.to_owned();
        self.ylim = ylim.to_owned();
        self
    }
    pub fn scale(&mut self, scale: f64) {
        self.scale=scale
    }
    pub fn get_scale(&self) -> f64 {
        self.scale
        
    }
}


pub struct Plot2D {
    title: String,
    xlabel: String,
    ylabel: String,
    axes: Grid,
    lines: Vec<Line2D>,
    
}
#[derive(Debug)]
pub struct Axes3D {
    xlim: [f32;2],
    ylim: [f32;2],
    zlim: [f32;2],
    scale: f32,
    
}

impl Axes3D {
    pub fn new() -> Self {
        Self {
            xlim: [0.0, 1.0],
            ylim: [0.0, 1.0],
            zlim: [0.0, 1.0],
            scale: 1.0,
        }
    }
    pub fn get_xaxes(&self) -> [f32;2] {
        self.xlim
    }
    pub fn get_yaxes(&self) -> [f32;2] {
        self.ylim
    }
    pub fn get_zaxes(&self) -> [f32;2] {
       self.zlim 
    }
    pub fn scale(&mut self, scale: f32) {
        self.scale = scale;
        
    }
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn axes(mut self, xlim: &[f32;2], ylim: &[f32;2], zlim: &[f32;2]) -> Self {
        self.xlim = xlim.to_owned();
        self.ylim = ylim.to_owned();
        self.zlim = zlim.to_owned();
        self
    }
    
}

pub struct Plot3D {
    title: String,
    xlabel: String,
    ylabel: String,
    zlabel: String,
    axes: Grid3D,
    surface: Option<Surface3D>,
    
}

pub struct Colormap(pub Vec<(f32,f32,f32)>);

impl Colormap {
    
    pub fn new(name: &str, data: &[f32]) -> Self {
        let vals = Linspace::linspace(0.0,3.0, data.len());
        let mut colors = vec![(0.0,0.0,0.0);data.len()];
        let mut sorted_data: Vec<f32> = data.iter().map(|x| *x).collect();
        sorted_data.as_mut_slice().sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let color_map = map_points_f32(&sorted_data[0..], (0.0, 3.0));
        //println!("color_map is {:?}", color_map);
        match name {
            "hot" => {
            for val in color_map.iter() {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                if val.1.max(1.0) == 1.0 {
                    r=val.1;
                } else if val.1.max(2.0) == 2.0 {
                    r=1.0;
                    g=val.1-1.0;
                } else if val.1.max(3.0) == 3.0{
                    r=1.0;
                    g=1.0;
                    b=val.1-2.0;
                }
                let mut index_iter= data.iter().enumerate().filter(|(i, x)| **x==val.0);
                while let Some(indices) = index_iter.next() {
                    colors[indices.0] = (r,g,b);
                }
                
                }
            },
            _ => (),
        };
        Self{0:colors}


    }
    
}

pub struct Surface3D {
    color: Option<Color>,
    pub x_data: DMatrix<f32>,
    pub y_data: DMatrix<f32>,
    pub z_data: DMatrix<f32>,
    pub colormap: Option<Colormap>,
    legend: Option<String>,
}
#[derive(Debug)]
pub struct Grid3D {
    pub axes: Axes3D,
    pub grid: String,
}

#[derive(Debug)]
pub struct Line2D {
    color: Color,
    pub data: Vec<(f64, f64)>,
    pub linestyle: String,
    legend: Option<String>,
}



pub struct Grid {
    pub axes: Axes2D,
    pub grid: String,
}

pub struct Plot {
    
}


impl Plot3D {
    pub fn new() -> Self {
        Self {
            title: String::from("Plot"),
            xlabel: String::from("x"),
            ylabel: String::from("y"),
            zlabel: String::from("z"),
            axes: Grid3D::default(),
            surface: None,
        }
    }

    pub fn plot(s: Surface3D) -> Plot3D {
        let mut default = Self::new();
        let x_min: f32 = s.x_data.row(0).min();
        let x_max: f32 = s.x_data.row(0).max();
        let y_min: f32 = s.y_data.column(0).min();
        let y_max: f32 = s.y_data.column(0).max();
        let z_min: f32 = s.z_data.min();
        let z_max: f32 = s.z_data.max();
        let g = Grid3D::new(Axes3D::new().axes(&[x_min, x_max],
                                               &[y_min, y_max],
                                               &[z_min, z_max]),
                                               "none");
        default.axes = g;

        default.surface = Some(s);
        default
    }

    pub fn colormap(mut self, colormap: &str) -> Plot3D {
        let s = self.surface.map(|mut s| {
            let cmap = Colormap::new(colormap, s.z_data.as_slice());
            s.colormap = Some(cmap);
            s
        });
        self.surface = s;
        self



    }

    pub fn get_axes(&self) -> &Grid3D {
        &self.axes
    }
    pub fn get_surface(&self) -> &Option<Surface3D> {
        &self.surface
    }

//    pub fn show(plot: Plot3D<'static>) -> iced::Result {
//        Window3D::run(Settings{
//            window: window::Settings::default(),
//            flags: plot,
//            default_font: None,
//            default_text_size: 20,
//            antialiasing: true,
//        })
//    }

}

impl Plot2D {


    pub fn plot(l: Line2D) -> Self {
        let mut default = Self::new();
        let (x_min, y_min): (f64,f64) = double_min(&l.data);
        let (x_max, y_max): (f64,f64) = double_max(&l.data);
        let g = Grid::new(Axes2D::new().axes(&[x_min, x_max], &[y_min, y_max]), "none");
        default.axes = g;
        default.lines.push(l);
        default

    }

    pub fn _plot<T: Into<f64> + Copy>(x: &[T], y: &[T]) -> Self {
        let mut default = Self::new();
        let x_min: f64 = (x[0]).into();
        let x_max: f64 = (*(x.last().unwrap())).into();
        let (y_min,_): (f64, usize) = min(y);
        let y_max: f64 = (*(y.last().unwrap())).into();
        let g = Grid::new(Axes2D::new().axes(&[x_min, x_max], &[y_min, y_max]), "none");
        let line = Line2D::new(x, y);
        default.axes = g;
        default.lines.push(line);
        default

    }

    pub fn new() -> Self {
        Self {
            title: String::from("Plot"),
            xlabel:  String::from("x"),
            ylabel: String::from("y"),
            axes: Grid::default(),
            lines: Vec::new(),

        }
    }

    pub fn show(self) -> iced::Result {
        Window::run(Settings{
            window: window::Settings::default(),
            flags: self,
            default_font: None,
            default_text_size: 20,
            antialiasing: true,
        })
    }

    pub fn grid(mut self, grid: &str) -> Self {
        self.axes.grid = String::from(grid);
        self
    }

//    pub fn get_axes(xlim: &[f64;2], ylim: &[f64;2], grid: &str) {
//        
//    }
    pub fn get_axes(&self) -> &Grid {
        &self.axes
    }

    pub fn get_lines(&self) -> &Vec<Line2D> {
        &self.lines
    }

    pub fn add_line(&mut self, line: Line2D) {
        self.lines.push(line);
    }


}

impl Grid {
    pub fn default() -> Self {
        Self {
            axes: Axes2D{xlim: [0.0, 1.0], ylim: [0.0, 1.0], scale: 1.0},
            grid: String::from("none"),
        }
    }
    pub fn get_axes(&self) -> &Axes2D {
        &self.axes
    }
    pub fn new(axes: Axes2D, grid: &str) -> Self {
        Self {
            axes,
            grid: grid.to_owned(),
        }
    }
}

impl Grid3D {
    pub fn default() -> Self {
        Self {
            axes: Axes3D{xlim: [0.0, 1.0], ylim: [0.0, 1.0], zlim: [0.0, 1.0], scale: 1.0},
            grid: String::from("none"),
        }
    }
    pub fn get_axes(&self) -> &Axes3D {
        &self.axes
    }
    pub fn new(axes: Axes3D, grid: &str) -> Self {
        Self {
            axes,
            grid: grid.to_owned(),
        }
    }
}

impl Line2D {

    pub fn new<T: Into<f64> + Copy>(x: &[T], y: &[T]) -> Self {
        let data: Vec<(f64,f64)> = x.iter().map(|&x| x.into()).zip(y.iter().map(|&y| y.into())).collect();
        Self {
            color: Color::BLACK,
            data,
            linestyle: "-".to_owned(),
            legend: None,
            
        }
    }
    
    pub fn color(mut self, color: Color) -> Self {
       self.color = color;
       self
    }

    pub fn get_color(&self) -> &Color {
        &self.color
        
    }

    pub fn get_data(&self) -> &Vec<(f64,f64)> {
       &self.data 
    }

    pub fn linestyle(mut self, linestyle: &str) -> Self {
        self.linestyle = linestyle.to_owned();
        self
    }


}
impl Surface3D {
    pub fn new(x: DMatrix<f32>, y: DMatrix<f32>, z: DMatrix<f32>) -> Self {
        Self {
            color: Some(Color::BLUE),
            x_data: x,
            y_data: y,
            z_data: z,
            colormap: None,
            legend: None,
        }
        
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn get_color(&self) -> &Option<Color> {
        &self.color
    }
    pub fn as_mut_ref(&mut self) -> &mut Self {
        self
    }
}


impl From<(&[f64], &[f64])> for Line2D {

    fn from(vals: (&[f64], &[f64])) -> Self {
        Line2D::new(vals.0, vals.1)
    }
    
}

impl From<(&Vec<f64>, &Vec<f64>)> for Line2D {
    
    fn from(vals: (&Vec<f64>, &Vec<f64>)) -> Self {
        Line2D::new(vals.0, vals.1)
    }

}

pub struct Linspace<T> {
    data: Vec<T>,
}

impl Linspace<f64> {

    pub fn linspace(start: f64, end: f64, steps: usize) -> Vec<f64> {
        let step_size = (end-start)/(steps as f64);
        let mut data = (0..steps+1).map(|x| start+(x as f64)*step_size).collect();
        data
    }

}

impl Linspace<f32> {
    pub fn linspace_f32(start: f32, end: f32, steps: usize) -> Vec<f32> {
        let step_size = (end-start)/(steps as f32);
        let mut data = (0..steps+1).map(|x| start+(x as f32)*step_size).collect();
        data
    }
    
}

impl IntoIterator for Linspace<f64> {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

