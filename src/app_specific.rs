// use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
// use std::ffi::OsString;
// use std::os::windows::ffi::OsStringExt;
// use std::ptr;
// use std::mem;   
// fn get_current_focused_app() -> Option<String> {
//     unsafe {
//         let mut buffer = [0u16; 512];
//         let hwnd = GetForegroundWindow();
//         if hwnd.is_null() {
//             return None;
//         }
//         // Get
//         let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) as usize;
//         if len == 0 {
//             return None;
//         }
//         Some(OsString::from_wide(&buffer[..len]).to_string_lossy().into_owned())
//     }
// }
