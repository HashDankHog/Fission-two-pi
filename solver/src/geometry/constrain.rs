use crate::vec::NumVec;
use crate::parameter::Parameter;
fn point(points: &NumVec<f64>, point: usize) -> NumVec<f64> {
    NumVec(vec![
        points.0[point*3+0],
        points.0[point*3+1],
        points.0[point*3+2]
    ])
}
pub enum Constraint {
    Distance{point_1: usize, point_2: usize, distance: f64},
    Angle{point_1: usize, point_2: usize, point_3: usize, cosangle: f64}, 
    Fix{point: usize, position: (f64,f64,f64)}, //I would have this as a num vec, but for some reason I cant due to some bs with lifetimes
}

impl Constraint {
    pub fn error(self) -> Parameter {
        match self {
            Self::Distance { point_1: p1, point_2: p2, distance: d } => {
                Parameter(Box::new(move |p| 
                    (
                        (p.0[p2*3+0]-p.0[p1*3+0]).powf(2.0) +
                        (p.0[p2*3+1]-p.0[p1*3+1]).powf(2.0) +
                        (p.0[p2*3+2]-p.0[p1*3+2]).powf(2.0) -
                        d.powf(2.0)
                    ).powf(2.0)
                ))
            },
            Self::Angle { point_1: p1, point_2: p2, point_3: p3, cosangle: a } => {
                Parameter(Box::new(move |p| 
                    (
                        (
                            point(p,p1).add(&point(p,p3).scale(-1.0)).unwrap()
                            .dot_prod(&point(p,p2).add(&point(p,p3).scale(-1.0)).unwrap()).unwrap()
                        ) /
                        (
                            point(p,p1).add(&point(p,p3).scale(-1.0)).unwrap()
                            .dot_prod(&point(p,p1).add(&point(p,p3).scale(-1.0)).unwrap()).unwrap().powf(0.5)
                            *
                            point(p,p2).add(&point(p,p3).scale(-1.0)).unwrap()
                            .dot_prod(&point(p,p2).add(&point(p,p3).scale(-1.0)).unwrap()).unwrap().powf(0.5)
                        )
                        - a
                    ).powf(2.0)
                ))
            },
            Self::Fix { point: p1, position: pos } => {
                Parameter(Box::new(move |p| 
                (
                    p.0[p1*3+0] - pos.0 +
                    p.0[p1*3+1] - pos.1 +
                    p.0[p1*3+2] - pos.2
                ).powf(2.0)
            ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_distance() {
        let p = NumVec(vec![
            0.0,0.0,0.0,
            1.0,0.0,0.0]);
        let con = Constraint::Distance { point_1: 0, point_2: 1, distance: 1.0 };
        assert_eq!(con.error().0(&p), 0.0)
    }

    #[test]
    fn error_angle() {
        let p = NumVec(vec![
            1.0,0.0,0.0,
            0.0,1.0,0.0,
            0.0,0.0,0.0
        ]);
        let con = Constraint::Angle { point_1: 0, point_2: 1, point_3: 2, cosangle: 0.0 };
        assert_eq!(con.error().0(&p),0.0)
    }

    #[test]
    fn error_fix() {
        let p = NumVec(vec![
            1.0,2.0,3.0
        ]);
        let con = Constraint::Fix { point: 0, position: (1.0,2.0,3.0) };
        assert_eq!(con.error().0(&p),0.0);
    }
}
