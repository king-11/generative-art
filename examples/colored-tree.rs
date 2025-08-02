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
}

impl Line {
    pub fn new(p: Vec2) -> Self {
        Self {
            points: vec![p],
            length: 0.0,
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
const COLOR: f32 = 200.0;
const STEP: f32 = SCREEN_WIDTH as f32 * 0.01;

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


        let d_change = model.noise.get([(last.x * SN) as f64, (last.y * SN) as f64]) as f32 * PI;

        let map_d_change = map_range(d_change, -1.0, 1.0, -PI, PI);

        let new = *last + vec2(
            map_d_change.cos() * STEP,
            map_d_change.sin() * STEP
        );

        line.length +=
            f32::sqrt((last.x - new.x) * (last.x - new.x) + (last.y - new.y) * (last.y - new.y));
        line.points.push(new);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(srgba(0.0, 0.0, 0.0, 0.8));
    }
    // draw.rect().w_h(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).color(srgba(0.0, 0.0, 0.0, 0.1));

    for line in model.lines.iter() {
        let mut line_length = 0.0;
        let items = line.points.len();
        for idx in 0..=items - 2 {
            let (current, next) = (line.points[idx], line.points[idx + 1]);

            line_length += f32::sqrt(
                (current.x - next.x) * (current.x - next.x)
                    + (current.y - next.y) * (current.y - next.y),
            );

            let sat = 3.0 * (line.length - line_length) / line.length;
            let hue = (COLOR + 130.0 * (SCREEN_HEIGHT as f32 - current.y) / SCREEN_HEIGHT as f32) % 360.0 / 360.0;

            draw.line().points(current, next).hsva(hue, sat, 1.0, 0.2).weight(2.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
