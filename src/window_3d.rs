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
    
    pub fn new() -> Self {
        Self {
            vertice_cache: Default::default(),
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
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {state: State::new()},
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


impl<Message> canvas::Program<Message> for State {

    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
        ) -> (event::Status, Option<Message>) {
            let dphi = 0.01/(2.0*PI);
            let dtheta = 0.01/(2.0*PI);
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
                                    let theta_new = theta+dy*dtheta;
                                    self.vertice_cache.clear();
                                    self.camera = polar_to_cartesian(r,theta_new, phi_new);

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
                let mut vertex_map: Vec<Vec<usize>> = Vec::with_capacity(self.vertices.ncols());

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
                let mut x_grid = Linspace::linspace(-5.0, 5.0, 1000);
                let mut y_grid = Linspace::linspace(-5.0, 5.0, 1000);

                let mut x_grid_window = map_points(&x_grid, (frame.width()*0.1, frame.width()*0.9));
                println!("Frame height is {:?}", frame.height());
                let mut y_grid_window = map_points(&y_grid, (frame.height()*0.9, frame.height()*0.1));

                let mut camera_view = create_camera(&(self.camera), &Vector3::<f32>::zeros());
                
                let mut vertex_points: Vec<Point> = Vec::with_capacity(self.vertices.ncols());
                let mut point_drawer = path::Builder::new();
                for v in self.vertices.column_iter() {
                    //println!("Vertice is {}", v);
                    let mut point_2d = project(&camera_view, &v.into());
                    println!("Point is {:?}", point_2d);

                    let x = point_2d[0];
                    let y = point_2d[1];
    //                line.get_data().iter().map(|(x,y)| {
                    let x_coord = x_grid_window.iter().min_by(|&&x_1, &&x_2| {
                        let diff = (x_1.0 as f32-x).abs()-(x_2.0 as f32-x).abs();
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
                        let diff = (y_1.0 as f32-y).abs()-(y_2.0 as f32-y).abs();
                        if diff.is_sign_positive() {
                            if diff==0.0 {
                                return Ordering::Equal
                            }
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }

                    }).unwrap().1;

                    println!("y coord is {}, and x coord is {}", y_coord, x_coord);

                    point_drawer.circle(Point::new(x_coord as f32, y_coord as f32), 2.0);
                    vertex_points.push(Point::new(x_coord as f32, y_coord as f32));
   
            }
                let p = point_drawer.build();
                let mut vertex_drawer = path::Builder::new();
                for i in 0..self.vertices.ncols() {

                    for &j in vertex_map[i].iter() {
                        println!("first point is {:?}: second point is {:?}", vertex_points[i], vertex_points[j]);
                        vertex_drawer.move_to(vertex_points[i]);
                        vertex_drawer.line_to(vertex_points[j]);
                        //frame.fill(&Path::line(vertex_points[i], vertex_points[j]), iced::Color::BLACK);


                    }

                }
                let p_v = vertex_drawer.build();
                println!("The width and height are {:?}:{:?}", frame.width(), frame.height());

                frame.fill(&p, iced::Color::BLACK);
                frame.stroke(&p_v, Stroke{color: Color::BLACK, width: 2.0, line_cap: LineCap::Butt
                , line_join: LineJoin::Miter});
                //frame.fill(&p_v, iced::Color::BLACK);


        
        });
        vec![points_draw]
    }

}



