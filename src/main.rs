use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    t: f32,
    x: f32,
    y:f32,
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
    //Initialise equation parameters and store into model state.
    let mut rng = rand::thread_rng();

    let mut t: f32 = 0.0;
    let mut x: f32 = t;
    let mut y: f32 = t;
    let DT: f32 = 0.02;
    
    let ampliX1: f32 = rng.gen_range(0.0..1.0);
    let ampliX2: f32 = 1.0 - ampliX1;
    let ampliY1: f32 = rng.gen_range(0.0..1.0);
    let ampliY2: f32 = 1.0 - ampliY1;

    let freqX1: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqX2: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqY1: f32 = 1.0 + rng.gen_range(0.0..6.0);
    let freqY2: f32 = 1.0 + rng.gen_range(0.0..6.0);

    let phaseX1: f32 = 0.0;
    let phaseX2: f32 = rng.gen_range(0.0..12.0) * (2.0 * std::f32::consts::PI) / 12.0;
    let phaseY1: f32 = rng.gen_range(0.0..12.0) * (2.0 * std::f32::consts::PI) / 12.0;
    let phaseY2: f32 = rng.gen_range(0.0..12.0) * (2.0 * std::f32::consts::PI) / 12.0;

    let dampX1:f32 = rng.gen_range(0.005..0.01);
    let dampX2:f32 = rng.gen_range(0.005..0.01);
    let dampY1:f32 = rng.gen_range(0.005..0.01);
    let dampY2:f32 = rng.gen_range(0.005..0.01);
    
    let mut model = Model { 
        t, x, y,
        ampliX1, ampliX2, ampliY1, ampliY2,
        freqX1, freqX2, freqY1, freqY2,
        phaseX1, phaseX2, phaseY1, phaseY2,
        dampX1, dampX2, dampY1, dampY2
    };

    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) { }

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();

    //Draw background
    let draw = app.draw();
    draw.background().color(PLUM);
    
    println!("Amplis:\nampliX1 = {};\nampliX2 = {};\nampliY1 = {};\nampliY2 = {}", _model.ampliX1, _model.ampliX2, _model.ampliY1, _model.ampliY2);

    let points = (0..100).map(|_i| {
        let point = pt2(_model.x*100.0, _model.y*100.0);
        step( _model ); //Cant send &mut _model?
        (point, STEELBLUE)
    });
    draw.polyline()
        .weight(3.0)
        .points_colored(points);
    
    draw.to_frame(app, &frame).unwrap();
}

fn step(
    _model: &mut Model
) {
    _model.x = eq(_model.t, _model.ampliX1, _model.freqX1, _model.phaseX1, _model.dampX1) - eq(_model.t, _model.ampliX2, _model.freqX2, _model.phaseX2, _model.dampX2);
    _model.y = eq(_model.t, _model.ampliY1, _model.freqY1, _model.phaseY1, _model.dampY1) - eq(_model.t, _model.ampliY2, _model.freqY2, _model.phaseY2, _model.dampY2);
    
    _model.t += 0.02;
}

fn eq(t:f32, ampli:f32, freq:f32, phase:f32, damp:f32) -> f32 {
    ampli * f32::sin(t * freq + phase) * f32::exp(-damp * t)
}