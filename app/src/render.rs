use serde::{Serialize, Deserialize};
use solver::geometry::profile::*;
use solver::geometry::connection::Connection;
#[derive(Clone, Serialize, Deserialize)]
pub struct Color(pub u8,pub u8,pub u8, pub u8);

#[derive(Serialize, Deserialize, Clone)]
pub struct Frame {
    pub pixels: Vec<u8>,
    width: usize,
    height: usize,
}
impl Default for Frame {
    fn default() -> Self {
        Frame { 
            pixels: vec![0,0,0,255],
            width:  1,
            height: 1 
        }
    }
}
impl From<(usize, usize)> for Frame {
    fn from(value: (usize, usize)) -> Self {
        let mut vec = Vec::new();
        for _ in 0..value.0*value.1 {
            vec.push(0);
            vec.push(0);
            vec.push(0);
            vec.push(255);
        }
        Frame { pixels: vec, width: value.0, height: value.1 }
    }
}
impl Frame {
    pub fn pixel(&self, pixel: (usize, usize)) -> Result<Color, &'static str> {
        if pixel.0 > self.width || pixel.1 > self.height {
            Err("attempted to read pixel outside of bounds")
        } else {
            Ok(Color(
                self.pixels[4*(self.width*pixel.1+pixel.0)+0],
                self.pixels[4*(self.width*pixel.1+pixel.0)+1],
                self.pixels[4*(self.width*pixel.1+pixel.0)+2], 
                self.pixels[4*(self.width*pixel.1+pixel.0)+3],
            ))
        }
    }
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels = vec![0; width*height*4];
    }
    pub fn set_pixel(&mut self, pixel: (usize, usize), color: Color) {
        if pixel.0 > self.width || pixel.1 > self.height {
        } else {
            self.pixels[4*(self.width*pixel.1+pixel.0)+0] = color.0;
            self.pixels[4*(self.width*pixel.1+pixel.0)+1] = color.1;
            self.pixels[4*(self.width*pixel.1+pixel.0)+2] = color.2;
            self.pixels[4*(self.width*pixel.1+pixel.0)+3] = color.3;
        }
    }
    pub fn clear(&mut self) {
        self.pixels = vec![0; self.pixels.len()];
    }
    pub fn draw_rect(&mut self, coord: (usize, usize), size: (usize, usize), color: Color) {
        if coord.0 >= self.width || coord.1 >= self.height || coord.0+size.0 >= self.width || coord.1+size.1 >= self.height {
            return;
        }
        for height in coord.1..=(coord.1+size.1) {
            for width in coord.0..=(coord.0+size.0) {
                self.set_pixel((width, height), color.clone());
            }
        }
    }
    pub fn draw_line(&mut self, _point1: (usize, usize), _point2: (usize, usize)) {
        unimplemented!()
    }
    /// This is a temporary function that will be removed in a later version
    pub fn draw_profile(&mut self, profile: &mut Profile){
        let point_radius = 20;
        let (points, _) = profile.points();
        for point in points {
            if point.0 as usize >= point_radius && point.1 as usize >= point_radius{
                self.draw_rect((point.0 as usize -point_radius, point.1 as usize-point_radius), (point_radius*2,point_radius*2), Color(0,0,0,255));
            }
        }
        for connection in profile.connections.clone() {
            match connection {
                _ => {} // unimplemented
            }
        }
    }
}

pub struct Camera {
    pub position: (f64, f64, f64),
    pub heading: (f64, f64, f64),
    pub focal_length: f64
}
impl Camera {
    pub fn translate(&mut self, position: Point, heading: Point) {
        self.position.0 += position.0;
        self.position.1 += position.1;
        self.position.2 += position.2;

        self.heading.0 += heading.0;
        self.heading.1 += heading.1;
        self.heading.2 += heading.2;
    }
    fn project_point(&mut self, _point: Point) {
        unimplemented!()
    }
    pub fn render_to(&self, _frame: &mut Frame) {
        unimplemented!()
    }
}

/* 
#[cfg(test)]
mod tests {

use tauri::image;

use super::*;

#[test]
fn test(){
    let point = mage
    let a = 
    println!()
}
}
*/