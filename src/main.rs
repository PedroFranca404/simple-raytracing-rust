use std::f32::consts::PI;
use macroquad::prelude::*;


#[macroquad::main("Ray Tracing in Rust!")]
async fn main() {
    let mut center_x: f32 = screen_width() / 2.0;
    let mut center_y: f32 = screen_height() / 2.0;
    let sun_radius: f32 = 64.0;
    let radius: f32 = 32123.0;
    let rays: i32 = 120;
    let mut colorofsun = YELLOW;

    loop {
        clear_background(LIGHTGRAY);

        if is_sun_hover(center_x, center_y, sun_radius) && is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            center_x = mouse_x;
            center_y = mouse_y;
        }

        draw_circle(center_x, center_y, sun_radius, colorofsun);
        draw_rays(rays, center_x, center_y, radius);

        next_frame().await;
    }
}

fn draw_rays(rays: i32, center_x: f32, center_y: f32, radius: f32) {
    for i in 0..rays {
        let angle: f32 = (i as f32) * 2.0 * PI / (rays as f32);

        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        draw_line(center_x, center_y, x, y,1.0, YELLOW);
    }
}

fn is_sun_hover(center_x: f32, center_y: f32, sun_radius: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let dist_x = mouse_x - center_x;
    let dist_y = mouse_y - center_y;

    let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();

    if dist <= sun_radius {
        return true;
    } else {
        return false;
    }
}