#![recursion_limit = "512"]

mod date_range;
mod education;
mod experience;
mod location;
mod phone_number;
mod protos;
mod resume;
mod skills;
mod tag_agent;

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use protos::Resume;
use resume::ResumeComponent;

use crate::protos::Location;

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
    let resume = load_resume().unwrap();
    let props = ModelProps {
        resume: Rc::new(resume),
    };
    App::<Model>::new().mount_to_body_with_props(props);
}

const RESUME_DATA: &'static str = include_str!("../resume_data.yaml");

pub fn load_resume() -> Result<Resume, String> {
    let mut base = serde_yaml::from_str::<Resume>(RESUME_DATA).map_err(|e| format!("{e:?}"))?;
    let locations = &base.locations;
    fn lookup_location(locations: &[Location], city: &str) -> Result<Location, String> {
        for location in locations {
            if location.city == city {
                return Ok(location.clone());
            }
        }
        Err(format!(
            "Failed to find city `{city} out of options: {:?}`",
            locations.iter().map(|l| &l.city).collect::<Vec<_>>()
        ))
    }
    for exp in &mut base.experience {
        exp.parsed_location = Some(lookup_location(locations, &exp.location)?);
        for duty in &mut exp.duty {
            duty.tags.extend_from_slice(&exp.tags);
        }
    }
    for edu in &mut base.education {
        edu.parsed_location = Some(lookup_location(locations, &edu.location)?);
    }
    base.parsed_location = Some(lookup_location(locations, &base.location)?);

    dbg!(&base);
    Ok(base)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_resume() {
        dbg!(load_resume().unwrap());
    }
}
