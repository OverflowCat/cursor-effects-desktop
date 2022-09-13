#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::os::windows::prelude::OsStrExt;
use tauri::Manager;
use windows::Win32::{
    Foundation::{HWND, POINT},
    UI::WindowsAndMessaging::GetCursorPos,
};

#[derive(Debug, Serialize, Deserialize)]
struct MouseStatus {
    x: i32,
    y: i32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_mouse_position() -> MouseStatus {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point);
        MouseStatus {
            x: point.x,
            y: point.y,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(windows)]
            {
                let hwnd = window.hwnd().unwrap().0;
                let hwnd = HWND(hwnd);
                unsafe {
                    let mut style_ex = WINDOW_EX_STYLE(GetWindowLongW(hwnd, GWL_EXSTYLE) as u32);
                    style_ex |= WS_EX_APPWINDOW // for taskbar
                    | WS_EX_COMPOSITED
                    | WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOPMOST;
                    use windows::Win32::UI::WindowsAndMessaging::*;
                    let nindex = GWL_EXSTYLE;
                    let _pre_val = SetWindowLongA(hwnd, nindex, style_ex.0 as i32);
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_mouse_position])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
