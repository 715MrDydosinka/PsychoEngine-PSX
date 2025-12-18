#![no_std]
#![no_main]

mod wdf;

use psx::constants::*;
use psx::gpu::{VideoMode, Color};
use psx::{dprintln, Framebuffer};
use psx::{printf, print, println};
use psx::sys::gamepad::{Gamepad, Button};
use psx::dma;

use wdf::parse_wdf;

#[no_mangle]
fn main() {
    let buf0 = (0, 0);
    let buf1 = (0, 240);
    let res = (320, 240);
    //let res = (320, 256);
    let txt_offset = (0, 0);
    let mut fb = Framebuffer::new(buf0, buf1, res, VideoMode::NTSC, Some(Color::new(100, 100, 100))).unwrap();
    let font = fb.load_default_font();
    let mut txt = font.new_text_box(txt_offset, res);
    let mut gpu_dma = dma::GPU::new();

    let mut gamepad = Gamepad::new();

    const COLS: usize = 16;
    const ROWS: usize = 16;

    //let mut wdf = parse_wdf::<ROWS,COLS>("lv0");


    loop {
        txt.reset();


        if gamepad.poll_p1().pressed(Button::Circle)
        {
            println!("Hui");
            dprintln!(txt, "TRIANGLE: {}", gamepad.poll_p1().pressed(Button::Circle));
        }

        fb.draw_sync();
        fb.wait_timer1();
        fb.swap();

    }
}





