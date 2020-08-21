//Console helper functions, used for console operations.

extern crate winapi;
use winapi::shared::ntdef::*;
use winapi::shared::minwindef::*;
use winapi::um::wincon::*;
use CONSOLE_HANDLE;

pub fn get_output_handle() -> HANDLE {
    unsafe {
        if let Some(handle) = CONSOLE_HANDLE {
            return handle;
        } else {
            let handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
            CONSOLE_HANDLE = Some(handle);
            return handle;
        }
    }
}

pub fn get_buffer_info() -> winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO{
    let handle = get_output_handle();
    if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    let mut buffer = winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
    };
    unsafe {
        winapi::um::wincon::GetConsoleScreenBufferInfo(handle, &mut buffer);
    }
    buffer
}

pub fn set_cursor_possition(y: i16, x: i16) {
    let handle = get_output_handle();
    if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }
    unsafe {
        winapi::um::wincon::SetConsoleCursorPosition(handle, COORD { X: x, Y: y });
    }
}


