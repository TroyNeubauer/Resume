use yew::prelude::*;
use yewtil::NeqAssign;

use crate::date_range::DateRangeComponent;
use crate::location::LocationComponent;
use crate::protos::{DegreeKind, Education};

impl std::string::ToString for DegreeKind {
    fn to_string(&self) -> String {
        match self {
            DegreeKind::Bachelors => "Bachelor of Science".to_string(),
            DegreeKind::Masters => "Master of Sciences".to_string(),
            DegreeKind::NonDegree => "Non-Degree".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct EducationProps {
    pub education: Vec<Education>,
}

pub struct EducationComponent {
    props: EducationProps,
}

impl Component for EducationComponent {
    type Message = ();
    type Properties = EducationProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "EDUCATION" }</h2>
            <ul class="edu-list">
                { for self.props.education.iter().map(|edu| self.view_entry(edu)) }
            </ul>
            </>
        }
    }
}

impl EducationComponent {
    fn view_entry(&self, edu: &Education) -> Html {
        let title = match edu.degree {
            DegreeKind::NonDegree => "Non-Degree".to_owned(),
            _ => format!("{}: {}", edu.degree.to_string(), edu.major),
        };
        let period = &edu.period;
        let location = edu.parsed_location.clone().unwrap();
        let desc = match &edu.description {
            x if x != "" => html! { <p>{ format!("{}", x) }</p> },
            _ => html! {},
        };
        html! {
            <li>
                <div class="edu-view">
                    <h3>{ title }</h3>
                    <h4>{ &edu.institution }</h4>
                    <div class="detail">
                        <span class="detail-date"><DateRangeComponent period=period/></span>
                        <span class="detail-loc"><LocationComponent location=location/></span>
                    </div>
                    <div class="edu-spec">{ desc }</div>
                </div>
            </li>
        }
    }
}
