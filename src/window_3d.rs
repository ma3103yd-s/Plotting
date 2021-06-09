use iced::{
    canvas::event::{self, Event},
    canvas::{self, Cursor, path, Path, Text, Stroke, Fill, LineJoin, LineCap},
    executor, window, Application, Canvas, Color, Command, Element, mouse,
    Length, Point, Rectangle, Settings, Size, Subscription, Vector, HorizontalAlignment,
    VerticalAlignment, Row, button, Button,
};
use nalgebra::base::{Vector3, MatrixMN, dimension::{U3,U8}};
use crate::math::*;
use crate::plot::*;
use std::f32::consts::PI;

use std::cmp::Ordering;



pub struct Window3D {
    state: State,
}

pub struct State {
    vertice_cache: canvas::Cache,
    plot: Plot3D,
    camera: Vector3<f32>,
    angles: (f32, f32),
    camera_control: Camera,

}

enum Camera {
    Pressed(f32, f32),
    Released,
}

#[derive(Debug)]
pub enum Message {
    MousePressed,
    MouseReleased,
    
}

impl State {
    
    pub fn new(plot: Plot3D) -> Self {
        Self {
            vertice_cache: Default::default(),
            plot,
            camera: Vector3::new(0.0,0.0,10.0),
            angles: (0.0, -90.0),
            camera_control: Camera::Released,
        }
    }
}

impl Application for Window3D {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Plot3D;

