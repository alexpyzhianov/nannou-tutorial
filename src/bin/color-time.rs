use nannou::color;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(640, 640).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let frames_label = format!("Frames elapsed: {}", app.elapsed_frames());
    draw.text(frames_label.as_ref())
        .y(20.0)
        .width(300.0)
        .font_size(24)
        .color(BLACK);

    let fps_label = format!("Current FPS: {:.1}", app.fps());
    draw.text(fps_label.as_ref())
        .y(-20.0)
        .font_size(24)
        .color(BLACK);

    draw.background()
        .color(color::hsv(app.time.sin(), 0.5, 1.0));

    draw.to_frame(app, &frame).unwrap();
}
