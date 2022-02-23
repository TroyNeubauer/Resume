#![recursion_limit = "512"]

mod date_range;
mod education;
mod experience;
mod location;
mod phone_number;
mod resume;
mod resume_ui;
mod skills;
mod tag_agent;

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use resume::Resume;
use resume_ui::ResumeComponent;

const RESUME_YAML: &str = include_str!("../resume.yaml");

enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct ModelProps {
    pub resume: Rc<Resume>,
}

struct Model {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
    props: ModelProps,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ModelProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <ResumeComponent resume=self.props.resume.clone() />
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    let resume = load_resume().unwrap();
    let props = ModelProps {
        resume: Rc::new(resume),
    };
    App::<Model>::new().mount_to_body_with_props(props);
}

pub fn load_resume() -> Result<Resume, serde_yaml::Error> {
    serde_yaml::from_str(RESUME_YAML)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_resume() {
        let _ = load_resume().unwrap();
    }
}
