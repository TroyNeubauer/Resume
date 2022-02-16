use std::string::ToString;

use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::resume::DateRange;

pub trait DateFormat {
    fn format_month(&self) -> String;
}

impl DateFormat for crate::protos::resume::Date {
    fn format_month(&self) -> String {
        self.0.format("%b %Y").to_string()
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct DateRangeProps {
    pub period: DateRange,
}

pub struct DateRangeComponent {
    props: DateRangeProps,
}

impl Component for DateRangeComponent {
    type Message = ();
    type Properties = DateRangeProps;

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
        let start = self.props.period.start.format_month();
        let end = match self.props.period.end {
            Some(end) => end.format_month(), 
            None => "Present".to_string()
        };
        html! {
            <>
            <i class="far fa-calendar-alt"></i>{ start }{ " – "}{ end }
            </>
        }
    }
}
