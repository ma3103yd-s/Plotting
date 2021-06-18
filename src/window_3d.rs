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
            camera: Vector3::new(0.19866645,0.7567806,-0.6227477),
            angles: (47.499996, -116.100006),
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

pub fn find_coord(p: f32, points: &[(f32, f32)]) -> (f32, f32) {
    *points.iter().min_by(|&&x_1, &&x_2| {
        let diff = (x_1.1 as f32-p).abs()-(x_2.1 as f32-p).abs();

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
            // Code to rotate the plot when the mouse is clicked and dragged
            
            // Sensitivity for the rotation.
            let sens = 0.1;
            // Get the cursor position if the cursor is inside the window
            let cursor_position =
                if let Some(position) = cursor.position_in(&bounds) {
                    position
                } else {
                    return (event::Status::Ignored, None);
                };
            // match on mouse press
            match event {
                Event::Mouse(mouse_event) => {
                   match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            self.camera_control = Camera::Pressed(cursor_position.x,
                                                                  cursor_position.y);
                        },
                        // Check how much the mouse moved
                        mouse::Event::CursorMoved{x, y} => {

                            
                            // If the camera has been pressed then rotate according to the distance
                            // moved
                            match self.camera_control {
                                Camera::Pressed(x_c, y_c) => {
                                    let dx = x-x_c;
                                    let dy = y_c-y;

                                    self.angles.0 += sens*dy;
                                    self.angles.1 +=sens*dx;
                                    // Only allow rotations in theta to be between -90 to 90
                                    // degrees
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

            
            // Generate the geometry on the canvas

            let points_draw = self.vertice_cache.draw(bounds.size(), |frame| {
                // Draw the title
                let mut title_text = Text::from(self.plot.get_title());
                title_text.position = Point::new(frame.width()*0.5, frame.height()*0.05);
                title_text.size = 30.0;
                frame.fill_text(title_text);


                // Get the axis limits
                let xlims = self.plot.get_axes().get_axes().get_xaxes();
                let ylims = self.plot.get_axes().get_axes().get_yaxes();
                let zlims = self.plot.get_axes().get_axes().get_zaxes();
                    
                // Get the largest absolue value to
                // make sure the grid can fit all points no matter the rotation
                let max_val: f32 = (xlims[1].max(ylims[1])).max(zlims[1]);
                let min_val: f32 = (xlims[0]).min(ylims[0]).min(zlims[0]);
                let max_val = max_val.max(min_val.abs());

                // Create grid of points with some extra space so everything will fit nicely
                let mut grid = Linspace::linspace_f32(-max_val*1.5, max_val*1.5, 1000);

                // Map the grid to the window coordinates for both x and y;
                let mut x_grid_window = map_points_f32(&grid, (frame.width()*0.1, frame.width()*0.9));

                let mut y_grid_window = map_points_f32(&grid, (frame.height()*0.9, frame.height()*0.1));
                
                // Generate the camera. Set the camera to look at the origin
                let mut camera_view = create_camera(&(self.camera), &Vector3::<f32>::zeros());

                // Draw the axes
                let mut axes_drawer = path::Builder::new();
                let axes_3d = self.plot.get_axes().get_axes();
                let x_spacing = axes_3d.get_xspacing();
                let y_spacing = axes_3d.get_yspacing();
                let z_spacing = axes_3d.get_zspacing();
                let nbr_of_z_values = ((zlims[1]-zlims[0])/(z_spacing)) as usize;
                let nbr_of_x_values = ((xlims[1]-xlims[0])/(x_spacing)) as usize;
                let nbr_of_y_values = ((ylims[1]-ylims[0])/(y_spacing)) as usize;

                let spacing = (x_spacing.max(y_spacing)).max(z_spacing);
                // Create a drawer for the axes dashes
                let mut dash_drawer = path::Builder::new();
                
                // Generate an array of values for the axes
                let x_vals_lin = Linspace::linspace_f32(xlims[0], xlims[1], nbr_of_x_values);
                let y_vals_lin = Linspace::linspace_f32(ylims[0], ylims[1], nbr_of_y_values);
                let z_vals_lin = Linspace::linspace_f32(zlims[0], zlims[1], nbr_of_z_values);
                
                // Set locations for where the axes are drawn.
                let mut ystart = xlims[0];
                let mut zstart = ylims[1];
                let mut zxstart = xlims[0];
                let mut zxend = xlims[1];
                let mut xstart = ylims[0];
                
                
                let mut sign = 1.0;
                // Change location of axes depending on angle to make sure they are always visible
                if(self.angles.1 < -180.0) {
                    ystart = xlims[0];
                    zstart = ylims[0];
                    zxstart = xlims[0];
                    zxend = xlims[1];
                    xstart = ylims[1];
                    sign = -1.0;
                }
                
                // Create a drawer for the grid rectangles
                let mut grid_rectangle = path::Builder::new();
                
                // Determine number of decimals to present on axes
                let mut nbr_of_x_digits = 0;
                let mut nbr_of_y_digits = 0;
                let mut nbr_of_z_digits = 0;
                for i in 0..std::f32::DIGITS {
                    let temp_x = x_spacing.fract()*(10.0_f32.powi(i as i32));
                    let diff_x = temp_x.fract();

                    if(diff_x < std::f32::EPSILON) {
                        break;
                    }
                    nbr_of_x_digits +=1;
                }
                for i in 0..std::f32::DIGITS {
                    let temp_y = y_spacing.fract()*(10.0_f32.powi(i as i32));
                    let diff_y = temp_y.fract();
                    if(diff_y < std::f32::EPSILON) {
                        break;
                    }
                    nbr_of_y_digits +=1;
                }
                for i in 0..std::f32::DIGITS {
                    let temp_z = z_spacing.fract()*(10.0_f32.powi(i as i32));
                    let diff_z = temp_z.fract();
                    if(diff_z < std::f32::EPSILON) {

                        break;
                    }
                    nbr_of_z_digits +=1;
                }
                println!("number of z_digits are {}", nbr_of_z_digits);

                // Draw the x axes dashes,texts and xy, xz grid.
                for i in 0..x_vals_lin.len()-1 {
                    // project the dashes to the screen
                    let dash_line = project_line(&camera_view,
                                                &[x_vals_lin[i], zlims[0], -xstart+sign*spacing/4.0].into(),
                                               &[x_vals_lin[i], zlims[0], -xstart - sign*spacing/4.0].into());
                    // Get the closes point in the grid corresponding to the projection
                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;



                    dash_drawer.move_to(Point::new(start_x,start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                    // Generate coordinates for the axes texts                             
                    let mut text = Text::from(format!("{:.ndigits$}", x_vals_lin[i],
                                                      ndigits = nbr_of_x_digits));
                    let start_text = project(&camera_view, &[x_vals_lin[i], zlims[0], -xstart+sign*spacing/2.0]
                                             .into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);

                    if(i==x_vals_lin.len()-2) {
                        
                        let dash_line = project_line(&camera_view,
                                                    &[x_vals_lin[i+1], zlims[0], -xstart+sign*spacing/4.0]
                                                    .into(),
                                                   &[x_vals_lin[i+1], zlims[0], -xstart-sign*spacing/4.0]
                                                   .into());

                        let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                        let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                        let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                        let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;



                        dash_drawer.move_to(Point::new(start_x,start_y));
                        dash_drawer.line_to(Point::new(end_x, end_y));
                                                                                                          
                        let mut text = Text::from(format!("{:.ndigits$}", x_vals_lin[i+1],
                                                          ndigits = nbr_of_x_digits));
                        let start_text = project(&camera_view,
                                                 &[x_vals_lin[i+1], zlims[0], -xstart+sign*spacing/2.0].into());
                        let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                        let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                        text.position = Point::new(text_x_coord, text_y_coord);
                        text.horizontal_alignment = HorizontalAlignment::Center;
                        frame.fill_text(text);


                    }
                    // loop through y points and project the 4 points of the rectangle at the
                    // current x value
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
                    // Do the same for the z-values
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
                // Draw the z-axes dashes, texts and zy grid similarily to the xy,xz grid
                for i in 0..z_vals_lin.len()-1 {
                    let dash_line = project_line(&camera_view,
                                                 &[zxstart, z_vals_lin[i], -zstart-sign*spacing*0.25].into(),
                                                 &[zxstart, z_vals_lin[i], -zstart+sign*spacing*0.25]
                                                 .into());
                    let text_pos = project(&camera_view,
                                           &[zxstart, z_vals_lin[i], -zstart-sign*spacing*0.5].into());
                    let text_x_coord = find_point(text_pos[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(text_pos[1], &y_grid_window[0..]).1;
                    let mut text = Text::from(format!("{:.ndigits$}", z_vals_lin[i],
                                                      ndigits = nbr_of_z_digits));
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Left;
                    frame.fill_text(text);
                    
                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;

                    dash_drawer.move_to(Point::new(start_x, start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                    if(i==z_vals_lin.len()-2) {
                    
                        let dash_line = project_line(&camera_view,
                                                     &[zxstart, z_vals_lin[i+1], -zstart-sign*spacing*0.25]
                                                     .into(),
                                                     &[zxstart, z_vals_lin[i+1], -zstart+sign*spacing*0.25]
                                                     .into());
                        let text_pos = project(&camera_view,
                                               &[zxstart, z_vals_lin[i+1], -zstart-sign*spacing*0.5].into());
                        let text_x_coord = find_point(text_pos[0], &x_grid_window[0..]).1;
                        let text_y_coord = find_point(text_pos[1], &y_grid_window[0..]).1;
                        let mut text = Text::from(format!("{:.ndigits$}", z_vals_lin[i+1],
                                                          ndigits = nbr_of_z_digits));
                        text.position = Point::new(text_x_coord, text_y_coord);
                        text.horizontal_alignment = HorizontalAlignment::Left;
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
                // Draw the y axes
                for &val in y_vals_lin.iter() {
                    let dash_line = project_line(&camera_view,
                                                    &[ystart-spacing/4.0, zlims[0], -val].into(),
                                                     &[ystart+spacing/4.0, zlims[0], -val].into());

                    let start_x = find_point(dash_line[0].0, &x_grid_window[0..]).1;
                    let end_x = find_point(dash_line[1].0, &x_grid_window[0..]).1;
                    let start_y = find_point(dash_line[0].1, &y_grid_window[0..]).1;
                    let end_y = find_point(dash_line[1].1, &y_grid_window[0..]).1;

                    dash_drawer.move_to(Point::new(start_x,start_y));
                    dash_drawer.line_to(Point::new(end_x, end_y));
                                                                                                        
                    let mut text = Text::from(format!("{:.ndigits$}", val, ndigits = nbr_of_y_digits));
                                                                                                        
                    let start_text = project(&camera_view, &[ystart-spacing*0.5, zlims[0], -val].into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                                                                                                        
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Left;
                    frame.fill_text(text);

                }

                // add the dashes to the frame
                frame.stroke(&dash_drawer.build(),Stroke{color: iced::Color::new(0.0,0.0,0.0,1.0),
                width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

                // Add the rectangles to the frame
                let rect_grid_p = grid_rectangle.build();
                frame.fill(&rect_grid_p, iced::Color::new(0.99,0.99,0.99,1.0));
                frame.stroke(&rect_grid_p, Stroke {color: iced::Color::new(0.4, 0.4, 0.4, 1.0),
                width: 2.0, line_cap: LineCap::Butt, line_join: LineJoin::Miter});
                
                // project the axes lines
                let x_axes = project_line(&camera_view,
                                          &[xlims[0], zlims[0], -xstart].into(),
                                          &[xlims[1], zlims[0], -xstart].into());
                let y_axes = project_line(&camera_view,
                                          &[ystart, zlims[0], -ylims[0]].into(),
                                          &[ystart, zlims[0], -ylims[1]].into());
                let z_axes = project_line(&camera_view,
                                          &[zxstart, zlims[0], -zstart].into(),
                                          &[zxstart, zlims[1], -zstart].into());
                // Set spacing for the axes labels. Values are chosen after observations
                let x_label_spacing = (xlims[1]-xlims[0])/30.0;
                let y_label_spacing = (ylims[1]-ylims[0])/10.0;
                let z_label_spacing = (zlims[1]-zlims[0])/8.0;
                // project locations for the labels
                let x_label = project(&camera_view,
                                           &[xlims[1]+x_label_spacing, zlims[0], -xstart].into());
                let y_label = project(&camera_view,
                                           &[ystart,
                                           zlims[0], -ylims[1]-y_label_spacing].into());
                let z_label = project(&camera_view,
                                           &[zxstart, zlims[1]+z_label_spacing, -zstart].into());
                // generate arrays of axes lines and labels
                let all_axes = [x_axes, y_axes, z_axes];
                let labels_pos = [x_label, y_label, z_label];
                let labels = [self.plot.get_xlabel(), self.plot.get_ylabel(), self.plot.get_zlabel()];
                let mut count = 0;
                // Loop through the axes array and draw them
                for &ax in all_axes.iter() {
                let start_x = find_point(ax[0].0, &x_grid_window[0..]).1;
                let end_x = find_point(ax[1].0, &x_grid_window[0..]).1;
                let start_y = find_point(ax[0].1, &y_grid_window[0..]).1;
                let end_y = find_point(ax[1].1, &y_grid_window[0..]).1;
                axes_drawer.move_to(Point::new(start_x, start_y));
                axes_drawer.line_to(Point::new(end_x, end_y));
                let text_x = find_point(labels_pos[count][0], &x_grid_window[0..]).1;
                let text_y = find_point(labels_pos[count][1], &y_grid_window[0..]).1;
                let mut text = Text::from(format!("{}", labels[count]));
                text.position = Point::new(text_x, text_y);
                text.vertical_alignment = VerticalAlignment::Center;
                text.horizontal_alignment = HorizontalAlignment::Center;
                text.size = 20.0;
                frame.fill_text(text);
                count +=1;
                }


                // add the axes to the frame
                let axes_p = axes_drawer.build();
                frame.stroke(&axes_p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});


                
                // Draw the surface plot. Loops through the rows(y-values) and columns(x-values)
                // and draws a rectangle at each z-value with the chosen color.
                if let Some(s) = self.plot.get_surface() {
                    let rows = s.z_data.nrows();
                    let cols = s.z_data.ncols();
                    for row in 0..rows-1 {
                        for col in 0..cols-1 {
                            let mut rectangle_drawer = path::Builder::new();
                            // Get the 4 points of the rectangle
                            let x1 = s.x_data[(0, col)];
                            let y1 = s.y_data[(row, 0)];
                            let x2 = s.x_data[(0, col+1)];
                            let y2 = s.y_data[(row+1, 0)];
                            let z1 = s.z_data[(row, col)];
                            let z2 = s.z_data[(row, col+1)];
                            let z3 = s.z_data[(row+1, col+1)];
                            let z4 = s.z_data[(row+1, col)];
                            // Project the points and get the window coordinates
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
                            // Choose color depeneding on plot settings
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

                            // add the rectangles to the frame
                            frame.fill(&r_p, color);
                            frame.stroke(&r_p, Stroke{color: Color::BLACK, width: 2.0,
                                line_cap: LineCap::Butt,
                                line_join: LineJoin::Miter});


                        }
                    }

                }


        
        });
        // return the geometry produced
        vec![points_draw]
    }

}



