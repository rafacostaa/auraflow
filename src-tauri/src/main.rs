// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, Mouse, Settings};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{Manager, State};

struct JigglerState {
    is_running: Arc<Mutex<bool>>,
    idle_threshold: Arc<Mutex<u64>>,
    jiggle_interval: Arc<Mutex<u64>>,
}

#[tauri::command]
fn start_jiggler(state: State<JigglerState>, window: tauri::Window) -> Result<String, String> {
    let mut running = state.is_running.lock();
    
    if *running {
        return Ok("Already running".to_string());
    }
    
    *running = true;
    drop(running);
    
    let is_running = Arc::clone(&state.is_running);
    let idle_threshold = Arc::clone(&state.idle_threshold);
    let jiggle_interval = Arc::clone(&state.jiggle_interval);
    
    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let mut last_position = enigo.location().unwrap_or((0, 0));
        let mut last_activity = Instant::now();
        let mut is_idle = false;
        
        while *is_running.lock() {
            let current_position = enigo.location().unwrap_or((0, 0));
            let current_time = Instant::now();
            
            // Check for user activity
            if current_position != last_position {
                last_activity = current_time;
                last_position = current_position;
                
                if is_idle {
                    let _ = window.emit("status", "Activity detected - pausing jiggler");
                    is_idle = false;
                }
            }
            
            let idle_time = current_time.duration_since(last_activity).as_secs();
            let threshold = *idle_threshold.lock();
            
            // Check if we should jiggle
            if idle_time >= threshold {
                if !is_idle {
                    let _ = window.emit("status", "Idle detected - starting auto-jiggle");
                    is_idle = true;
                }
                
                // Perform jiggle
                let x_offset = (rand::random::<i32>() % 10) - 5;
                let y_offset = (rand::random::<i32>() % 10) - 5;
                
                let _ = enigo.move_mouse(x_offset, y_offset, enigo::Coordinate::Rel);
                
                last_position = enigo.location().unwrap_or((0, 0));
                
                let message = format!("Jiggled ({:+}, {:+}) - idle for {}s", x_offset, y_offset, idle_time);
                let _ = window.emit("jiggle", message);
                
                let interval = *jiggle_interval.lock();
                thread::sleep(Duration::from_secs(interval));
            } else {
                thread::sleep(Duration::from_secs(5));
            }
        }
        
        let _ = window.emit("status", "Jiggler stopped");
    });
    
    Ok("Jiggler started".to_string())
}

#[tauri::command]
fn stop_jiggler(state: State<JigglerState>) -> Result<String, String> {
    let mut running = state.is_running.lock();
    *running = false;
    Ok("Jiggler stopped".to_string())
}

#[tauri::command]
fn get_settings(state: State<JigglerState>) -> (u64, u64) {
    let idle = *state.idle_threshold.lock();
    let interval = *state.jiggle_interval.lock();
    (idle, interval)
}

#[tauri::command]
fn update_settings(
    state: State<JigglerState>,
    idle_threshold: u64,
    jiggle_interval: u64,
) -> Result<String, String> {
    *state.idle_threshold.lock() = idle_threshold;
    *state.jiggle_interval.lock() = jiggle_interval;
    Ok("Settings updated".to_string())
}

fn main() {
    let state = JigglerState {
        is_running: Arc::new(Mutex::new(false)),
        idle_threshold: Arc::new(Mutex::new(120)),
        jiggle_interval: Arc::new(Mutex::new(60)),
    };
    
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            start_jiggler,
            stop_jiggler,
            get_settings,
            update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
