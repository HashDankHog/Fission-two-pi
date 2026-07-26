#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::parameter::Parameter;
use solver::parse::{parse, tokenize};
use solver::geometry::profile::*;
use solver::geometry::constrain::Constraint;
use tauri::ipc::Response;
use app::render::*;
use tauri::{Builder, Manager, State};
use std::sync::Mutex;

fn draw_profile(appstate: &mut AppState) {
    appstate.frame.draw_profile(&mut appstate.profile);
}

#[tauri::command]
fn render(state: State<'_, Mutex<AppState>>) -> Response {
    let mut state = state.lock().unwrap();
    draw_profile(&mut state);
    tauri::ipc::Response::new(state.frame.pixels.clone())
}
#[tauri::command]
fn set_screen(width: usize, height: usize, state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.frame.set_dimensions(width, height);
}

#[tauri::command]
fn add_constraint(state: State<'_, Mutex<AppState>>, constraint: Constraint) {
    let mut state = state.lock().unwrap();
    state.profile.add_constraint(constraint);
}

#[tauri::command]
fn remove_constraint() {
    unimplemented!()
}

#[tauri::command]
fn add_connection() {
    unimplemented!()
}

#[tauri::command]
fn remove_connection() {
    unimplemented!()
}

#[tauri::command]
fn set_parameters(_expressions: Vec<&str>) -> Vec<Result<f64, &'static str>> {
    unimplemented!()
}

struct AppState {
    profile: Profile,
    frame: Frame,
}

pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
        app.manage(Mutex::new(AppState{ profile: Profile::new(), frame: Frame::default()}));
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        render, 
        add_constraint,
        remove_constraint,
        add_connection,
        remove_connection,
        set_parameters,
        set_screen
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}