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
    vertices: MatrixMN<f32, U3,U8>,
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
            camera: Vector3::new(10.0, -10.0, 0.0),
            vertices: MatrixMN::from_columns(&[Vector3::new(0.0,0.0,0.0), Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, -1.0),
            Vector3::new(1.0,1.0,-1.0),Vector3::new(0.0,1.0,-1.0),Vector3::new(0.0,0.0,-1.0)]),
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
            let dphi = 0.001;
            let dtheta = 0.001;
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
                                    let dy = y-y_c;

                                    let (r, theta, phi) = cartesian_to_polar(&self.camera);
                                    let phi_new = phi+dx*dphi;
                                    let theta_new = theta-dy*dtheta;
                                    self.vertice_cache.clear();
                                    self.camera = polar_to_cartesian(r,theta_new, phi_new);
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
                let xlims = self.plot.get_axes().get_axes().get_xaxes();
                let ylims = self.plot.get_axes().get_axes().get_yaxes();
                let zlims = self.plot.get_axes().get_axes().get_zaxes();

                let max_val: f32 = (xlims[1].max(ylims[1])).max(zlims[1]);
                let min_val: f32 = (xlims[0].min(ylims[0])).min(zlims[0]);



                let mut vertex_map: Vec<Vec<usize>> = Vec::with_capacity(self.vertices.ncols());
                let y_1 = Vector3::new(0.0,0.0, -min_val);
                let y_2 = Vector3::new(0.0, 0.0, -max_val);
                let x_1 = Vector3::new(min_val, 0.0, 0.0);
                let x_2 = Vector3::new(max_val, 0.0, 0.0);
                let z_1 = Vector3::new(0.0, min_val, 0.0);
                let z_2 = Vector3::new(0.0, max_val, 0.0);
                //let x_axes = Vector3::new(5.0, 0.0, 0.0);
                //let z_axes = Vector3::new(0.0, 5.0, 0.0);




                vertex_map.push(vec![1, 7, 3]);
                vertex_map.push(vec![2, 6]);
                vertex_map.push(vec![3, 5]);
                vertex_map.push(vec![4]);
                vertex_map.push(vec![5, 7]);
                vertex_map.push(vec![6]);
                vertex_map.push(vec![7]);
                vertex_map.push(vec![7]);

                let mut origin = Point::new(frame.width()*0.5, frame.height()*0.5);
                // Create grid of points
                let mut grid = Linspace::linspace_f32(2.0*min_val, 2.0*max_val, 1000);


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
                let nbr_of_z_points = (zlims[1]-zlims[0])/scale;

                let mut grid_drawer = path::Builder::new();
                let mut dash_drawer = path::Builder::new();
                

                let x_vals_lin = Linspace::linspace_f32(xlims[0], xlims[1], nbr_of_x_points as usize);
                let y_vals_lin = Linspace::linspace_f32(ylims[0], ylims[1], nbr_of_y_points as usize);
                let z_vals_lin = Linspace::linspace_f32(zlims[0], zlims[1], nbr_of_z_points as usize);
                
                for &val in z_vals_lin.iter() {
                    let text_pos = project(&camera_view, &[xlim-scale/4.0, val, scale/4.0].into());
                    let text_x_coord = find_point(text_pos[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(text_pos[1], &y_grid_window[0..]).1;

                    let dash_line = project_line(&camera_view, &[0.0, val, scale/4.0].into(),
                                                 &[0.0, val, -scale/4.0].into());

                    let mut text = Text::from(format!("{:.0}", val));
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

                for &val in x_vals_lin.iter() {
                    let start = project(&camera_view, &[val, 0.0, -ylims[0]].into());
                    let end = project(&camera_view, &[val, 0.0, -ylims[1]].into());
                    let start_x_coord = find_point(start[0], &x_grid_window[0..]).1;
                    let start_y_coord = find_point(start[1], &y_grid_window[0..]).1;
                    let end_x_coord = find_point(end[0], &x_grid_window[0..]).1;
                    let end_y_coord = find_point(end[1], &y_grid_window[0..]).1;
                    grid_drawer.move_to(Point::new(start_x_coord, start_y_coord));
                    grid_drawer.line_to(Point::new(end_x_coord, end_y_coord));

                    let mut text = Text::from(format!("{:.0}", val));
                    let start_text = project(&camera_view, &[val, 0.0, -ylims[0]+scale/4.0].into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;
                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);

                }
                for &val in y_vals_lin.iter() {
                    let start = project(&camera_view, &[xlims[0], 0.0, -val].into());
                    let end = project(&camera_view, &[xlims[1], 0.0, -val].into());
                    let start_x_coord = find_point(start[0], &x_grid_window[0..]).1;
                    let start_y_coord = find_point(start[1], &y_grid_window[0..]).1;
                    let end_x_coord = find_point(end[0], &x_grid_window[0..]).1;
                    let end_y_coord = find_point(end[1], &y_grid_window[0..]).1;
                    grid_drawer.move_to(Point::new(start_x_coord, start_y_coord));
                    grid_drawer.line_to(Point::new(end_x_coord, end_y_coord));

                    let mut text = Text::from(format!("{:.0}", val));

                    let start_text = project(&camera_view, &[xlims[0]-scale/4.0, 0.0, -val].into());
                    let text_x_coord = find_point(start_text[0], &x_grid_window[0..]).1;
                    let text_y_coord = find_point(start_text[1], &y_grid_window[0..]).1;

                    text.position = Point::new(text_x_coord, text_y_coord);
                    text.horizontal_alignment = HorizontalAlignment::Center;
                    frame.fill_text(text);

                }


                frame.stroke(&dash_drawer.build(),Stroke{color: iced::Color::new(0.0,0.0,0.0,1.0),
                width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});


                frame.stroke(&grid_drawer.build(),Stroke{color: iced::Color::new(0.0,0.0,0.0,0.6),
                width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});



                let x_axes_1 = project(&camera_view, &x_1.into());
                let x_axes_2 = project(&camera_view, &x_2.into());
                let y_axes_1 = project(&camera_view, &y_1.into());
                let y_axes_2 = project(&camera_view, &y_2.into());
                let z_axes_1 = project(&camera_view, &z_1.into());
                let z_axes_2 = project(&camera_view, &z_2.into());

                let all_axes = [(x_axes_1, x_axes_2), (y_axes_1, y_axes_2), (z_axes_1, z_axes_2)];

                for pair in all_axes.iter() {
                let x_1_coord = find_point(pair.0[0], &x_grid_window[0..]).1;
                let y_1_coord = find_point(pair.0[1], &y_grid_window[0..]).1;
                let x_2_coord = find_point(pair.1[0], &x_grid_window[0..]).1;
                let y_2_coord = find_point(pair.1[1], &y_grid_window[0..]).1;
                axes_drawer.move_to(Point::new(x_1_coord, y_1_coord));
                axes_drawer.line_to(Point::new(x_2_coord, y_2_coord));
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
                                iced::Color::new(r,g,b,0.9)                             
                            } else {
                                if let Some(c2) = s.get_color() {
                                    iced::Color::new(c2.0, c2.1, c2.2, c2.3)
                                } else { iced::Color::new(0.0, 0.0, 1.0, 0.9)}
                            };

                            //println!("color is {:?}", color);
                            frame.fill(&r_p, color);
                            frame.stroke(&r_p, Stroke{color: Color::BLACK, width: 2.0,
                                line_cap: LineCap::Butt,
                                line_join: LineJoin::Miter});


                        }
                    }

                }




