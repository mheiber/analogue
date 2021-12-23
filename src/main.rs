use analogue::*;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let t = TimeSecs(app.time);
    let square = square_wave(FrequencyHz(1)).scale(100.0);
    let sine = sine_wave(FrequencyHz(1)).scale(100.0);
    let combined = square.clone() + sine.clone();

    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    let half = win.w() / 2.0;
    let range = 1..win.w() as u32;
    let time_and_x = range.map(|right| {
        let time = t + TimeSecs(right as f32 / 100.0);
        let x = (right as f32) - half;
        (time, x)
    });
    let plot = |signal: Signal, vert_shift: f32, color: rgb::Srgb<u8>| {
        let colored_pts = time_and_x.clone().map(|(time, x)| {
            let amp = signal.at(time) + vert_shift;
            (pt2(x, amp), color)
        });
        draw.polyline().weight(3.0).points_colored(colored_pts);
    };
    plot(sine, 0.0, BLUE);
    plot(square, 250.0, WHITE);
    plot(combined.scale(0.5), -250.0, GREEN);
    draw.to_frame(app, &frame).unwrap();
}
