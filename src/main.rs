#![feature(clamp)]

use nannou::prelude::*;
use rand::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Particle {
    origin: Point2,
    position: Point2,
    velocity: Vector2,
}

impl Particle {
    fn new(position: Point2, velocity: Vector2) -> Self {
        Particle { origin: position, velocity, position }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn display(&self, draw: &app::Draw) {
        let d: f32 = self.position.distance2(self.origin);
        let max_d: f32 = (640 / 2).pow(2) as f32;

        draw.ellipse()
            .xy(self.position)
            .w_h(3.0, 3.0)
            .rgba(1.0, 1.0 - (d / max_d).clamp(0.0, 1.0), 0.0, 1.0);
    }
}

struct ParticleSystem {
    particles: Vec<Particle>,
    pub origin: Point2,
}

impl ParticleSystem {
    fn new(origin: Point2, max_particles: usize) -> Self {
        let particles = Vec::with_capacity(max_particles);
        ParticleSystem { origin, particles }
    }

    fn add_particle(&mut self, p: Particle) {
        if self.particles.len() == self.particles.capacity() {
            return;
        }
        self.particles.push(p);
    }

    fn update(&mut self) {
        for p in &mut self.particles {
            p.update();
        }
    }

    fn draw(&self, draw: &app::Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}

struct Model {
    window_id: nannou::window::Id,
    ps: ParticleSystem,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window()
        .with_dimensions(640, 360)
        .view(view)
        .build()
        .unwrap();

    let ps = ParticleSystem::new(pt2(0.0, 0.0), 1000);
    Model { window_id, ps }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let w = match app.window(m.window_id) {
        None => return,
        Some(w) => w,
    };

    if app.elapsed_frames() % 5 == 0 {
        let rad = (thread_rng().gen_range(0, 360) as f32).to_radians();
        m.ps.add_particle(Particle::new(m.ps.origin, vec2(rad.cos(), rad.sin())));
    }

    m.ps.update();

    for i in (0..m.ps.particles.len()).rev() {
        if !w.rect().contains(m.ps.particles[i].position) {
            m.ps.particles.remove(i);
            continue;
        }

        if thread_rng().gen_range(0, 10000) == 0 {
            for d in 0..36 {
                let rad = ((d*10) as f32).to_radians();
                m.ps.add_particle(Particle::new(m.ps.particles[i].position, vec2(rad.cos(), rad.sin())));
            }
            m.ps.particles.remove(i);
            continue;
        }
    }
}

fn view(app: &App, m: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    m.ps.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}