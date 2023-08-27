use macros::GetSet;
use nannou::{
    prelude::{vec2, Vec2},
    rand::random_f32,
};

fn random_vec2() -> Vec2 {
    vec2(random_f32(), random_f32())
}

#[derive(Debug)]
pub struct Fustrum {
    radius: f32,
}

impl Fustrum {
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Debug, GetSet)]
pub struct Body {
    position: Vec2,
    speed: Vec2,
    acceleration: Vec2,
    #[gs_ignore]
    fustrum: Fustrum,
}

impl Body {
    pub fn new_random(radius: f32) -> Self {
        Self {
            position: random_vec2(),
            speed: random_vec2(),
            acceleration: random_vec2(),
            fustrum: Fustrum::new(radius),
        }
    }

    pub fn new(radius: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            speed: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            fustrum: Fustrum::new(radius),
        }
    }

    pub fn update(&mut self, window_size: (f32, f32)) {
        let (width, height) = window_size;
        self.position += self.speed * self.acceleration();
        if self.position.x >= width / 2_f32 || self.position.x <= -width / 2_f32 {
            self.position.x %= width;

            self.speed.x *= -1_f32;
        }
        if self.position.y >= height / 2_f32 || self.position.y <= -height / 2_f32 {
            self.position.y %= height;

            self.speed.y *= -1_f32;
        }
    }
}

#[derive(Debug)]
pub struct Agent {
    body: Body,
}

impl Agent {
    pub fn new(sight_distance: f32) -> Self {
        Self {
            body: Body::new(sight_distance),
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn update(&mut self, window_size: (f32, f32)) {
        self.body.update(window_size);
    }
}
