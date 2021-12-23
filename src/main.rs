/// adapted from https://github.com/nannou-org/nannou/blob/8ebb398/examples/audio/simple_audio.rs
/// and https://github.com/nannou-org/nannou/blob/7f996a2/examples/draw/draw_mesh.rs
use analogue::*;
use nannou::prelude::*;
use nannou_audio as audio;

struct Model {
    _stream: audio::Stream<AudioModel>,
    square: Signal,
    sine: Signal,
    combined: Signal,
}

struct AudioModel {
    phase: f64,
    hz: f64,
    combined: Signal,
}

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window().build().unwrap();
    let hz = FrequencyHz(1);
    let sine = sine_wave(hz);
    let square = square_wave(hz);
    let combined = sine.clone() + square.clone();

    let audio_host = audio::Host::new();

    let audio_model = AudioModel {
        phase: 0.0,
        hz: hz.0 as f64 * 440.0, // scaled to middle A for easy listening
        combined: combined.clone(),
    };

    // can be used for pause, play, etc. All is silent when the stream is `drop`ped
    let _stream = audio_host
        .new_output_stream(audio_model)
        .render(audio)
        .build()
        .unwrap();

    Model {
        _stream,
        sine,
        square,
        combined,
    }
}

fn audio(audio: &mut AudioModel, buffer: &mut audio::Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let amp = audio.combined.at(TimeSecs(audio.phase as f32));
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = amp * volume;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = TimeSecs(app.time);
    let scale = 100.0;
    let square = model.square.clone().scale(scale);
    let sine = model.sine.clone().scale(scale);
    let combined = model.combined.clone().scale(scale);

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
