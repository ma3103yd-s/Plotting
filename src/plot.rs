use crate::window::Plotting;
use crate::window::Message;
use iced::Settings;
use iced::window;
use iced::Application;



#[derive(Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);


impl Color {

    pub const BLACK: Color = Color{0: 0.0, 1: 0.0, 2: 0.0, 3: 1.0};
    pub const RED: Color = Color(1.0, 0.0, 0.0, 1.0);

}

pub fn min<T: Into<f64>+Copy>(vals: &[T]) -> (f64, usize) {
    let mut min = std::f64::MAX;
    let mut pos: usize = 0;
    for &val in vals.iter() {
        let val:f64 = val.into();
        if val < min {
            min = val;
            pos+=1;
        }

    }
    return (min, pos)
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
        self.scale=1.0
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

impl Plot2D {


    pub fn plot<T: Into<f64> + Copy>(x: &[T], y: &[T]) -> Self {
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
        Plotting::run(Settings{
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

pub struct Linspace {
    data: Vec<f64>,
}

impl Linspace {

    pub fn linspace(start: f64, end: f64, steps: usize) -> Vec<f64> {
        let step_size = (end-start).abs()/(steps as f64);
        let mut data = (0..steps+1).map(|x| start+(x as f64)*step_size).collect();
        data
    }

}

impl IntoIterator for Linspace {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