    fn new(_flags: Plot3D) -> (Self, Command<Self::Message>) {
        (
            Self {state: State::new(_flags)},
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plot3D")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }




    fn view(&mut self) -> Element<Message> {
        Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()

                
    }

}

pub fn find_point(p: f32, points: &[(f32,f32)]) -> (f32,f32) {
    *points.iter().min_by(|&&x_1, &&x_2| {
        let diff = (x_1.0 as f32-p).abs()-(x_2.0 as f32-p).abs();

        if diff.is_sign_positive() {
            if diff==0.0 {
                return Ordering::Equal
            }
            Ordering::Greater
        } else {
            Ordering::Less
        }

    }).unwrap()
    
}


impl<Message> canvas::Program<Message> for State {

    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
        ) -> (event::Status, Option<Message>) {
            let sens = 0.1;
            let cursor_position =
                if let Some(position) = cursor.position_in(&bounds) {
                    position
                } else {
                    return (event::Status::Ignored, None);
                };
            match event {
                Event::Mouse(mouse_event) => {
                   match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            self.camera_control = Camera::Pressed(cursor_position.x,
                                                                  cursor_position.y);
                        },
                        mouse::Event::CursorMoved{x, y} => {
                            
                            //println!("Cursor position is {:?}, {:?}", x, y);
                            match self.camera_control {
                                Camera::Pressed(x_c, y_c) => {
                                    let dx = x-x_c;
                                    let dy = y_c-y;

                                    self.angles.0 += sens*dy;
                                    self.angles.1 +=sens*dx;
                                    if(self.angles.0 > 89.0) {
                                        self.angles.0 = 89.0;
                                    }
                                    if(self.angles.0 < -89.0) {
                                        self.angles.0 = -89.0;
                                    }
                                    let dir_x = self.angles.1.to_radians().cos()
                                        *self.angles.1.to_radians().cos();
                                    let dir_y = self.angles.0.to_radians().sin();
                                    let dir_z =  self.angles.1.to_radians().sin()
                                        *self.angles.0.to_radians().cos();
                                    self.vertice_cache.clear();
                                    self.camera = Vector3::new(dir_x, dir_y, dir_z).normalize();
                                    self.camera_control=Camera::Pressed(x,y);

                                },
                                _ => (),
                                
                            }
                        },
                        mouse::Event::ButtonReleased(mouse::Button::Left) => {
                            self.camera_control = Camera::Released;
                        },
                        _ => (),
                        
                    };
                   (event::Status::Captured, None)
                }
                _ =>(event::Status::Ignored, None),
            }
        
    }

    
    fn draw(
        &self,
        bounds: Rectangle,
        _cursor: Cursor,
        ) -> Vec<canvas::Geometry> {

            let points_draw = self.vertice_cache.draw(bounds.size(), |frame| {
                //println!("yaw angle is {}", self.angles.1);
                let xlims = self.plot.get_axes().get_axes().get_xaxes();
                let ylims = self.plot.get_axes().get_axes().get_yaxes();
                let zlims = self.plot.get_axes().get_axes().get_zaxes();

                let max_val: f32 = (xlims[1].max(ylims[1])).max(zlims[1]);
                let min_val: f32 = -max_val;

                let mut origin = Point::new(frame.width()*0.5, frame.height()*0.5);
                // Create grid of points
                let mut grid = Linspace::linspace_f32(min_val*1.1, max_val*1.1, 1000);


                let mut x_grid_window = map_points_f32(&grid, (frame.width()*0.1, frame.width()*0.9));
                //println!("Frame height is {:?}", frame.height());
                let mut y_grid_window = map_points_f32(&grid, (frame.height()*0.9, frame.height()*0.1));

                let mut camera_view = create_camera(&(self.camera), &Vector3::<f32>::zeros());

                // Draw the axes
                let mut axes_drawer = path::Builder::new();
                let axes_3d = self.plot.get_axes().get_axes();
                let scale = axes_3d.get_scale();
                let nbr_of_x_points = (xlims[1]-xlims[0])/scale;
                let nbr_of_y_points = (ylims[1]-ylims[0])/scale;
                let nbr_of_z_points = (zlims[1]-zlims[0])/(scale);

                let mut dash_drawer = path::Builder::new();
                

                let x_vals_lin = Linspace::linspace_f32(xlims[0], xlims[1], nbr_of_x_points as usize);
                let y_vals_lin = Linspace::linspace_f32(ylims[0], ylims[1], nbr_of_y_points as usize);
                let z_vals_lin = Linspace::linspace_f32(zlims[0], zlims[1], nbr_of_z_points as usize);

                let mut ystart = xlims[0];
                let mut zstart = ylims[1];
                let mut zxstart = xlims[0];
                let mut zxend = xlims[1];
                let mut xstart = ylims[0];
                

                let mut sign = 1.0;
                if(self.angles.1 < -180.0) {
                    ystart = xlims[0];
                    zstart = ylims[0];
                    zxstart = xlims[0];
                    zxend = xlims[1];
                    xstart = ylims[1];
                    sign = -1.0;
                }
                
                let mut grid_rectangle = path::Builder::new();
                for i in 0..x_vals_lin.len()-1 {
                    let dash_line = project_line(&camera_view,
                                                &[x_vals_lin[i], zlims[0], -xstart+sign*scale/4.0].into(),
                                               &[x_vals_lin[i], zlims[0], -xstart - sign*scale/4.0].into());

                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;



                    dash_drawer.move_to(Point::new(start_x,start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                                                                                                      
                    let mut text = Text::from(format!("{:.0}", x_vals_lin[i]));
                    let start_text = project(&camera_view, &[x_vals_lin[i], zlims[0], -xstart+sign*scale/2.0]
                                             .into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);

                    if(i==x_vals_lin.len()-2) {
                        
                        let dash_line = project_line(&camera_view,
                                                    &[x_vals_lin[i+1], zlims[0], -xstart+sign*scale/4.0]
                                                    .into(),
                                                   &[x_vals_lin[i+1], zlims[0], -xstart-sign*scale/4.0]
                                                   .into());

                        let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                        let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                        let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                        let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;



                        dash_drawer.move_to(Point::new(start_x,start_y));
                        dash_drawer.line_to(Point::new(end_x, end_y));
                                                                                                          
                        let mut text = Text::from(format!("{:.0}", x_vals_lin[i+1]));
                        let start_text = project(&camera_view,
                                                 &[x_vals_lin[i+1], zlims[0], -xstart+sign*scale/2.0].into());
                        let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                        let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                        text.position = Point::new(text_x_coord, text_y_coord);
                        text.horizontal_alignment = HorizontalAlignment::Center;
                        frame.fill_text(text);


                    }

                    for j in 0..y_vals_lin.len()-1 {
                        let p1 = project(&camera_view,
                                         &[x_vals_lin[i], zlims[0], -y_vals_lin[j]].into());
                        let p2 = project(&camera_view,
                                         &[x_vals_lin[i], zlims[0], -y_vals_lin[j+1]].into());
                        let p3 = project(&camera_view,
                                         &[x_vals_lin[i+1], zlims[0], -y_vals_lin[j+1]].into());
                        let p4 = project(&camera_view,
                                         &[x_vals_lin[i+1], zlims[0], -y_vals_lin[j]].into());
                        let p1_coord_x = find_point(p1[0], &x_grid_window[0..]).1;
                        let p1_coord_y = find_point(p1[1], &y_grid_window[0..]).1;

                        let p2_coord_x = find_point(p2[0], &x_grid_window[0..]).1;
                        let p2_coord_y = find_point(p2[1], &y_grid_window[0..]).1;

                        let p3_coord_x = find_point(p3[0], &x_grid_window[0..]).1;
                        let p3_coord_y = find_point(p3[1], &y_grid_window[0..]).1;

                        let p4_coord_x = find_point(p4[0], &x_grid_window[0..]).1;
                        let p4_coord_y = find_point(p4[1], &y_grid_window[0..]).1;

                        grid_rectangle.move_to(Point::new(p1_coord_x, p1_coord_y));
                        grid_rectangle.line_to(Point::new(p2_coord_x, p2_coord_y));
                        grid_rectangle.line_to(Point::new(p3_coord_x, p3_coord_y));
                        grid_rectangle.line_to(Point::new(p4_coord_x, p4_coord_y));
                        grid_rectangle.line_to(Point::new(p1_coord_x, p1_coord_y));

                    }

                    for j in 0..z_vals_lin.len()-1 {
                        let p1 = project(&camera_view,
                                         &[x_vals_lin[i], z_vals_lin[j], -zstart].into());
                        let p2 = project(&camera_view,
                                         &[x_vals_lin[i], z_vals_lin[j+1], -zstart].into());
                        let p3 = project(&camera_view,
                                         &[x_vals_lin[i+1], z_vals_lin[j+1], -zstart].into());
                        let p4 = project(&camera_view,
                                         &[x_vals_lin[i+1], z_vals_lin[j], -zstart].into());
                        let p1_coord_x = find_point(p1[0], &x_grid_window[0..]).1;
                        let p1_coord_y = find_point(p1[1], &y_grid_window[0..]).1;

                        let p2_coord_x = find_point(p2[0], &x_grid_window[0..]).1;
                        let p2_coord_y = find_point(p2[1], &y_grid_window[0..]).1;

                        let p3_coord_x = find_point(p3[0], &x_grid_window[0..]).1;
                        let p3_coord_y = find_point(p3[1], &y_grid_window[0..]).1;

                        let p4_coord_x = find_point(p4[0], &x_grid_window[0..]).1;
                        let p4_coord_y = find_point(p4[1], &y_grid_window[0..]).1;

                        grid_rectangle.move_to(Point::new(p1_coord_x, p1_coord_y));
                        grid_rectangle.line_to(Point::new(p2_coord_x, p2_coord_y));
                        grid_rectangle.line_to(Point::new(p3_coord_x, p3_coord_y));
                        grid_rectangle.line_to(Point::new(p4_coord_x, p4_coord_y));
                        grid_rectangle.line_to(Point::new(p1_coord_x, p1_coord_y));

                    }

                    

                }

                for i in 0..z_vals_lin.len()-1 {
                    let dash_line = project_line(&camera_view,
                                                 &[zxstart, z_vals_lin[i], -zstart-sign*scale*0.25].into(),
                                                 &[zxstart, z_vals_lin[i], -zstart+sign*scale*0.25]
                                                 .into());
                    let text_pos = project(&camera_view,
                                           &[zxstart, z_vals_lin[i], -zstart-sign*scale*0.5].into());
                    let text_x_coord = find_point(text_pos[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(text_pos[1], &y_grid_window[0..]).1;
                    let mut text = Text::from(format!("{:.0}", z_vals_lin[i]));
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);
                    
                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;

                    dash_drawer.move_to(Point::new(start_x, start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                    if(i==z_vals_lin.len()-2) {
                    
                        let dash_line = project_line(&camera_view,
                                                     &[zxstart, z_vals_lin[i+1], -zstart-sign*scale*0.25]
                                                     .into(),
                                                     &[zxstart, z_vals_lin[i+1], -zstart+sign*scale*0.25]
                                                     .into());
                        let text_pos = project(&camera_view,
                                               &[zxstart, z_vals_lin[i+1], -zstart-sign*scale*0.5].into());
                        let text_x_coord = find_point(text_pos[0], &x_grid_window[0..]).1;
                        let text_y_coord = find_point(text_pos[1], &y_grid_window[0..]).1;
                        let mut text = Text::from(format!("{:.0}", z_vals_lin[i+1]));
                        text.position = Point::new(text_x_coord, text_y_coord);
                        text.horizontal_alignment = HorizontalAlignment::Center;
                        frame.fill_text(text);
                        
                        let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                        let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                        let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                        let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;

                        dash_drawer.move_to(Point::new(start_x, start_y));
                        dash_drawer.line_to(Point::new(end_x, end_y));
                    }


                    for j in 0..y_vals_lin.len()-1 {
                        let p1 = project(&camera_view,
                                         &[zxend, z_vals_lin[i], -y_vals_lin[j]].into());
                        let p2 = project(&camera_view,
                                         &[zxend, z_vals_lin[i], -y_vals_lin[j+1]].into());
                        let p3 = project(&camera_view,
                                         &[zxend, z_vals_lin[i+1], -y_vals_lin[j+1]].into());
                        let p4 = project(&camera_view,
                                         &[zxend, z_vals_lin[i+1], -y_vals_lin[j]].into());
                        let p1_coord_x = find_point(p1[0], &x_grid_window[0..]).1;
                        let p1_coord_y = find_point(p1[1], &y_grid_window[0..]).1;

                        let p2_coord_x = find_point(p2[0], &x_grid_window[0..]).1;
                        let p2_coord_y = find_point(p2[1], &y_grid_window[0..]).1;

                        let p3_coord_x = find_point(p3[0], &x_grid_window[0..]).1;
                        let p3_coord_y = find_point(p3[1], &y_grid_window[0..]).1;

                        let p4_coord_x = find_point(p4[0], &x_grid_window[0..]).1;
                        let p4_coord_y = find_point(p4[1], &y_grid_window[0..]).1;

                        grid_rectangle.move_to(Point::new(p1_coord_x, p1_coord_y));
                        grid_rectangle.line_to(Point::new(p2_coord_x, p2_coord_y));
                        grid_rectangle.line_to(Point::new(p3_coord_x, p3_coord_y));
                        grid_rectangle.line_to(Point::new(p4_coord_x, p4_coord_y));
                        grid_rectangle.line_to(Point::new(p1_coord_x, p1_coord_y));

                    }

                }
                for &val in y_vals_lin.iter() {
                    let dash_line = project_line(&camera_view,
                                                    &[ystart-scale/4.0, zlims[0], -val].into(),
                                                     &[ystart+scale/4.0, zlims[0], -val].into());

                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;

                    dash_drawer.move_to(Point::new(start_x,start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                                                                                                        
                    let mut text = Text::from(format!("{:.0}", val));
                                                                                                        
                    let start_text = project(&camera_view, &[ystart-scale*0.5, zlims[0], -val].into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                                                                                                        
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);

                }


                frame.stroke(&dash_drawer.build(),Stroke{color: iced::Color::new(0.0,0.0,0.0,1.0),
                width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

                
                let rect_grid_p = grid_rectangle.build();
                frame.fill(&rect_grid_p, iced::Color::new(0.99,0.99,0.99,1.0));
                frame.stroke(&rect_grid_p, Stroke {color: iced::Color::new(0.4, 0.4, 0.4, 1.0),
                width: 2.0, line_cap: LineCap::Butt, line_join: LineJoin::Miter});

                let x_axes = project_line(&camera_view,
                                          &[xlims[0], zlims[0], -ystart].into(),
                                          &[xlims[1], zlims[0], -ystart].into());
                let y_axes = project_line(&camera_view,
                                          &[ystart, zlims[0], -ylims[0]].into(),
                                          &[ystart, zlims[0], -ylims[1]].into());
                let z_axes = project_line(&camera_view,
                                          &[zxstart, zlims[0], -zstart].into(),
                                          &[zxstart, zlims[1], -zstart].into());


                let all_axes = [x_axes, y_axes, z_axes];

                for &ax in all_axes.iter() {
                let start_x = find_point(ax[0].0, &x_grid_window[0..]).1;
                let end_x = find_point(ax[1].0, &x_grid_window[0..]).1;
                let start_y = find_point(ax[0].1, &y_grid_window[0..]).1;
                let end_y = find_point(ax[1].1, &y_grid_window[0..]).1;
                axes_drawer.move_to(Point::new(start_x, start_y));
                axes_drawer.line_to(Point::new(end_x, end_y));
                }



                let axes_p = axes_drawer.build();
                frame.stroke(&axes_p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});


                

                if let Some(s) = self.plot.get_surface() {
                    let rows = s.z_data.nrows();
                    let cols = s.z_data.ncols();
                    for row in 0..rows-1 {
                        for col in 0..cols-1 {
                            let mut rectangle_drawer = path::Builder::new();
                            let x1 = s.x_data[(0, col)];
                            let y1 = s.y_data[(row, 0)];
                            let x2 = s.x_data[(0, col+1)];
                            let y2 = s.y_data[(row+1, 0)];
                            let z1 = s.z_data[(row, col)];
                            let z2 = s.z_data[(row, col+1)];
                            let z3 = s.z_data[(row+1, col+1)];
                            let z4 = s.z_data[(row+1, col)];
                            let p1 = project(&camera_view,
                                             &Vector3::new(x1, z1, -y1));
                            let coord_1_x = find_point(p1[0], &x_grid_window[0..]).1;
                            let coord_1_y = find_point(p1[1], &y_grid_window[0..]).1;

                            let p2 = project(&camera_view,
                                             &Vector3::new(x2, z2, -y1));
                            let coord_2_x = find_point(p2[0], &x_grid_window[0..]).1;
                            let coord_2_y = find_point(p2[1], &y_grid_window[0..]).1;
                            let p3 = project(&camera_view,
                                             &Vector3::new(x2, z3, -y2));
                            let coord_3_x = find_point(p3[0], &x_grid_window[0..]).1;
                            let coord_3_y = find_point(p3[1], &y_grid_window[0..]).1;
                            let p4 = project(&camera_view,
                                             &Vector3::new(x1, z4, -y2));
                            let coord_4_x = find_point(p4[0], &x_grid_window[0..]).1;
                            let coord_4_y = find_point(p4[1], &y_grid_window[0..]).1;
                            let p1 = Point::new(coord_1_x, coord_1_y);
                            let p2 = Point::new(coord_2_x, coord_2_y);
                            let p3 = Point::new(coord_3_x, coord_3_y);
                            let p4 = Point::new(coord_4_x, coord_4_y);
                            rectangle_drawer.move_to(p1);
                            rectangle_drawer.line_to(p2);
                            rectangle_drawer.line_to(p3);
                            rectangle_drawer.line_to(p4);
                            rectangle_drawer.line_to(p1);
                            let r_p = rectangle_drawer.build();
                            let color = if let Some(c) = &s.colormap {
                                let color_index = rows*col+row;
                                let r = c.0[color_index].0;
                                let g = c.0[color_index].1;
                                let b = c.0[color_index].2;
                                //println!("r,g,b is {},{},{}", r,g,b);
                                //println!("color index is {}", color_index);
                                iced::Color::new(r,g,b,1.0)                             
                            } else {
                                if let Some(c2) = s.get_color() {
                                    iced::Color::new(c2.0, c2.1, c2.2, c2.3)
                                } else { iced::Color::new(0.0, 0.0, 1.0, 1.0)}
                            };

                            //println!("color is {:?}", color);
                            frame.fill(&r_p, color);
                            frame.stroke(&r_p, Stroke{color: Color::BLACK, width: 2.0,
                                line_cap: LineCap::Butt,
                                line_join: LineJoin::Miter});


                        }
                    }

                }


        
        });
        vec![points_draw]
    }

}



