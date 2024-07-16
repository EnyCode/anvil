use yew::{classes, function_component, html, Component, Context, Html, Properties};

use crate::{
    anvil::Anvil,
    item::Item,
    presets::presets,
    util::{target_for_source_items, to_roman_numerals},
};

pub struct App {
    anvil: Anvil,
    source_items: Option<Vec<Item>>,
}

pub enum AppMessage {
    ApplyPreset(Vec<Item>),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            anvil: Anvil::new_java(),
            source_items: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::ApplyPreset(source_items) => {
                self.source_items = Some(source_items);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let body_html = match &self.source_items {
            Some(source_items) => {
                let results = self.anvil.combine_many(source_items.clone());

                let mut items = results.lowest_solution;
                let mut rows = Vec::new();

                while items.len() > 1 {
                    let mut new_items = Vec::new();

                    for i in 0..items.len() / 2 {
                        let item1 = items.remove(0);
                        let item2 = items.remove(0);

                        let (cost, result) =
                            self.anvil.combine(item1.clone(), item2.clone()).unwrap();
                        new_items.push(result.clone());

                        rows.push(html! {
                            <div class={classes!("row")}>
                                <ItemComponent item={item1} />
                                {"+"}
                                <ItemComponent item={item2} />
                                {"="}
                                <ItemComponent item={result} />
                                <span class={classes!("green")}>
                                    {format!("{cost} levels")}
                                </span>
                            </div>
                        });
                    }

                    items = [new_items, items].concat();
                }

                html! {
                    <>
                        <h1>
                            {format!("{} - ", items[0].item_type())}
                            <span class={classes!("green")}>
                                {format!("{} levels", results.lowest_cost)}
                            </span>
                        </h1>
                        <h2>
                            {format!(
                                "Maximum price {} - saves {}",
                                results.highest_cost,
                                results.highest_cost - results.lowest_cost
                            )}
                        </h2>
                        <div class={classes!("rows")}>
                            {for rows}
                        </div>
                    </>
                }
            }
            None => html! {},
        };

        html! {
            <>
                <h1>{"Presets"}</h1>
                <div id="presets">
                    {for presets().iter().map(|src_items| {
                        let preset = src_items.clone();

                        html! {
                            <div
                                onclick={ctx.link().callback(move |_| AppMessage::ApplyPreset(preset.clone()))}
                            >
                                <ItemComponent
                                    item={target_for_source_items(src_items)}
                                    hover={true}
                                />
                            </div>
                        }
                    })}
                </div>
                {body_html}
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    pub item: Item,
    #[prop_or(false)]
    pub hover: bool,
}

#[function_component]
fn ItemComponent(props: &ItemProps) -> Html {
    let mut classes = vec![
        "item".to_string(),
        props
            .item
            .item_type()
            .to_string()
            .to_lowercase()
            .replace(' ', "-"),
    ];

    if props.item.enchantments().len() > 0 {
        classes.push("enchanted".to_string());
    }

    if props.hover {
        classes.push("hover".to_string());
    }

    html! {
        <div class={classes!(classes)}>
            <span />
            <div>
                <span>
                    {props.item.item_type()}
                </span>
                <div>
                    {for props.item.enchantments().iter().map(|(e, l)| html! {
                        <span>{e}{to_roman_numerals(*l)}</span>
                    })}
                </div>
            </div>
        </div>
    }
}
