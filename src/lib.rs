//this file implements constructive solid geometry trees
//  methods for converting b-rep to csg and vice versa can be found in /src/bin/CAD
//  
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::parameter::Parameter;
use solver::parse::{parse, tokenize};
use solver::{geometry::Profile};
use std::sync::{LazyLock, Mutex};
use tauri::ipc::Response;
use std::cell::{RefCell};
use std::rc::Rc;


static SCREEN: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(vec![0]));
static EMPTY_SCREEN: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut a: Vec<u8> = vec![];
    for _ in 0..1000000 {
        a.push(0);
        a.push(0);
        a.push(0);
        a.push(255);
    }
    a
});

static HEIGHT: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
static WIDTH: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));

static SEGMENTS: LazyLock<Mutex<Vec<usize>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[tauri::command]
fn add_segment(position: usize) {
    (*SEGMENTS.lock().unwrap()).push(position);
}

#[tauri::command]
fn plot(segment: usize, expressions: Vec<String>, color: [u8;3]){
    unimplemented!()
    
}

//TODO: add thickness
//TODO: redo for bresenhams?
fn draw_line(pos_0: (f64,f64), pos_1: (f64,f64), color: [u8;3]) {
    let mut dx = pos_1.0 - pos_0.0;
    let mut dy = pos_1.1 - pos_0.1;
    let step;
    let scale = 2.0;
    if dx.abs() > dy.abs() {
        step = dx.abs() * scale;
    } else {
        step = dy.abs() * scale;
    }
    dx /= step;
    dy /= step;

    let mut x = pos_0.0;
    let mut y = pos_0.1;
    for _ in 0..(step as i32) {
        set_pixel(x.round() as usize, y.round() as usize, color);
        x += dx;
        y += dy;
    }
}

//this function is a complete and utter shit show
//TODO: fix
#[tauri::command]
fn update_parameter(expressions: Vec<String>) -> Vec<f64> {
    unimplemented!()
}

#[tauri::command]
fn update_canvas() -> Response {
    tauri::ipc::Response::new((*SCREEN.lock().unwrap()).clone())
}
#[tauri::command]
fn create_canvas(width: usize, height: usize) {
    *SCREEN.lock().unwrap() = vec![0; width*height*4];
    *WIDTH.lock().unwrap() = width;
    *HEIGHT.lock().unwrap() = height;
}

#[tauri::command]
fn clear_canvas() {
    *SCREEN.lock().unwrap() = (*EMPTY_SCREEN).clone();
}

#[tauri::command]
fn set_pixel(row: usize, colum: usize, c: [u8; 3]){
    
    let mut temp = SCREEN.lock().unwrap();
    let width = *WIDTH.lock().unwrap();
    (*temp)[4*(width*row+colum)] = c[0];
    (*temp)[4*(width*row+colum)+1] = c[1];
    (*temp)[4*(width*row+colum)+2] = c[2];
    (*temp)[4*(width*row+colum)+3] = 255;
}

#[tauri::command]
fn draw_rect(coord:(usize, usize), size: (usize, usize), color: [u8;3]){
    for row in (coord.1)..(coord.1+size.1) {
        for colum in (coord.0)..(coord.0+size.0) {
            set_pixel(row, colum, color);
        }
    }
}
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        update_canvas, create_canvas, clear_canvas, 
        set_pixel, draw_rect, update_parameter,
        add_segment, plot])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}