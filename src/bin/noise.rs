use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Agent {
    position: Vector2,
    acceleration: Vector2,
}

impl Agent {
    fn new(win_rect: Rect) -> Self {
        let position = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),
        );

        Agent {
            position,
            acceleration: position.normalize(),
        }
    }

    fn update(&mut self, noise: Perlin) {
        let n = noise.get([
            self.position.x as f64 / 100.0,
            self.position.y as f64 / 100.0,
        ]) * 4.0;
        self.acceleration = self.acceleration.rotate(n as f32);
        self.position += self.acceleration;
    }

    fn display(&self, draw: &Draw) {
        draw.rect()
            .xy(self.position)
            .w_h(2.0, 2.0)
            .rgba(0.0, 0.0, 0.0, 1.0);
    }
}

struct Model {
    agents: Vec<Agent>,
    noise_seed: u32,
}

fn model(app: &App) -> Model {
    app.new_window().size(500, 500).view(view).build().unwrap();

    let agent_count = 4000;
    let agents = (0..agent_count)
        .map(|_| Agent::new(app.window_rect()))
        .collect();

    Model {
        agents,
        noise_seed: 12,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = Perlin::new().set_seed(model.noise_seed);

    for agent in &mut model.agents {
        agent.update(noise);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(1.0, 1.0, 1.0, 0.03);

    model.agents.iter().for_each(|agent| {
        agent.display(&draw);
    });

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
