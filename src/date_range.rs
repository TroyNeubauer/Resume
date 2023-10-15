use std::string::ToString;

use yew::prelude::*;
use yewtil::NeqAssign;

use crate::protos::DateRange;

fn format_month(date: &chrono::NaiveDate) -> String {
    date.format("%b %Y").to_string()
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
        let start = format_month(&self.props.period.start);
        let end = if let Some(end) = &self.props.period.end {
            format_month(end)
        } else {
            "Present".to_string()
        };
        html! {
            <>
            <i class="far fa-calendar-alt"></i>{ start }{ " – "}{ end }
            </>
        }
    }
}
