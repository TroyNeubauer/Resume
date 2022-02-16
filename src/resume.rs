use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::education::EducationComponent;
use crate::experience::ExperienceComponent;
use crate::location::LocationComponent;
use crate::phone_number::PhoneNumberComponent;
use crate::protos::resume::Resume;
use crate::skills::SkillComponent;

#[derive(Clone, Properties, PartialEq)]
pub struct ResumeProps {
    pub resume: Rc<Resume>,
}

pub enum Msg {
    AmHover,
    Clear,
}

pub struct ResumeComponent {
    props: ResumeProps,
    link: ComponentLink<Self>,
    am_hover: bool,
}

impl Component for ResumeComponent {
    type Message = Msg;
    type Properties = ResumeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ResumeComponent {
            props,
            link,
            am_hover: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AmHover => self.am_hover = true,
            Msg::Clear => self.am_hover = false,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let res = &self.props.resume;
        let github = format!("github.com/{}", res.github_profile);
        let linkedin = format!("linkedin.com/in/{}", res.linkedin_profile);
        let phone = res.phone_number.clone();
        let location = res.location.clone();
        let education = res.education.to_vec();
        let experience = res.experience.to_vec();
        let skills = res.skills.to_vec();

        let am_class = if self.am_hover { "am-hover" } else { "am" };
        let on_hover = self.link.callback(|_| Msg::AmHover);
        let on_clear = self.link.callback(|_| Msg::Clear);
        html! {
            <div class="content">
                <header class="main-header">
                    <h1 class="main-header-name">{ res.name.to_ascii_uppercase() }</h1>
                    <ul class="main-header-list">
                        <li><i class="fas fa-envelope"></i>{ &res.email }</li>
                        <li><PhoneNumberComponent phone=phone /></li>
                        <li><a href=format!("https://{}", &github)>
                            <i class="fab fa-github"></i>{ github }</a></li>
                        <li><a href=format!("https://{}", &linkedin)>
                            <i class="fab fa-linkedin-in"></i>{ linkedin }</a></li>
                        <li><LocationComponent location=location /></li>
                    </ul>
                </header>
                <div class="main-column main-left">
                    <EducationComponent education=education />
                    <SkillComponent skills=skills/>
                    { self.view_links() }
                </div>
                <div class="main-column main-right">
                    <h2>{ "ABOUT ME"}</h2>
                    <div class="about-me">
                        <p class=am_class onmouseover=on_hover onmouseout=on_clear>
                            { &res.about_me }
                        </p>
                    </div>
                    <ExperienceComponent experience=experience/>
                </div>
            </div>
        }
    }
}

impl ResumeComponent {
    fn view_links(&self) -> Html {
        let res = &self.props.resume;
        let source_code = &res.source_code;
        let source_code_https = format!("https://{}", source_code);
        let host_link = &res.host_link;
        let pdf_name = format!("{}-Resume.pdf", res.name.replace(" ", ""));
        html! {
            <div class="links">
                <h2>{ "LINKS" }</h2>
                <ul class="links-list">
                    <li class="screen-only">
                        <a href=pdf_name>
                            <i class="fa fa-external-link" aria-hidden="true"></i>
                            { "Download a PDF of this resume" }
                        </a>
                    </li>
                    <li class="screen-only">
                        <a href=source_code_https>
                            <i class="fa fa-external-link" aria-hidden="true"></i>
                            { "View the source code" }
                        </a>
                    </li>
                    <li class="print-only">
                        { "View this resume as a WebAssembly app:" }
                        <p>
                            <i class="fa fa-external-link" aria-hidden="true"></i>
                            { host_link }
                        </p>
                    </li>
                    <li class="print-only">
                        { "View the source code:" }
                        <p>
                            <i class="fa fa-external-link" aria-hidden="true"></i>
                            { source_code }
                        </p>
                    </li>
                </ul>
            </div>
        }
    }
}
