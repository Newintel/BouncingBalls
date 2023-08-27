use nannou::{
    prelude::{Update, Vec2, GRAY, RED, WHITE},
    App, Frame,
};

use crate::{agent::Agent, Controller, Environment};

pub struct MouseControl;

impl Controller for MouseControl {
    fn view(&self, app: &App, model: &Environment, frame: &Frame) {
        let draw = app.draw();
        let mouse = &app.mouse;
        let drawing = draw.ellipse().xy(mouse.position()).wh(Vec2::splat(10_f32));

        if let Some(agent) = &model.creating_agent {
            let position = agent.body().position();
            draw.ellipse().color(RED).xy(position).radius(10_f32);

            drawing.color(RED);

            draw.arrow()
                .start(position)
                .end(mouse.position())
                .color(WHITE);
        } else {
            drawing.color(GRAY);
        }

        draw.to_frame(app, frame).expect("Failed to draw");
    }

    fn update(&mut self, app: &App, model: &mut Environment, _update: &Update) {
        let mouse = &app.mouse;
        if mouse.buttons.left().is_down() && model.creating_agent.is_none() {
            let mut agent = Agent::new(50_f32);
            agent.body_mut().set_position(mouse.position());
            model.creating_agent.replace(agent);
        } else if mouse.buttons.left().is_up() {
            if let Some(mut agent) = model.creating_agent.take() {
                let position = agent.body().position();
                agent
                    .body_mut()
                    .set_speed((mouse.position() - position).normalize())
                    .set_acceleration(Vec2::splat(mouse.position().distance(position)));
                model.agents.push(agent);
            }
        }
    }
}
