use std::f32::consts::PI;
use macroquad::prelude::*;

const STEP_SIZE: f32 = 2.0;
const RAYS: i32 = 500;
const SUN_RADIUS: f32 = 64.0;
const OBSTACLE_RADIUS: f32 = 96.0;

#[macroquad::main("Ray Tracing in Rust!")]
async fn main() {
    let mut center_x: f32 = (screen_width() / 4.0);
    let mut center_y: f32 = (screen_height() / 4.0);
    let mut obstacle_x: f32 = (screen_width() / 4.0) * 3.0;
    let mut obstacle_y: f32 = (screen_height() / 4.0) * 3.0;

    loop {
        clear_background(DARKGRAY);

        if is_circle_hover(center_x, center_y, SUN_RADIUS) && is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            center_x = mouse_x;
            center_y = mouse_y;
        }

        if is_circle_hover(obstacle_x, obstacle_y, OBSTACLE_RADIUS) && is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            obstacle_x = mouse_x;
            obstacle_y = mouse_y;
        }

        draw_circle(obstacle_x, obstacle_y, OBSTACLE_RADIUS, GRAY);
        draw_rays(RAYS, center_x, center_y, obstacle_x, obstacle_y);
        draw_circle(center_x, center_y, SUN_RADIUS, WHITE);

        next_frame().await;
    }
}

fn draw_rays(rays: i32, center_x: f32, center_y: f32, obstacle_x: f32, obstacle_y: f32) {
    for i in 0..rays {
        let angle: f32 = (i as f32) * 2.0 * PI / (rays as f32);

        let mut ray_x = center_x;
        let mut ray_y = center_y;

        for i in 0..900 {
            ray_x += angle.cos() * STEP_SIZE;
            ray_y += angle.sin() * STEP_SIZE;

            let dist_x = ray_x - obstacle_x;
            let dist_y = ray_y - obstacle_y;
            let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();

            if dist <= OBSTACLE_RADIUS {
                break;
            }

            if ray_x > screen_width() || ray_x < 0.0 || ray_y > screen_height() || ray_y < 0.0 {
                break
            }
        }

        draw_line(center_x, center_y, ray_x, ray_y, 1.5, YELLOW);
    }
}

fn is_circle_hover(center_x: f32, center_y: f32, sun_radius: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let dist_x = mouse_x - center_x;
    let dist_y = mouse_y - center_y;

    let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();

    dist <= sun_radius
}