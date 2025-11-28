// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, Mouse, Settings};
use parking_lot::Mutex;
use rand::Rng;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, State, Icon};
use image::io::Reader as ImageReader;
use std::io::Cursor;

struct JigglerState {
    is_running: Arc<Mutex<bool>>,
    idle_threshold: Arc<Mutex<u64>>,
    jiggle_interval: Arc<Mutex<u64>>,
}

#[tauri::command]
fn start_jiggler(state: State<JigglerState>, window: tauri::Window, app_handle: tauri::AppHandle) -> Result<String, String> {
    let mut running = state.is_running.lock();
    
    if *running {
        return Ok("Already running".to_string());
    }
    
    *running = true;
    drop(running);
    
    // Update system tray icon and status
    let icon_bytes = include_bytes!("../icons/icon-active.png");
    if let Ok(reader) = ImageReader::new(Cursor::new(icon_bytes)).with_guessed_format() {
        if let Ok(img) = reader.decode() {
            let rgba_data = img.to_rgba8().into_raw();
            let icon = Icon::Rgba { 
                rgba: rgba_data, 
                width: img.width(), 
                height: img.height() 
            };
            let _ = app_handle.tray_handle().set_icon(icon);
        }
    }
    let _ = app_handle.tray_handle().get_item("status").set_title("Status: Running ✓");
    let _ = app_handle.tray_handle().get_item("start").set_enabled(false);
    let _ = app_handle.tray_handle().get_item("stop").set_enabled(true);
    
    // Notify all windows
    let _ = app_handle.emit_all("jiggler-started", ());
    
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
                let mut rng = rand::thread_rng();
                let x_offset = rng.gen_range(-5..=5);
                let y_offset = rng.gen_range(-5..=5);
                
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
fn stop_jiggler(state: State<JigglerState>, app_handle: tauri::AppHandle) -> Result<String, String> {
    let mut running = state.is_running.lock();
    *running = false;
    
    // Update system tray icon and status
    let icon_bytes = include_bytes!("../icons/icon.png");
    if let Ok(reader) = ImageReader::new(Cursor::new(icon_bytes)).with_guessed_format() {
        if let Ok(img) = reader.decode() {
            let rgba_data = img.to_rgba8().into_raw();
            let icon = Icon::Rgba { 
                rgba: rgba_data, 
                width: img.width(), 
                height: img.height() 
            };
            let _ = app_handle.tray_handle().set_icon(icon);
        }
    }
    let _ = app_handle.tray_handle().get_item("status").set_title("Status: Stopped");
    let _ = app_handle.tray_handle().get_item("start").set_enabled(true);
    let _ = app_handle.tray_handle().get_item("stop").set_enabled(false);
    
    // Notify all windows
    let _ = app_handle.emit_all("jiggler-stopped", ());
    
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

#[tauri::command]
fn update_tray_menu(app_handle: tauri::AppHandle, running: bool) -> Result<String, String> {
    let tray = app_handle.tray_handle();
    if running {
        let _ = tray.get_item("start").set_enabled(false);
        let _ = tray.get_item("stop").set_enabled(true);
        let _ = tray.get_item("status").set_title("Status: Running ✓");
    } else {
        let _ = tray.get_item("start").set_enabled(true);
        let _ = tray.get_item("stop").set_enabled(false);
        let _ = tray.get_item("status").set_title("Status: Stopped");
    }
    Ok("Tray menu updated".to_string())
}

fn main() {
    let state = JigglerState {
        is_running: Arc::new(Mutex::new(false)),
        idle_threshold: Arc::new(Mutex::new(120)),
        jiggle_interval: Arc::new(Mutex::new(60)),
    };
    
    // Create system tray menu
    let start = CustomMenuItem::new("start".to_string(), "Start Jiggler");
    let stop = CustomMenuItem::new("stop".to_string(), "Stop Jiggler").disabled();
    let show = CustomMenuItem::new("show".to_string(), "Show Window");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Window");
    let status = CustomMenuItem::new("status".to_string(), "Status: Stopped").disabled();
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(status)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(start)
        .add_item(stop)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(hide)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);
    
    tauri::Builder::default()
        .manage(state)
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let app_handle = app.app_handle();
                match id.as_str() {
                    "start" => {
                        let window = app.get_window("main").unwrap();
                        let state: tauri::State<JigglerState> = app_handle.state();
                        let _ = start_jiggler(state, window.clone(), app_handle.clone());
                    }
                    "stop" => {
                        let state: tauri::State<JigglerState> = app_handle.state();
                        let _ = stop_jiggler(state, app_handle.clone());
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            start_jiggler,
            stop_jiggler,
            get_settings,
            update_settings,
            update_tray_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
