use nannou::{prelude::*, noise::{Perlin, NoiseFn}};

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec<Vec2>) -> Self {
        Self { positions: p }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin
}

const N_THINGS: usize = 1000;
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();
    let mut things = vec![];

    for _ in 0..N_THINGS {
        things.push(Thing::new(vec![vec2(
            (random_f32() - 0.5) * SCREEN_WIDTH as f32,
            (random_f32() - 0.5) * SCREEN_HEIGHT as f32,
        )]));
    }
    let noise = Perlin::new();
    Model { things, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let sn = 0.01;
    for thing in model.things.iter_mut() {
        let last = thing.positions[0];
        let new = last + vec2(
            model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
            model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32
        );
        thing.positions.insert(0, new);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(BISQUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
