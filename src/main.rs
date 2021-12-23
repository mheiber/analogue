use analogue::*;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let t = TimeSecs(app.time);
    let square = square_wave(FrequencyHz(1)).scale(100.0);
    let sine = sine_wave(FrequencyHz(1)).scale(100.0);

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
    let sine_plot = time_and_x.clone().map(|(time, x)| {
        let amp = sine.at(time);
        (pt2(x, amp), BLUE)
    });
    let square_plot = time_and_x.clone().map(|(time, x)| {
        let amp = square.at(time) + 250.0;
        (pt2(x, amp), WHITE)
    });
    let combined_plot = time_and_x.clone().map(|(time, x)| {
        let amp = ((square.at(time) + sine.at(time)) / 2.0) - 250.0;
        (pt2(x, amp), GREEN)
    });
    draw.polyline().weight(3.0).points_colored(sine_plot);
    draw.polyline().weight(3.0).points_colored(square_plot);
    draw.polyline().weight(3.0).points_colored(combined_plot);
    draw.to_frame(app, &frame).unwrap();
}
