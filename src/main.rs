use std::rc::Rc;

use agent::Agent;

use mouse::MouseControl;
use nannou::{
    app::{Builder, ModelFn, UpdateFn, ViewFn},
    prelude::*,
};

mod agent;
mod mouse;

struct MyApp {
    controllers: Vec<Rc<dyn Controller>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            controllers: vec![],
        }
    }

    pub fn add_controller(&mut self, controller: Rc<dyn Controller>) -> &mut Self {
        self.controllers.push(controller);
        self
    }

    pub fn model(&self) -> ModelFn<MyModel> {
        |app: &App| {
            let window = app.window_rect();
            let (width, height) = window.w_h();

            let environment = Environment {
                width,
                height,
                agents: Vec::new(),
                creating_agent: None,
            };

            MyModel {
                controllers: vec![Rc::new(MouseControl)],
                environment,
            }
        }
    }

    pub fn update(&self) -> UpdateFn<MyModel> {
        |app: &App, model: &mut MyModel, update: Update| {
            model
                .environment
                .agents
                .iter_mut()
                .for_each(|ag| ag.update((model.environment.width, model.environment.height)));

            for controller in &mut model.controllers {
                Rc::get_mut(controller).and_then(|contr| {
                    contr.update(app, &mut model.environment, &update);
                    Some(())
                });
            }
        }
    }

    pub fn view(&self) -> ViewFn<MyModel> {
        |app: &App, model: &MyModel, frame: Frame| {
            model.environment.view(app, &frame);

            for controller in &model.controllers {
                controller.view(app, &model.environment, &frame);
            }
        }
    }

    pub fn app(&self) -> Builder<MyModel> {
        nannou::app(self.model())
            .update(self.update())
            .simple_window(self.view())
    }
}

pub struct Environment {
    width: f32,
    height: f32,
    agents: Vec<Agent>,
    creating_agent: Option<Agent>,
}

impl Environment {
    pub fn update(&mut self) {
        self.agents
            .iter_mut()
            .for_each(|ag| ag.update((self.width, self.height)));
    }

    pub fn view(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.background().color(BLACK);

        for agent in &self.agents {
            draw.ellipse()
                .color(WHITE)
                .xy(agent.body().position())
                .radius(10_f32);
        }

        draw.to_frame(app, frame).expect("Failed to draw");
    }
}

pub struct MyModel {
    controllers: Vec<Rc<dyn Controller>>,
    environment: Environment,
}

fn main() {
    let app = MyApp::new();
    app.app().run();
}

pub trait Controller {
    fn view(&self, app: &App, model: &Environment, frame: &Frame);
    fn update(&mut self, _app: &App, _model: &mut Environment, _update: &Update) {}
}
