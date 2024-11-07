#![recursion_limit = "512"]

mod date_range;
mod education;
mod experience;
mod location;
mod phone_number;
mod resume;
mod skills;
mod tag_agent;
mod types;

use std::{collections::HashSet, rc::Rc};

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use resume::ResumeComponent;
pub use types::*;

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
            "Unknown city `{city}. Available options are: {:?}`",
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

    let all_referenced_tags: HashSet<&str> = base
        .experience
        .iter()
        .map(|e| e.duty.iter())
        .flatten()
        .map(|d| d.tags.iter())
        .flatten()
        .map(std::ops::Deref::deref)
        .collect();

    let all_defined_tags: HashSet<&str> = base
        .skills
        .iter()
        // Awards wont have any reference tags in experience, so dont count them
        .filter(|s| s.category != "Awards")
        .map(|e| e.tags.iter())
        .flatten()
        .map(std::ops::Deref::deref)
        .collect();

    let undefined = &all_referenced_tags - &all_defined_tags;
    if !undefined.is_empty() {
        return Err(format!("Undefined tags: {:?}", undefined));
    }

    let unused = &all_defined_tags - &all_referenced_tags;
    if !unused.is_empty() {
        return Err(format!("Unused tags: {:?}", unused));
    }

    // Remove archived tags from the left side
    let non_archived_referenced_tags: HashSet<&str> = base
        .experience
        .iter()
        .filter(|e| !e.archived)
        .map(|e| e.duty.iter())
        .flatten()
        .map(|d| d.tags.iter())
        .flatten()
        .map(std::ops::Deref::deref)
        .collect();

    let archived_tags = &all_referenced_tags - &non_archived_referenced_tags;

    for category in &mut base.skills {
        category
            .tags
            .retain(|tag| !archived_tags.contains(tag.as_str()));
    }

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
