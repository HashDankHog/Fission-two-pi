use crate::geometry::{connection::Connection, constrain::Constraint};
use crate::parameter::Parameter;
use crate::matrix::*;
use crate::vec::NumVec;

#[derive(Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64, pub f64);

pub struct Profile {
    pub connections: Vec<Connection>,
    points: NumVec<f64>,
    matrix: Jacobian,
    errors: Vec<Parameter>
}
impl Profile {
    pub const fn new() -> Self {
        Profile{connections: Vec::new(), points: NumVec(Vec::new()), 
            matrix: Jacobian::new(), errors: Vec::new()}
    }
    pub fn points(&mut self) -> (Vec<Point>, Solution<f64>) {
        self.matrix = Jacobian::from((self.errors.clone(), &self.points));
        let solution_points: NumVec<f64>;
        let solution = self.matrix.solve(self.points.clone());
        match  solution.clone() {
            Solution::Inconsistant => solution_points = self.points.clone(),
            Solution::Unique(points) => solution_points = points,
            Solution::Infinite { particular: points, homogeneous: _ } => solution_points = points
        }
        let mut return_points = Vec::new();
        for p in 0..solution_points.len()/3 {
            return_points.push(Point(
                solution_points.0[p*3+0],
                solution_points.0[p*3+1],
                solution_points.0[p*3+2]
            ));
        }
        (return_points, solution)
    }
    
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.errors.push(constraint.error())
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection)
    }

    pub fn remove_constraint(&mut self) {
        unimplemented!()
    }

    pub fn remove_connection(&mut self) {
        unimplemented!()
    }
    pub fn add_point(&mut self, coord: Point){
        self.points.push(coord.0);
        self.points.push(coord.1);
        self.points.push(coord.2);
    }
    pub fn remove_point(&mut self, point: usize){
        self.points.0.remove(point*3);
        self.points.0.remove(point*3);
        self.points.0.remove(point*3);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //this test also doenst pass but it is for the same reason
    //this means that I got a GCS working!
    fn points_test() {
        let mut a = Profile::new();
        a.add_point(Point(0.0,0.0,0.0));
        a.add_point(Point(1.0,0.0,0.0));
        a.add_point(Point(2.0,0.5,0.0));

        a.add_constraint(Constraint::FixX { point: 0, position: 0.0 });
        a.add_constraint(Constraint::FixY { point: 0, position: 0.0 });
        a.add_constraint(Constraint::FixZ { point: 0, position: 0.0 });

        a.add_constraint(Constraint::FixX { point: 1, position: 1.0 });
        a.add_constraint(Constraint::FixY { point: 1, position: 0.0 });
        a.add_constraint(Constraint::FixZ { point: 1, position: 0.0 });


        a.add_constraint(Constraint::Distance { point_1: 0, point_2: 2, distance: 1.0 });
        a.add_constraint(Constraint::Angle { point_1: 2, point_2: 1, point_3: 0, cosangle: 0.7 });
        a.add_constraint(Constraint::FixZ { point: 2, position: 0.0 });

        let (points, _) = a.points();
        
        assert_eq!(points,vec!(Point(0.0,0.0,0.0), Point(1.0,0.0,0.0), Point(0.701,0.701,0.0)))
    }

    #[test]
    //this test doesnt pass but it is because of numerical error
    fn points_test_minimal() {
        let mut a = Profile::new();
        a.add_point(Point(2.0,0.5,11.3));
        
        a.add_constraint(Constraint::FixX { point: 0, position: 0.0 });
        a.add_constraint(Constraint::FixY { point: 0, position: 0.0 });
        a.add_constraint(Constraint::FixZ { point: 0, position: 0.0 });

        let (points, _) = a.points();
        assert_eq!(points, vec![Point(0.0,0.0,0.0)]);
    }
}
