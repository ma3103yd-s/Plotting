use plotting::plot::Plot2D;
use plotting::plot::Linspace;
use plotting::plot::Line2D;
use plotting::plot::Color;



pub fn main() {

    let x: Vec<f64> = Linspace::linspace(-10.0, 10.0, 100);
    let y: Vec<f64> = x.iter().map(|&x| x*x*x).collect();
    //let mut plot = Plot2D::plot((&x, &y).into()).grid("none");
    let y2: Vec<f64> = x.iter().map(|&x| x*x).collect();
    let line = Line2D::new(&x,&y2).color(Color::RED).linestyle(".");
    //plot.add_line(line);
    Plot2D::plot(line).show();
    //println!("lines are {:?}", plot.get_lines());


    //plot.show();

    
    
}
