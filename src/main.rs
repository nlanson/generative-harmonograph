#![allow(non_snake_case)]
use nannou::prelude::*;
use rand::Rng;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    t: f32,
    x: f32,
    y: f32,
    oldX: f32,
    oldY: f32,
    ampliX1: f32,
    ampliX2: f32,
    ampliY1: f32,
    ampliY2: f32,
    freqX1: f32,
    freqX2: f32,
    freqY1: f32,
    freqY2: f32,
    phaseX1:f32,
    phaseX2: f32,
    phaseY1: f32,
    phaseY2: f32,
    dampX1: f32,
    dampX2: f32,
    dampY1: f32,
    dampY2: f32
}

fn model(app: &App) -> Model {
    //Window Element
    let _window = app.new_window().view(view).build().unwrap();
    
    //RNG Param Init
    let mut rng = rand::thread_rng();

    let t: f32 = 0.0;
    let x: f32 = t;
    let y: f32 = t;
    let oldX: f32 = x;
    let oldY: f32 = y;

    let ampliX1: f32 = rng.gen_range(0.0..1.0);
    let ampliX2: f32 = 1.0 - ampliX1;
    let ampliY1: f32 = rng.gen_range(0.0..1.0);
    let ampliY2: f32 = 1.0 - ampliY1;

    let freqX1: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqX2: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqY1: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqY2: f32 = 1.0 + rng.gen_range(0.0..6.0);

    let phaseX1: f32 = 0.0;
    let phaseX2: f32 = rng.gen_range(0.0..12.0) * 6.2831855 / 12.0;
    let phaseY1: f32 = rng.gen_range(0.0..12.0) * 6.2831855 / 12.0;
    let phaseY2: f32 = rng.gen_range(0.0..12.0) * 6.2831855 / 12.0;

    let dampX1:f32 = rng.gen_range(0.005..0.01);
    let dampX2:f32 = rng.gen_range(0.005..0.01);
    let dampY1:f32 = rng.gen_range(0.005..0.01);
    let dampY2:f32 = rng.gen_range(0.005..0.01);
    
    let mut model = Model { 
        _window,
        t, x, y, oldX, oldY,
        ampliX1, ampliX2, ampliY1, ampliY2,
        freqX1, freqX2, freqY1, freqY2,
        phaseX1, phaseX2, phaseY1, phaseY2,
        dampX1, dampX2, dampY1, dampY2
    };

    step(&mut model);

    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    step(_model);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    //Draw background
    let draw = app.draw();

    let start_point = pt2(_model.oldX*250.0, _model.oldY*250.0);
    let end_point = pt2(_model.x*250.0, _model.y*250.0);
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(0.5)
        .color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();

    //println!("old: {} | new: {}", start_point, end_point);
}

fn step(
    _model: &mut Model
) {
    _model.oldX = _model.x;
    _model.oldY = _model.y;
    
    _model.x = eq(_model.t, _model.ampliX1, _model.freqX1, _model.phaseX1, _model.dampX1) + eq(_model.t, _model.ampliX2, _model.freqX2, _model.phaseX2, _model.dampX2);
    _model.y = eq(_model.t, _model.ampliY1, _model.freqY1, _model.phaseY1, _model.dampY1) + eq(_model.t, _model.ampliY2, _model.freqY2, _model.phaseY2, _model.dampY2);
    
    _model.t += 0.02;
}

fn eq(t:f32, ampli:f32, freq:f32, phase:f32, damp:f32) -> f32 {
    (ampli) * (f32::sin((t * freq) + phase)) * (f32::pow(std::f32::consts::E, -damp * t))
}