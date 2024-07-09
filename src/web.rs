use yew::{html, Component};

use crate::anvil::Anvil;

pub struct App {
    anvil: Anvil,
}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            anvil: Anvil::new_java(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <p>{ "Hello!" }</p>
        }
    }
}
