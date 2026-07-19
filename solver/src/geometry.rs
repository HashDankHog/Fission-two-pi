use crate::parameter::Parameter;
use crate::matrix::Jacobian;
use crate::matrix::Matrix;

#[derive(Clone)]
pub enum Constraint {
    Offset(Vec<(Parameter, usize)>),
    Distance{ point_1: usize, point_2: usize, distance: Parameter },
    Angle{ point_1: usize, point_2: usize, point_3: usize, angle: Parameter }
}

pub enum Connection {
    Line{ point_1: usize, point_2: usize },
    Arc{ point_1: usize, point_2: usize, point_3: usize }
}

pub struct Point(pub Vec<f64>);

pub struct Profile {
    dimension: usize,
    connections: Vec<Connection>,
    constraint_matrix: Jacobian,
    points: Vec<f64>,
}

impl Profile {
    pub fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let point_number = self.points.len()/self.dimension;
        for point in 0..point_number {
            points.push(Point(self.points[point*self.dimension..(point*self.dimension+self.dimension)].to_vec()))
        }
        points
    }
    pub fn update(&mut self) {
        let point_dimension = self.points.len();
        
    }
    pub fn plot(&self) {
        unimplemented!()
    }
    pub fn add_constraint(&mut self, constraint: Constraint) {
        unimplemented!()
    }
    pub fn add_connection(&mut self, connection: Connection) {
        unimplemented!()
    }
    pub fn add_point(&mut self, point: Point) {
        unimplemented!()
    }
    
}
