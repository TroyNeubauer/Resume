use std::collections::HashSet;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

#[derive(Debug)]
pub enum TagUpdate {
    SetTags(Vec<String>),
}

pub struct TagAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for TagAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = TagUpdate;
    type Output = Vec<String>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            TagUpdate::SetTags(tags) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, tags.clone())
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

#[derive(Debug)]
pub enum SkillUpdate {
    SetSkill(String),
}

pub struct SkillAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for SkillAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = SkillUpdate;
    type Output = String;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            SkillUpdate::SetSkill(skill) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, skill.clone())
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
