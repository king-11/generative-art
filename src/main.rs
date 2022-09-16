use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Line {
    points: Vec<Vec2>,
    length: f32,
    color: Srgba
}

impl Line {
    pub fn new(p: Vec2) -> Self {
        Self {
            points: vec![p],
            length: 0.0,
            color: srgba(random_f32(), random_f32(), random_f32(), random_f32())
        }
    }
}

struct Model {
    lines: Vec<Line>,
    noise: Perlin,
}

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 1024;
const MAX_LENGTH: f32 = SCREEN_WIDTH as f32 * 2.0;
const LINES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) / 1000;
const SN: f32 = 0.001;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .build()
        .unwrap();

    let lines = (0..LINES)
        .map(|_| {
            let point = vec2(
                (random::<f32>() - 0.5) * SCREEN_WIDTH as f32,
                (random::<f32>() - 0.5) * SCREEN_HEIGHT as f32,
            );

            Line::new(point)
        })
        .collect();

    let noise = Perlin::new();
    Model { lines, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for line in model.lines.iter_mut() {
        let last = line.points.last().unwrap();
        if line.length > MAX_LENGTH  {
            continue;
        }

        let d_change = model.noise.get([(last.x * SN) as f64, (last.y * SN) as f64, 0.0]) as f32 * PI;

        let map_d_change = map_range(d_change, -1.0, 1.0, -360.0, 360.0);

        let new = *last + vec2(
            map_d_change.cos(),
            map_d_change.sin()
        );

        line.length +=
            f32::sqrt((last.x - new.x) * (last.x - new.x) + (last.y - new.y) * (last.y - new.y));
        line.points.push(new);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    // draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0, 0.0, 0.0, 0.1));

    for line in model.lines.iter() {
        if line.length > MAX_LENGTH {
            continue;
        }
        draw.polyline()
            .weight(3.0)
            .points(line.points.iter().cloned())
            .color(line.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
