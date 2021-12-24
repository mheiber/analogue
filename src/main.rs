use analogue::{FrequencyHz, Signal, TimeSecs};
mod standard_signals;
use standard_signals::{sine_wave, square_wave};

/// adapted from https://github.com/nannou-org/nannou/blob/8ebb398/examples/audio/simple_audio.rs
/// and https://github.com/nannou-org/nannou/blob/7f996a2/examples/draw/draw_mesh.rs
use nannou::prelude as n;
use nannou_audio as audio;

struct Model {
    stream: audio::Stream<AudioModel>,
    signals: Vec<Signal>,
    combined: Signal,
    mode_kind: ModeKind,
    edit_state: EditState,
    shift_key_is_down: bool,
    ctrl_key_is_down: bool,
    view_y: f32,
    view_t: f32,
}

enum ModeKind {
    AddSignal,
    Edit,
}

#[derive(Default)]
struct EditState {
    selection: usize,
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
    app.new_window().event(event).build().unwrap();
    Model::default()
}

impl Default for Model {
    fn default() -> Self {
        let hz = FrequencyHz(1);
        let signals = Vec::new();
        let combined = Signal::sum(signals.clone());

        let audio_host = audio::Host::new();

        let audio_model = AudioModel {
            phase: 0.0,
            hz: hz.0 as f64 * 440.0, // scaled to middle A for easy listening
            combined: combined.clone(),
        };

        // can be used for pause, play, etc. All is silent when the stream is `drop`ped
        let stream = audio_host
            .new_output_stream(audio_model)
            .render(audio)
            .build()
            .unwrap();

        Self {
            stream,
            signals,
            combined,
            mode_kind: ModeKind::AddSignal,
            shift_key_is_down: Default::default(),
            ctrl_key_is_down: Default::default(),
            edit_state: Default::default(),
            view_y: Default::default(),
            view_t: Default::default(),
        }
    }
}

impl Model {
    fn phase(&mut self, amt: f64) {
        let selection = self.edit_state.selection;
        self.signals[selection] = self.signals[selection].phase(amt);
    }
    fn scale(&mut self, amt: f64) {
        let selection = self.edit_state.selection;
        self.signals[selection] = self.signals[selection].scale(amt);
    }
    fn incr_frequency(&mut self, amt: f64) {
        let selection = self.edit_state.selection;
        self.signals[selection] = self.signals[selection].incr_frequency(amt);
    }
    fn incr_selection(&mut self, amt: i32) {
        let selection = self.edit_state.selection as i32;
        let len = self.signals.len() as i32;
        self.edit_state.selection = (selection + amt).rem_euclid(len) as usize;
    }
    fn push_signal_and_goto_edit_mode(&mut self, signal: Signal) {
        self.signals.push(signal);
        self.mode_kind = ModeKind::Edit;
        self.edit_state.selection = self.signals.len() - 1;
    }
    fn remove_signal(&mut self) {
        if !self.signals.is_empty() {
            let selection = self.edit_state.selection as usize;
            self.signals.remove(selection);
            let len = self.signals.len();
            if len == 0 {
                self.goto_add_mode();
            } else if self.edit_state.selection == len {
                self.edit_state.selection = len - 1
            }
        }
    }
    fn goto_add_mode(&mut self) {
        self.mode_kind = ModeKind::AddSignal;
    }
}

fn event(_app: &n::App, model: &mut Model, event: n::WindowEvent) {
    use n::Key::*;
    use n::WindowEvent::*;

    if model.signals.is_empty() {
        model.goto_add_mode()
    }
    let shift = &mut model.shift_key_is_down;
    let ctrl = &mut model.ctrl_key_is_down;
    let gentle: f64 = if *ctrl { 0.1 } else { 1.0 };
    match &model.mode_kind {
        ModeKind::Edit => match event {
            KeyPressed(H) if *shift => model.view_t -= 30.0 * gentle as f32,
            KeyPressed(H) => model.phase(-0.2 * gentle),
            KeyPressed(L) if *shift => model.view_t += 30.0 * gentle as f32,
            KeyPressed(L) => model.phase(0.2 * gentle),
            KeyPressed(U) if *shift => model.scale(1.0 + 0.25 * gentle),
            KeyPressed(D) if *shift => model.scale(1.0 / (1.0 + 0.25 * gentle)),
            KeyPressed(U) => model.incr_frequency(1.0 + 0.25 * gentle),
            KeyPressed(D) => model.incr_frequency(1.0 / (1.0 + 0.25 * gentle)),

            KeyPressed(J) if *shift => model.view_y -= 20.0 * gentle as f32,
            KeyPressed(J) => model.incr_selection(1),
            KeyPressed(K) if *shift => model.view_y += 20.0 * gentle as f32,
            KeyPressed(K) => model.incr_selection(-1),
            KeyPressed(X) => model.remove_signal(),
            KeyPressed(O) => model.goto_add_mode(),
            KeyPressed(LShift | RShift) => *shift = true,
            KeyPressed(LControl | RControl) => *ctrl = true,
            KeyPressed(Slash) if *shift /* question mark*/ => model.goto_add_mode(),
            KeyReleased(LShift | RShift) => *shift = false,
            KeyReleased(LControl | RControl) => *ctrl = false,
            _ => (),
        },
        ModeKind::AddSignal => match event {
            KeyPressed(Key1) => model.push_signal_and_goto_edit_mode(square_wave(FrequencyHz(1))),
            KeyPressed(Key2) => model.push_signal_and_goto_edit_mode(sine_wave(FrequencyHz(1))),
            KeyPressed(O) => model.push_signal_and_goto_edit_mode(sine_wave(FrequencyHz(1))),
            _ => (),
        },
    }
    model.combined = Signal::sum(model.signals.clone());
    let combined = model.combined.clone();
    model
        .stream
        .send(|mut audio_model| audio_model.combined = combined)
        .unwrap();
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
    match &model.mode_kind {
        ModeKind::Edit => view_edit_signals(app, model, frame),
        ModeKind::AddSignal => view_add_signal(app, model, frame),
    }
}

fn view_edit_signals(app: &n::App, model: &Model, frame: n::Frame) {
    let t = TimeSecs((app.time + model.view_t) as f64);
    let selection = model.edit_state.selection;
    let scale = 100.0 + selection as f64 * 5.0;
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
            (n::pt2(x as f32, amp as f32 + model.view_y), color)
        });
        draw.polyline().weight(3.0).points_colored(colored_pts);
    };
    let half_h = (win.h() / 2.0) as i32;
    let step = (win.h() / (model.signals.len() + 1) as f32) as usize;
    let vert_shifts = (-half_h..half_h).step_by(step).map(|y| y + 125).rev();

    for (i, (vert_shift, signal)) in vert_shifts.zip(signals).enumerate() {
        let is_selected = i as usize == selection;
        let color = if i == model.signals.len() {
            n::AQUA
        } else if is_selected {
            n::YELLOW
        } else {
            n::BLUE
        };
        plot(signal, vert_shift as f64, color);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn view_add_signal(app: &n::App, _model: &Model, frame: n::Frame) {
    let draw = app.draw();
    draw.background().color(n::BLACK);
    draw.text(
        r#"
        "Make a selection:
        (1) add square wave
        (2) add sine wave

        after that, experiment with keys:
            h, j, k, and l
            u and d
            x and o
            Hold Shift with another key
            Hold Ctrl with another key

--Then press o to return to this menu--"#,
    )
    .left_justify()
    .font_size(24)
    .width(app.window_rect().w() / 2.0);
    draw.to_frame(app, &frame).unwrap();
}
