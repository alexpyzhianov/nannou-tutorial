use nannou::prelude::*;

const ROW_COUNT: u32 = 40;
const COL_COUNT: u32 = 40;
const PAD: u32 = 20;
const CANVAS_WIDTH: u32 = COL_COUNT * PAD;
const CANVAS_HEIGHT: u32 = ROW_COUNT * PAD;
const DOT_RADIUS_MAG: f32 = 1.1;
const DOT_RADIUS_MIN: f32 = DOT_RADIUS_MAG + 1.5;
const WAVE_AMPLITUDE: f32 = 8.0;
const WAVE_PERIOD: f32 = 5.0;
const WAVE_SPEED: f32 = 1.9;

fn main() {
    println!("Rendering a {} by {} image", CANVAS_WIDTH, CANVAS_HEIGHT);
    nannou::sketch(view).size(CANVAS_WIDTH, CANVAS_HEIGHT).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let bounds = app.window_rect().pad(PAD as f32 * 2.0);
    let row_h = bounds.h() / ROW_COUNT as f32;
    let col_w = bounds.w() / COL_COUNT as f32;
    let t = app.duration.since_start.secs() as f32 * WAVE_SPEED;

    draw.background().color(hsv(0.0, 0.0, 0.01));

    for row in 0..=ROW_COUNT {
        for col in 0..=COL_COUNT {
            let period = t + (row + col) as f32 / WAVE_PERIOD;

            let x = bounds.left() + (col as f32) * col_w;
            let x_offset = period.sin() * WAVE_AMPLITUDE;
            let y = bounds.top() - (row as f32) * row_h;
            let y_offset = period.cos() * WAVE_AMPLITUDE;

            draw.ellipse()
                .x_y(x + x_offset, y + y_offset)
                .radius(DOT_RADIUS_MAG * period.sin() + DOT_RADIUS_MIN)
                .color(hsv(0.0, 0.0, 0.9));
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if app.keys.down.contains(&Key::S) {
        app.main_window().capture_frame(
            app.exe_name().unwrap() + app.elapsed_frames().to_string().as_ref() + ".png",
        );
    }

    // Uncomment to lower the framerate
    // std::thread::sleep(std::time::Duration::from_millis(100));
}
