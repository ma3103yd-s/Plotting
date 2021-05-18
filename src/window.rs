/*use iced::canvas::{self, Cursor, path, Path, Stroke};
use iced::{
    executor, window, Application, Canvas, Color, Command, Element,
    Length, Point, Rectangle, Settings, Size, Vector,
};
*/



use iced::{
    canvas::{self, Cursor, path, Path, Text, Stroke, Fill, LineJoin, LineCap},
    executor, window, Application, Canvas, Color, Command, Element,
    Length, Point, Rectangle, Settings, Size, Subscription, Vector, HorizontalAlignment,
    VerticalAlignment, Row, button, Button,
};

use std::cmp::Ordering;


use crate::plot::*;

pub struct Window {
   plot: Plotting,
   button_state: button::State,
}

pub struct Plotting {
    state: State,
}

struct State {
    plot_background: canvas::Cache,
    lines: canvas::Cache,
    plot: Plot2D,

}


impl State {

    pub fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    pub fn new(plot: Plot2D) -> Self {
        Self {
        plot_background: Default::default(),
        lines: Default::default(),
        plot,
        }
        
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PlotSaved,
}


impl Application for Window {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Plot2D;

    fn new(_flags: Plot2D) -> (Self, Command<Self::Message>) {
        (
            Self {
                plot: Plotting {
                    state: State::new(_flags),
                },
                button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlotSaved => {
                ()
            },
            _ => (),
        }
        Command::none()
    }


    fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(10)
            .push(
                Button::new(&mut self.button_state, iced::Text::new("Save"))
                .on_press(Message::PlotSaved)
                )
            .push(self.plot.state.view())
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
        let ylims = self.plot.get_axes().get_axes().get_yaxes();

        let edge: usize = 30;


        let background = self.plot_background.draw(bounds.size(), |frame| {

            let x_grid = Linspace::linspace(xlims[0], xlims[1], frame.width() as usize-2*edge);
            let y_grid = Linspace::linspace(ylims[0], ylims[1], frame.height() as usize-2*edge);
            let x_grid_window: Vec<(f64, f32)> = x_grid.iter().cloned().
                zip((edge..(frame.width() as usize - edge+1)).map(|x| x as f32)).collect();
//            let x_origin = x_grid_window.iter().
//                filter(|(a, _)| a.min((xlims[0].abs()+xlims[1].abs())/frame.width() as f64)==*a).
//                map(|(_, b)| *b).next().unwrap();

            let y_grid_window: Vec<(f64, f32)> = y_grid.iter().cloned().
                zip((edge..(frame.height() as usize-edge+1)).rev().map(|x| x as f32)).collect();
//            let y_origin = y_grid_window.iter().
//                filter(|(a, _)| a.min((ylims[0].abs()+ylims[1].abs())/frame.height() as f64)==*a).
//                map(|(_, b)| *b).next().unwrap();


            let y_origin = y_grid_window.iter().min_by(|(a,_), (b,_)| {
                if a.abs()<b.abs() {
                    if(a==b) { return Ordering::Equal }
                    return Ordering::Less
                }
                Ordering::Greater
            }).unwrap().1;
            let x_origin = x_grid_window.iter().min_by(|(a,_), (b,_)| {
                 if a.abs()<b.abs() {
                    if(a==b) { return Ordering::Equal }
                    return Ordering::Less
                }
                Ordering::Greater               
            }).unwrap().1;


            // Draw the x axes
            let mut x_axes = path::Builder::new();
            let center = frame.center();
            let right_center = Point::new(frame.width()-edge as f32, y_origin);
            let left_center = Point::new(edge as f32, y_origin);
            x_axes.move_to(left_center);
            x_axes.line_to(right_center);
            x_axes.line_to(right_center+Vector::from([-1.0, -1.0]));
            x_axes.move_to(right_center);
            x_axes.line_to(right_center+Vector::from([-1.0, 1.0]));
            let p = x_axes.build();
            // Draw the y axes
            let mut y_axes = path::Builder::new();
            let upper_center = Point::new(x_origin, edge as f32);
            let lower_center = Point::new(x_origin, frame.height()-edge as f32);
            y_axes.move_to(lower_center);
            y_axes.line_to(upper_center);
            y_axes.line_to(upper_center+Vector::from([-1.0, 1.0]));
            y_axes.move_to(upper_center);
            y_axes.line_to(upper_center+Vector::from([1.0, 1.0]));
            let p2 = y_axes.build();
            
            frame.fill(&p, Color::BLACK);
            frame.stroke(&p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

            frame.stroke(&p2, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});
            
            // Build the grid depending on plot.
            let mut x_grid = path::Builder::new();
            let mut y_grid = path::Builder::new();
            let grid = self.plot.get_axes();
            let nbr_of_x_points = (xlims[1]-xlims[0])/grid.get_axes().get_scale();
            let nbr_of_y_points = (ylims[1]-ylims[0])/grid.get_axes().get_scale();
            let x_step = (frame.width()-(2*edge) as f32) as f64/nbr_of_x_points;
            let y_step = (frame.height()-(2*edge) as f32) as f64/nbr_of_y_points;

            // Initialize variables for the axes texts
            let mut x_text_val = xlims[0];
            let mut y_text_val = ylims[0];
            let mut x_text: Text = Text::from(format!("{:.0}", x_text_val));
            let mut y_text: Text = Text::from("");
            // Draw the text for the first x value
            x_text.position = Point::new(x_grid_window[0].1, y_origin+5.0);
            x_text.horizontal_alignment = HorizontalAlignment::Center;
            frame.fill_text(x_text);
            // Draw the grid depending on what type of grid the plot uses.
            match grid.grid.as_str() {
                "none" => {
                    let mut x_pos: f32 = x_grid_window[0].1;
                    let mut y_pos: f32 = y_grid_window[0].1;
                    while(x_pos < frame.width()-(edge+1) as f32) {
                        x_grid.move_to(Point::new(x_pos,y_origin-3.0));
                        x_grid.line_to(Point::new(x_pos, y_origin+3.0));
                        x_pos+=x_step as f32;
                        x_text_val += grid.get_axes().get_scale();
                        x_text = Text::from(format!("{:.0}", x_text_val));
                        x_text.position = Point::new(x_pos, y_origin+5.0);
                        x_text.horizontal_alignment = HorizontalAlignment::Center;
                        if((x_pos-x_origin).abs()>0.1*x_step as f32) {
                            if((x_origin-y_origin).abs()>0.01) {
                                frame.fill_text(x_text);
                            }

                        }

                    }

                    while(y_pos > edge as f32) {
                        y_grid.move_to(Point::new(x_origin-3.0, y_pos));
                        y_grid.line_to(Point::new(x_origin+3.0, y_pos));
                        y_text = Text::from(format!("{:.0}", y_text_val));
                        y_text_val += (ylims[1]-ylims[0]).abs()/nbr_of_y_points;
                        y_text.position = Point::new(x_origin-10.0, y_pos);
                        y_text.vertical_alignment = VerticalAlignment::Center;
                        y_text.horizontal_alignment = HorizontalAlignment::Right;
                        if((y_pos-y_origin).abs()>0.1*y_step as f32) {
                            if((x_origin-y_origin).abs()>0.01) {
                                frame.fill_text(y_text);                           
                            }

                        }
                        y_pos-=y_step as f32;



                    }



                },
                "both" => {
                    let mut x_pos: f32 = x_grid_window[0].1;
                    let mut y_pos: f32 = y_grid_window[0].1;
                    while(x_pos < frame.width()+x_step as f32-edge as f32) {
                        x_grid.move_to(Point::new(x_pos,edge as f32));
                        x_grid.line_to(Point::new(x_pos, frame.height()-edge as f32));
                        x_pos+=x_step as f32;
                    }

                    while(y_pos > edge as f32-x_step as f32) {
                        y_grid.move_to(Point::new(edge as f32, y_pos));
                        y_grid.line_to(Point::new(frame.width()-edge as f32, y_pos));
                        y_pos-=y_step as f32;
                    }                   
                },
                _ => panic!("Not a valid string")
            };
            // Draw the text for the last y value
            y_text = Text::from(format!("{:.0}", y_text_val));
            y_text.position = Point::new(x_origin-10.0, edge as f32);
            y_text.vertical_alignment = VerticalAlignment::Center;
            y_text.horizontal_alignment = HorizontalAlignment::Right;
            frame.fill_text(y_text);
            // Build the grid and draw it.
            let x_grid = x_grid.build();
            let y_grid = y_grid.build();
            //frame.stroke(&p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt,
              //  line_join: LineJoin::Miter});
            frame.stroke(&x_grid, Stroke{color: Color::BLACK, width: 1.0, line_cap: LineCap::Butt,
                line_join: LineJoin::Miter});
            frame.stroke(&y_grid, Stroke{color: Color::BLACK, width: 1.0, line_cap: LineCap::Butt,
                line_join: LineJoin::Miter});



        });
        
        let _lines = self.lines.draw(bounds.size(), |frame| {
            let x_grid = Linspace::linspace(xlims[0], xlims[1], ((frame.width()-2.0*edge as f32)*4.0) as usize);
            let y_grid = Linspace::linspace(ylims[0], ylims[1], ((frame.height()-2.0*edge as f32)*4.0) as usize);

            let x_grid_window: Vec<(f64, f32)> = x_grid.iter().cloned().
                zip((4*edge..((frame.width()*4.0) as usize - 4*edge+1)).map(|x| x as f32*0.25)).collect();
            let y_grid_window: Vec<(f64, f32)> = y_grid.iter().cloned().
                zip((4*edge..((frame.height()*4.0) as usize-4*edge+1)).rev().map(|x| x as f32*0.25)).collect();
            for line in self.plot.get_lines() {
                 let mut line_draw  = path::Builder::new();
                 let min_y_vals: Vec<f64> = y_grid.iter().map(|x| (x-line.data[0].1).abs()).collect();
                 let min_x_vals: Vec<f64> = x_grid.iter().map(|x| (x-line.data[0].0).abs()).collect();
                 let (_, ind_y) = min(&min_y_vals);
                 let (_, ind_x) = min(&min_x_vals);
                 let start_y = y_grid_window[ind_y].1;
                 let start_x = x_grid_window[ind_x].1;
//                 let start_y = y_grid_window.iter().find(|(a,_)| *a==line.data[0].1).unwrap().1;
                 line_draw.move_to(Point::new(start_x, start_y));
                 for (x,y) in line.get_data().iter() {
//                 line.get_data().iter().map(|(x,y)| {
                    let x_coord = x_grid_window.iter().min_by(|&&x_1, &&x_2| {
                        let diff = (x_1.0-x).abs()-(x_2.0-x).abs();
                        if diff.is_sign_positive() {
                            if diff==0.0 {
                                return Ordering::Equal
                            }
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }

                    }).unwrap().1;
                    let y_coord = y_grid_window.iter().min_by(|&&y_1, &&y_2| {
                        let diff = (y_1.0-y).abs()-(y_2.0-y).abs();
                        if diff.is_sign_positive() {
                            if diff==0.0 {
                                return Ordering::Equal
                            }
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }

                    }).unwrap().1;

                    let new_point = Point::new(x_coord, y_coord);
                    match &line.linestyle[0..] {
                        "-" => line_draw.line_to(new_point),
                        "." => {
                           line_draw.circle(new_point, 2f32); 
                        },
                        _ => panic!("Not a valid linestyle"),
                    }

                }
                let p = line_draw.build();
                let line_color = line.get_color();
                let color = iced::Color::new(line_color.0, line_color.1, line_color.2, line_color.3);
                //frame.fill(&p, Color::BLACK);
                match &line.linestyle[0..] {
                    "-" => frame.stroke(&p, Stroke{color, width: 2.0, line_cap: LineCap::Butt,
                    line_join: LineJoin::Miter}),
                    "." => {
                       frame.fill(&p, color); 
                    },
                    _ => panic!("Not a valid linestyle"),
                }

            }
            
        });

        vec![background, _lines]
        
    
    }
    

}



