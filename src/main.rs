use std::f32::consts::PI;
use macroquad::prelude::*;


#[macroquad::main("Hello World!")]
async fn main() {
    let center_x: f32 = screen_width() / 2.0;
    let center_y: f32 = screen_height() / 2.0;
    let radious: f32 = 128.0;
    let mut angle: f32 = 0.0;
    let speed: f32 = 0.1;

    loop {
        clear_background(LIGHTGRAY);

        let x = center_x + radious * angle.cos();
        let y = center_y + radious * angle.sin();

        draw_line(center_x, center_y, x, y, 10.0, RED);

        if angle < 2.0 * PI {
            angle += speed;
        } else {
            angle = 0.0;
        }

        next_frame().await;
    }
}