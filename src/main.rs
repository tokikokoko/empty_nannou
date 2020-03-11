use nannou::prelude::*;

struct Model {}

fn main() {
    println!("Hello nannou!");
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
}