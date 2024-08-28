use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ActionProps {
    pub action: Action,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Remove,
}

impl Action {
    fn id(&self) -> &'static str {
        match self {
            Self::Remove => "remove",
        }
    }

    fn classes(&self) -> Vec<&'static str> {
        match self {
            Self::Remove => vec!["red"],
        }
    }

    fn tooltip(&self) -> Option<&'static str> {
        match self {
            Self::Remove => None,
        }
    }
}

#[function_component]
pub fn ActionComponent(props: &ActionProps) -> Html {
    html! {
        <div class={classes!("action", "hover", props.action.id())}>
            <span />
            <div>
                <span class={props.action.classes()}>
                    {format!("{:?}", props.action)}
                </span>

                if let Some(tooltip) = props.action.tooltip() {
                    <div class="blue">{tooltip}</div>
                }
            </div>
        </div>
    }
}
