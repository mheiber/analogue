use analogue::{
    standard_signals::{sine_wave, square_wave},
    FrequencyHz, Signal, TimeSecs,
};
/// adapted from https://github.com/nannou-org/nannou/blob/8ebb398/examples/audio/simple_audio.rs
/// and https://github.com/nannou-org/nannou/blob/7f996a2/examples/draw/draw_mesh.rs
use nannou::prelude as n;
use nannou_audio as audio;

struct Model {
    _stream: audio::Stream<AudioModel>,
    signals: Vec<Signal>,
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

fn model(app: &n::App) -> Model {
    app.new_window().build().unwrap();
    let hz = FrequencyHz(1);
    let signals = vec![square_wave(hz), sine_wave(hz)];
    let combined = Signal::sum(signals.clone());

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
        signals,
        combined,
    }
}

fn audio(audio: &mut AudioModel, buffer: &mut audio::Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let amp = audio.combined.at(TimeSecs(audio.phase));
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = (amp * volume) as f32;
        }
    }
}

fn view(app: &n::App, model: &Model, frame: n::Frame) {
    let t = TimeSecs(app.time as f64);
    let scale = 100.0;
    let combined = model.combined.scale(0.5);
    let signals = model
        .signals
        .iter()
        .chain(std::iter::once(&combined))
        .map(|s| s.scale(scale));

    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(n::BLACK);

    let half = (win.w() as f64) / 2.0;
    let range = 1..win.w() as u32;
    let time_and_x = range.map(|right| {
        let time = t + TimeSecs(right as f64 / 100.0);
        let x = (right as f64) - half;
        (time, x)
    });
    let plot = |signal: Signal, vert_shift: f64, color: n::rgb::Srgb<u8>| {
        let colored_pts = time_and_x.clone().map(|(time, x)| {
            let amp = signal.at(time) + vert_shift;
            (n::pt2(x as f32, amp as f32), color)
        });
        draw.polyline().weight(3.0).points_colored(colored_pts);
    };
    let half_h = (win.h() / 2.0) as i32;
    let step = (win.h() / (model.signals.len() + 1) as f32) as usize;
    let vert_shifts = (-half_h..half_h).step_by(step).map(|y| y + 125).rev();

    for (vert_shift, signal) in vert_shifts.zip(signals) {
        plot(signal, vert_shift as f64, n::BLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}