//                if let Some(s) = self.plot.get_surface() {
//                    let mut row_nbr = 1;
//                    for row in s.z_data.row_iter() {
//                        let start_x = s.x_data[(0,0)];
//                        let start_y = s.y_data[(row_nbr-1,0)];
//                        let start_z = s.z_data[(row_nbr-1,0)];
//                        let start_point = project(&camera_view,
//                                                  &Vector3::new(start_x, start_y, -start_z));
//                        let start_coord_x = find_point(start_point[0], &x_grid_window[0..]).1;
//                        let start_coord_y = find_point(start_point[1], &y_grid_window[0..]).1;
//                        grid_drawer.move_to(Point::new(start_coord_x, start_coord_y));
//                        for (i, &val) in row.iter().enumerate() {
//                            let x = s.x_data[(0, i)];
//                            let z = val;
//                            let point_1 = project(&camera_view,
//                                                      &Vector3::new(x, start_y, -z));
//
//                            let coord_x_1 = find_point(point_1[0], &x_grid_window[0..]).1;
//                            let coord_y_1 = find_point(point_1[1], &y_grid_window[0..]).1;
//                            grid_drawer.line_to(Point::new(coord_x_1, coord_y_1));
//                            if(row_nbr!=s.z_data.nrows()) {
//                                let y = s.y_data[(row_nbr, i)];
//                                let z = s.z_data[(row_nbr, i)];
//
//                                let point_2 = project(&camera_view,
//                                                  &Vector3::new(x, y, -z));
//                                let coord_x_2 = find_point(point_2[0], &x_grid_window[0..]).1;
//                                let coord_y_2 = find_point(point_2[1], &y_grid_window[0..]).1;
//                                grid_drawer.line_to(Point::new(coord_x_2, coord_y_2));
//                                grid_drawer.move_to(Point::new(coord_x_1, coord_y_1));
//                            }
//
//
//                        }
//                        row_nbr +=1;
//
//                    }
//                }


