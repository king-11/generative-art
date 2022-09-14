use std::f32::EPSILON;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vec2>,
    white_start: bool,
    orange_start: bool
}

impl Thing {
    pub fn new(p: Vec<Vec2>) -> Self {
        Self { positions: p, white_start: false, orange_start: false }
    }
}

struct Model {
    thing: Thing,
}

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();
    let thing = Thing::new(vec![vec2(0.0, 0.0)]);
    Model { thing }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 360.0;
    let last = model.thing.positions[0];
    let new_x = last.x + time;
    let new = vec2(new_x, 0.0);

    if 2.0 * new.x > SCREEN_WIDTH as f32 + EPSILON {
        return;
    }

    let ratio = 2.0 * model.thing.positions[0].x / SCREEN_WIDTH as f32;
    if ratio > 0.66 + EPSILON && !model.thing.orange_start {
        model.thing.orange_start = true;
        model.thing.positions.resize(1, last);
    }
    else if ratio > 0.33 + EPSILON && !model.thing.white_start {
        model.thing.white_start = true;
        model.thing.positions.resize(1, last);
    }

    model.thing.positions.insert(0, new);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    // draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0, 0.0, 0.0, 0.1));

    let mut color = DARKGREEN;
    let ratio = 2.0 * model.thing.positions[0].x / SCREEN_WIDTH as f32;

    if ratio > 0.66 + EPSILON {
        color = DARKORANGE;
    }
    else if ratio > 0.33 + EPSILON {
        color = WHITE;
    }


    draw.polyline()
        .stroke_weight(2.0)
        .points(model.thing.positions.iter().cloned())
        .rotate(time * TAU)
        .color(color);

    draw.to_frame(app, &frame).unwrap();
}
