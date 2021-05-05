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
            camera: Vector3::new(0.0, 0.0, 10.0),
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
                                    let phi_new = phi+dy*dphi;
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
                let y_1 = Vector3::new(0.0,0.0, -2.0);
                let y_2 = Vector3::new(0.0, 0.0, 2.0);
                let x_1 = Vector3::new(-2.0, 0.0, 0.0);
                let x_2 = Vector3::new(2.0, 0.0, 0.0);
                let z_1 = Vector3::new(0.0,-2.0, 0.0);
                let z_2 = Vector3::new(0.0, 2.0, 0.0);
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
                let mut grid = Linspace::linspace_f32(-3.0, 3.0, 1000);


                let mut x_grid_window = map_points_f32(&grid, (frame.width()*0.3, frame.width()*0.7));
                //println!("Frame height is {:?}", frame.height());
                let mut y_grid_window = map_points_f32(&grid, (frame.height()*0.7, frame.height()*0.3));

                let mut camera_view = create_camera(&(self.camera), &Vector3::<f32>::zeros());

                // Draw the axes
                let mut axes_drawer = path::Builder::new();
                let axes_3d = self.plot.get_axes().get_axes();
                let nbr_of_x_points = (xlims[1]-xlims[0])/axes_3d.get_scale();
                let nbr_of_y_points = (ylims[1]-ylims[0])/axes_3d.get_scale();
                let nbr_of_z_points = (zlims[1]-zlims[0])/axes_3d.get_scale();


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



//                for axes in &[(x_axes_1, x_axes_2), (y_axes_1, y_axes_2), (z_axes_1, z_axes_2)] {
//                    let x_1 = axes.0[0];
//                    let y_1 = axes.1[1];
//                    let x_2 = axes.1[0];
//                    let y_2 = axes.1[1];
////                    println!("x1, y1 is {},{}", x_1, y_1);
////                    println!("x2, y2 is {},{}", x_2,y_2);
//                    let x_1_coord = find_point(x_1, &x_grid_window[0..]).1;
//                    let y_1_coord = find_point(y_1, &y_grid_window[0..]).1;
//                    let x_2_coord = find_point(x_2, &x_grid_window[0..]).1;
//                    let y_2_coord = find_point(y_2, &y_grid_window[0..]).1;
////                    println!("x1_coord, y1_cord is {},{}", x_1_coord, y_1_coord);
////                    println!("x2_coord, y2_coord is {},{}", x_2_coord,y_2_coord);
//                    axes_drawer.move_to(Point::new(x_1_coord, y_1_coord));
//                    axes_drawer.line_to(Point::new(x_2_coord, y_2_coord));
//
//                }

                let axes_p = axes_drawer.build();
                frame.stroke(&axes_p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

                let mut grid_drawer = path::Builder::new();
                

                if let Some(s) = self.plot.get_surface() {
                    grid_drawer.move_to(origin);
                    let mut row_nbr = 0;
                    for row in s.z_data.row_iter() {
                        let y = s.y_data[(row_nbr,0)];
                        for (i, &val) in row.iter().enumerate() {
                            let x = s.x_data[(row_nbr,i)];
                            let z = val;
                            let point = project(&camera_view, &Vector3::new(x,y,-z));
                            let x_coord = find_point(point[0], &x_grid_window[0..]);
                            let y_coord = find_point(point[1], &y_grid_window[0..]);
                            grid_drawer.line_to(Point::new(x_coord.1, y_coord.1));
                        }
                        row_nbr+=1;
                    }
                    grid_drawer.move_to(origin);
                    let mut col_nbr = 0;
                    for col in s.z_data.column_iter() {
                        let x = s.x_data[(0,col_nbr)];
                        for (i, &val) in col.iter().enumerate() {
                            let y = s.y_data[(i, col_nbr)];
                            let z = val;
                            let point = project(&camera_view, &Vector3::new(x,y,-z));
                            let x_coord = find_point(point[0], &x_grid_window[0..]);
                            let y_coord = find_point(point[0], &y_grid_window[0..]);
                            grid_drawer.line_to(Point::new(x_coord.1, y_coord.1));
                        }
                        col_nbr+=1;
                    }
                }

                let grid_p = grid_drawer.build();

                frame.stroke(&grid_p, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});

                
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



