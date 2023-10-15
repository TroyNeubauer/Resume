use yew::agent::{Dispatched, Dispatcher};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::date_range::DateRangeComponent;
use crate::location::LocationComponent;
use crate::protos::{Duty, Experience};
use crate::tag_agent::{TagAgent, TagUpdate};

#[derive(Clone, Properties, PartialEq)]
pub struct ExperienceProps {
    pub experience: Vec<Experience>,
}

pub enum Msg {
    Hover(Duty),
    Clear,
}

pub struct ExperienceComponent {
    props: ExperienceProps,
    hovered: Option<Duty>,
    tag_agent: Dispatcher<TagAgent>,
    link: ComponentLink<Self>,
}

impl Component for ExperienceComponent {
    type Message = Msg;
    type Properties = ExperienceProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            hovered: None,
            tag_agent: TagAgent::dispatcher(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let tags = match msg {
            Msg::Hover(duty) => {
                let tags = duty.tags.to_vec();
                self.hovered = Some(duty);
                tags
            }
            Msg::Clear => {
                self.hovered = None;
                Vec::new()
            }
        };
        self.tag_agent.send(TagUpdate::SetTags(tags));
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2>{ "EXPERIENCE" }</h2>
            <ul class="experience-list">
                { for self.props.experience.iter().map(|edu| self.view_entry(edu)) }
            </ul>
            </>
        }
    }
}

impl ExperienceComponent {
    fn view_entry(&self, exp: &Experience) -> Html {
        let period = exp.period.clone();
        let location = exp.parsed_location.clone().unwrap();
        let link = if let Some(site) = &exp.website {
            html! { <h4><a href=site.clone()>{ &exp.organization }</a></h4> }
        } else {
            html! { <h4><a>{ &exp.organization }</a></h4> }
        };

        html! {
            <li>
                <h3>{ &exp.title }</h3>
                { link }
                <div class="detail">
                    <span class="detail-date"><DateRangeComponent period=period/></span>
                    <span class="detail-loc"><LocationComponent location=location/></span>
                </div>
                <ul class="duty-list">
                    { for exp.duty.iter().map(|duty| self.view_duty(duty)) }
                </ul>
            </li>
        }
    }

    fn view_duty(&self, duty: &Duty) -> Html {
        let send_duty = duty.clone();
        let mouseover = self
            .link
            .callback(move |_| Msg::Hover(send_duty.to_owned()));
        let mouseout = self.link.callback(|_| Msg::Clear);
        let mut class = "duty-item";
        if let Some(hov_duty) = &self.hovered {
            if hov_duty == duty {
                class = "duty-item duty-selected"
            }
        }
        html! {
            <li class=class onmouseover=mouseover onmouseout=mouseout>
                { &duty.description }
            </li>
        }
    }
}
