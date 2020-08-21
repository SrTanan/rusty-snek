extern crate winapi;
extern crate rand;

use winapi::shared::ntdef::*;
use winapi::um::wincon::*;
static mut CONSOLE_HANDLE: Option<HANDLE> = None;

pub mod draw;
pub mod console_helper;

struct Rect {
    x: i16,
    y: i16,
    width: i16,
    height: i16
}
impl Copy for Rect { }

impl Clone for Rect {
    fn clone(&self) -> Rect {
        *self
    }
}

fn key_state(character: i32) -> bool {
    unsafe {
        if (winapi::um::winuser::GetAsyncKeyState(character) as u16 & 0x8000u16) > 0 {
            return true;
        }
    }
    return false;
}

fn detect_collision(a: Rect, b: Rect) -> bool {
    if a.x < b.x + b.width   &&   a.x + a.width > b.x {
        if a.y < b.y + b.height   &&   a.y + a.height > b.y {
            return true;
        }
    }

    return false;
}

fn main() {

    let mut score: u8 = 0;
    let mut high_score: u8 = 0;
    let mut trail = Vec::<Rect>::new();
    let mut tail_length: u8 = 5;
    let mut vx: i16 = 0;
    let mut vy: i16 = 0;
    let current_character: u16 = 42;

    let mut player = Rect {
        x: 50,
        y: 20,
        width: 3,
        height: 2
    };

    let screen_info = SMALL_RECT {
        Left: 0,
        Top: 0,
        Right: 100,
        Bottom: 40
    };

    let mut food = Rect {
        x: rand::random::<i16>()%(screen_info.Right-8),
        y: rand::random::<i16>()%(screen_info.Bottom-6),
        width: 4,
        height: 3
    };

    if food.x < 0 {
        food.x *= -1;
    }

    if food.y < 0 {
        food.y *= -1;
    }

    unsafe {
        winapi::um::wincon::SetConsoleTitleA((b"snek" as *const u8) as *const i8);
        draw::print_text(&format!("{}",0),0,0);
    }

    loop {
        unsafe {
            winapi::um::wincon::SetConsoleWindowInfo(
                console_helper::get_output_handle(),
                1,
                &screen_info
            );
    
            winapi::um::wincon::SetConsoleScreenBufferSize(
                console_helper::get_output_handle(),
                COORD {X: 101, Y: 41}
            );
        }

        draw::clear();
        
        draw::print_text(&format!("Score: {}",score),0,0);
        draw::print_text(&format!("High score: {}",high_score),0,1);

        if key_state(65) == true && vx != 3 {
            vx = -3;
            vy = 0;
        }

        if key_state(68) == true && vx != -3{
            vx = 3;
            vy = 0;
        }

        if key_state(83) == true && vy != -2 {
            vy = 2;
            vx = 0;
        }

        if key_state(87) == true && vy != 2 {
            vy = -2;
            vx = 0;
        }

        player.x += vx;
        player.y += vy;

        if player.x < screen_info.Left {
            player.x = screen_info.Right;
        } else if player.x > screen_info.Right {
            player.x = screen_info.Left;
        }

        if player.y < screen_info.Top {
            player.y = screen_info.Bottom;
        } else if player.y > screen_info.Bottom {
            player.y = screen_info.Top;
        }

        for i in 0..trail.len() {
            if detect_collision(player, trail[i]) == true {
                tail_length = 5;

                if score > high_score {
                    high_score = score;
                }

                score = 0;
                
                vx = 0;
                vy = 0;
                trail.clear();
                break;
            }
        }

        trail.push(Rect {x: player.x, y: player.y, width: 3, height: 2});

        if trail.len() as u8 > tail_length {
            trail = trail.split_off(trail.len() - tail_length as usize);
        }
        
        if detect_collision(player, food) == true {
            tail_length += 1;
            score += 1;

            food = Rect {
                x: rand::random::<i16>()%(screen_info.Right-8),
                y: rand::random::<i16>()%(screen_info.Bottom-6),
                width: 4,
                height: 3
            };

            if food.x < 0 {
                food.x *= -1;
            }
        
            if food.y < 0 {
                food.y *= -1;
            }
        }       

        draw::draw_rectangle(food.x,food.y,4,3,65);

        for i in 0..trail.len() {
            draw::draw_rectangle(trail[i].x,trail[i].y,3,2,current_character);
        }

        //print_text("Hello world!", 0,0);

        unsafe{winapi::um::synchapi::Sleep(60);}
    }
}