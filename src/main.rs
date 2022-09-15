use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    side: u64,
}

impl Thing {
    pub fn new(side: u64) -> Self {
        Self { side }
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
    let thing = Thing::new(SCREEN_WIDTH as u64);
    Model { thing }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() / 60;

    let mut thing = &mut model.thing;
    if thing.side > time {
        thing.side -= time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames() as f32;
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    // draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0, 0.0, 0.0, 0.1));

    draw.rect()
        .rotate(time)
        .stroke_weight(5.0)
        .stroke_color(WHITE)
        .color(BLACK)
        .h(model.thing.side as f32)
        .w(model.thing.side as f32)
        .x_y(0.0, 0.0);

    draw.to_frame(app, &frame).unwrap();
}
