//Console drawing functions

extern crate winapi;
use winapi::shared::minwindef::*;
use winapi::um::wincon::*;
use console_helper;

pub fn clear() {
    let handle = console_helper::get_output_handle();
    if handle == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        panic!("NoConsole")
    }

    let screen_buffer = console_helper::get_buffer_info();
    let console_size: DWORD = screen_buffer.dwSize.X as u32 * screen_buffer.dwSize.Y as u32;
    let coord_screen = COORD { X: 0, Y: 0 };

    let mut amount_chart_written: DWORD = 0;
    unsafe {
        winapi::um::wincon::FillConsoleOutputCharacterW(
            handle,
            32 as u16,
            console_size,
            coord_screen,
            &mut amount_chart_written,
        );
    }
    console_helper::set_cursor_possition(0, 0);
}

pub fn draw_rectangle(x: i16, y: i16, width: i16, height: i16, character: u16) {
    for _y in 0..height {
        let mut written_amount: DWORD = 0;
        unsafe {
            winapi::um::wincon::FillConsoleOutputCharacterW(
                console_helper::get_output_handle(),
                character,
                width as DWORD,
                COORD { X: x, Y: y+_y},
                &mut written_amount,
            );
        }
    }
}

pub fn print_text(text: &str, x: i16, y: i16) {
    let string_in_bytes = text.as_bytes();

    for i in 0..string_in_bytes.len() {
        let mut written_amount: DWORD = 0;
        unsafe {
            winapi::um::wincon::FillConsoleOutputCharacterW(
                console_helper::get_output_handle(),
                string_in_bytes[i] as u16,
                1 as DWORD,
                COORD { X: x+ (i as i16), Y: y},
                &mut written_amount,
            );
        }
    }
}
