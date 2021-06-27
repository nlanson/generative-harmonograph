use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    //Draw background
    let draw = app.draw();
    draw.background().color(PLUM);
    
    //Initialise equation parameters.
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

    let points = (0..100).map(|i| {
        let point = pt2(x*100.0, y*100.0);
        step(
        &mut x, &mut y, &mut t, 
        &ampliX1, &ampliX2, &ampliY1, &ampliY2,
        &freqX1, &freqX2, &freqY1, &freqY2,
        &phaseX1, &phaseX2, &phaseY1, &phaseY2,
        &dampX1, &dampX2, &dampY1, &dampY2
        );
        (point, STEELBLUE)
    });
    draw.polyline()
        .weight(3.0)
        .points_colored(points);
    
    draw.to_frame(app, &frame).unwrap();
}

fn step(
    x: &mut f32,
    y: &mut f32,
    t: &mut f32,
    ampliX1:&f32, ampliX2:&f32, ampliY1:&f32, ampliY2:&f32,
    freqX1:&f32, freqX2:&f32, freqY1:&f32, freqY2:&f32,
    phaseX1:&f32, phaseX2:&f32, phaseY1:&f32, phaseY2:&f32, 
    dampX1:&f32, dampX2:&f32, dampY1:&f32, dampY2:&f32
) {
    *x = eq(*t, ampliX1, freqX1, phaseX1, dampX1) - eq(*t, ampliX2, freqX2, phaseX2, dampX2);
    *y = eq(*t, ampliY1, freqY1, phaseY1, dampY1) - eq(*t, ampliY2, freqY2, phaseY2, dampY2);
    
    *t += 0.02;
}

fn eq(t:f32, ampli:&f32, freq:&f32, phase:&f32, damp:&f32) -> f32 {
    ampli * f32::sin(t * freq + phase) * f32::exp(-damp * t)
}