
#[derive(Debug)]
pub struct Color([f64;4]);


impl Color {

    pub const BLACK: Color = Color{0: [0.0,0.0,0.0,1.0]};

}


pub struct Axes2D {
    xlim: [f64;2],
    ylim: [f64;2],
    scale: f64
}

impl Axes2D {
    pub fn get_xaxes(&self) -> [f64;2] {
        self.xlim
    }
    pub fn get_yaxes(&self) -> [f64;2] {
        self.ylim
    }
    pub fn axes(&mut self, xlim: &[f64;2], ylim: &[f64;2]) {
        self.xlim = xlim.to_owned();
        self.ylim = ylim.to_owned();
    }
    pub fn scale(&mut self, scale: f64) {
        self.scale=1.0
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
    data: Vec<(f64, f64)>,
    linestyle: String,
    legend: Option<String>,
}



pub struct Grid {
    pub axes: Axes2D,
    pub grid: String,
}

impl Plot2D {
    pub fn Plot<T: Into<f64>>(x: &[T], y: &[T]) -> Self {
        unimplemented!()
        
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

//    pub fn get_axes(xlim: &[f64;2], ylim: &[f64;2], grid: &str) {
//        
//    }
    pub fn get_axes(&self) -> &Grid {
        &self.axes
    }

    pub fn get_mut_lines(&mut self) -> &mut Vec<Line2D> {
        &mut self.lines
        
    }
}

impl Grid {
    pub fn default() -> Self {
        Self {
            axes: Axes2D{xlim: [0.0, 1.0], ylim: [0.0, 1.0], scale: 0.1},
            grid: String::from("none"),
        }
    }
    pub fn get_axes(&self) -> &Axes2D {
        &self.axes
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

    fn draw() {
        unimplemented!()        
    }

}

pub struct Linspace {
    data: Vec<f64>,
}

impl Linspace {

    pub fn linspace(start: f64, end: f64, steps: usize) -> Self {
        let step_size = (end-start).abs()/(steps as f64);
        let mut data = (0..steps).map(|x| start+(x as f64)*step_size).collect();
        Self {
            data
        }
    }

}

impl IntoIterator for Linspace {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