//                let grid_p = grid_drawer.build();

//                frame.stroke(&grid_p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
//                , line_join: LineJoin::Miter});

                
//                let mut vertex_points: Vec<Point> = Vec::with_capacity(self.vertices.ncols());
//                let mut point_drawer = path::Builder::new();
//                // Draw the points.
//                for v in self.vertices.column_iter() {
//                    //println!("Vertice is {}", v);
//                    let mut point_2d = project(&camera_view, &v.into());
//                    println!("Point is {:?}", point_2d);
//
//                    let x = point_2d[0];
//                    let y = point_2d[1];
//    //                line.get_data().iter().map(|(x,y)| {
//                    let x_coord = x_grid_window.iter().min_by(|&&x_1, &&x_2| {
//                        let diff = (x_1.0 as f32-x).abs()-(x_2.0 as f32-x).abs();
//                        if diff.is_sign_positive() {
//                            if diff==0.0 {
//                                return Ordering::Equal
//                            }
//                            Ordering::Greater
//                        } else {
//                            Ordering::Less
//                        }
//
//                    }).unwrap().1;
//
//                    let y_coord = y_grid_window.iter().min_by(|&&y_1, &&y_2| {
//                        let diff = (y_1.0 as f32-y).abs()-(y_2.0 as f32-y).abs();
//                        if diff.is_sign_positive() {
//                            if diff==0.0 {
//                                return Ordering::Equal
//                            }
//                            Ordering::Greater
//                        } else {
//                            Ordering::Less
//                        }
//
//                    }).unwrap().1;
//                    let x_coord = find_point(x, &x_grid_window[0..]).1;
//                    let y_coord = find_point(y, &y_grid_window[0..]).1;
//
//                    //println!("y coord is {}, and x coord is {}", y_coord, x_coord);
//
//                    point_drawer.circle(Point::new(x_coord as f32, y_coord as f32), 2.0);
//                    vertex_points.push(Point::new(x_coord as f32, y_coord as f32));
//   
//            }
//                let p = point_drawer.build();
//                let mut vertex_drawer = path::Builder::new();
//                for i in 0..self.vertices.ncols() {
//
//                    for &j in vertex_map[i].iter() {
//                        println!("first point is {:?}: second point is {:?}", vertex_points[i], vertex_points[j]);
//                        vertex_drawer.move_to(vertex_points[i]);
//                        vertex_drawer.line_to(vertex_points[j]);
//                        //frame.fill(&Path::line(vertex_points[i], vertex_points[j]), iced::Color::BLACK);
//
//
//                    }
//
//                }
//                let p_v = vertex_drawer.build();
//                println!("The width and height are {:?}:{:?}", frame.width(), frame.height());
//
//                frame.fill(&p, iced::Color::BLACK);
//                frame.stroke(&p_v, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
//                , line_join: LineJoin::Miter});
//                frame.fill(&p_v, iced::Color::BLACK);


        
        });
        vec![points_draw]
    }

}



