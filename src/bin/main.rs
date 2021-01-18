use plotting::window::Plotting;
use plotting::plot::Plot2D;
use plotting::plot::Linspace;
use iced::Settings;
use iced::window;
use iced::Application;


pub fn main() -> iced::Result {

    let x: Vec<f64> = Linspace::linspace(-10.0, 10.0, 100);
    let y: Vec<f64> = x.iter().map(|&x| x*x).collect();
    let plot = Plot2D::plot(&x, &y);
    

    Plotting::run(Settings{
        window: window::Settings::default(),
        flags: plot,
        default_font: None,
        default_text_size: 20,
        antialiasing: true,
    })
    
}
