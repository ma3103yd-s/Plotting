use plotting::window::Plotting;
use iced::Settings;
use iced::Application;


pub fn main() -> iced::Result {

    Plotting::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    
}
