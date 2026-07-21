use std::ops::ControlFlow::{Break, Continue};

use serde::{Serialize, Deserialize};

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
                self.pixels[4*(self.width*pixel.0+pixel.1)+0],
                self.pixels[4*(self.width*pixel.0+pixel.1)+1],
                self.pixels[4*(self.width*pixel.0+pixel.1)+2], 
                self.pixels[4*(self.width*pixel.0+pixel.1)+3],
            ))
        }
    }
    pub fn set_pixel(&mut self, pixel: (usize, usize), color: Color) {
        if pixel.0 > self.width || pixel.1 > self.height {
        } else {
            self.pixels[4*(self.width*pixel.0+pixel.1)+0] = color.0;
            self.pixels[4*(self.width*pixel.0+pixel.1)+1] = color.1;
            self.pixels[4*(self.width*pixel.0+pixel.1)+2] = color.2;
            self.pixels[4*(self.width*pixel.0+pixel.1)+3] = color.3;
        }
    }
    pub fn clear(&mut self) {
        self.pixels = vec![0; self.pixels.len()];
    }
    pub fn draw_rect(&mut self, coord: (usize, usize), size: (usize, usize), color: Color) {
        if coord.0 > self.width || coord.1 > self.height || coord.0+size.0> self.width || coord.1+size.1> self.height {
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