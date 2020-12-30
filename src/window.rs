/*use iced::canvas::{self, Cursor, path, Path, Stroke};
use iced::{
    executor, window, Application, Canvas, Color, Command, Element,
    Length, Point, Rectangle, Settings, Size, Vector,
};
*/
use iced::{
    canvas::{self, Cursor, path, Path, Stroke, LineJoin, LineCap},
    executor, window, Application, Canvas, Color, Command, Element,
    Length, Point, Rectangle, Settings, Size, Subscription, Vector,
};


use crate::plot::*;

pub struct Plotting {
    state: State,
}

struct State {
    plot_background: canvas::Cache,
    plot: Plot2D,

}


impl State {
    
    pub fn new(plot: Plot2D) -> Self {
        Self {
        plot_background: Default::default(),
        plot,
        }
        
    }

    pub fn add_plot(&mut self, plot: Line2D) {
        self.plot.get_mut_lines().push(plot);
    }
}

#[derive(Debug)]
pub enum Message {
    ShowCalled(Line2D),
}


impl Application for Plotting {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Plotting, Command<Self::Message>) {
        (
            Plotting {
                state: State::new(Plot2D::new()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
           Message::ShowCalled(plot)  => self.state.add_plot(plot)
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {

        
    }

    fn view(&mut self) -> Element<Message> {
        Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        
    }

}

impl<Message> canvas::Program<Message> for State {
    fn draw(
        &self,
        bounds: Rectangle,
        _cursor: Cursor,
        ) -> Vec<canvas::Geometry> {

        let xlims = self.plot.get_axes().get_axes().get_xaxes();
        let ylims = self.plot.get_axes().get_axes().get_xaxes();

        let background = self.plot_background.draw(bounds.size(), |frame| {

            let mut x_grid = Linspace::linspace(xlims[0], xlims[1], (frame.width()+1.0) as usize);
            let mut y_grid = Linspace::linspace(ylims[0], ylims[1], (frame.height()+1.0) as usize);
            let mut x_center: Vec<(f64, f32)> = x_grid.into_iter().
                zip((0..(frame.width() as usize + 1)).map(|x| x as f32)).collect();
            let mut x_origin = x_center.iter().
                filter(|(a, _)| a.min((xlims[0].abs()+xlims[1].abs())/frame.width() as f64)==*a).
                map(|(_, b)| *b).next().unwrap()+10.0;
            let mut y_center: Vec<(f64, f32)> = y_grid.into_iter().
                zip((0..(frame.height() as usize+1)).rev().map(|x| x as f32)).collect();
            let mut y_origin = y_center.iter().
                filter(|(a, _)| a.min((ylims[0].abs()+ylims[1].abs())/frame.height() as f64)==*a).
                map(|(_, b)| *b).next().unwrap()-10.0;

            let mut origin = Point::new(x_origin, y_origin);
            println!("Origin is {:?}", origin);
            println!("end point is {}", x_center.pop().unwrap().0);

            let mut x_axes = path::Builder::new();
            println!("height is {}", frame.height());
            let center = frame.center();
            let right_center = Point::new(frame.width(), y_origin);
            x_axes.move_to(origin);
            x_axes.line_to(right_center);
            x_axes.line_to(right_center+Vector::from([-1.0, -1.0]));
            x_axes.move_to(right_center);
            x_axes.line_to(right_center+Vector::from([-1.0, 1.0]));
            let p = x_axes.build();

            let mut y_axes = path::Builder::new();
            let upper_center = Point::new(x_origin, 0.0);
            y_axes.move_to(origin);
            y_axes.line_to(upper_center);
            y_axes.line_to(upper_center+Vector::from([-1.0, 1.0]));
            y_axes.move_to(upper_center);
            y_axes.line_to(upper_center+Vector::from([1.0, 1.0]));
            let p2 = y_axes.build();
            
            frame.fill(&p, Color::BLACK);
            frame.stroke(&p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

            frame.fill(&p2, Color::BLACK);
            frame.stroke(&p2, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});


        });
        vec![background]
    }
    

}



