use strum::IntoEnumIterator;
use yew::{classes, function_component, html, Component, Context, Html, Properties};

use crate::{
    anvil::Anvil,
    enchantments::Enchantment,
    item::{item, Item, ItemType},
    presets::presets,
    util::{target_for_source_items, to_roman_numerals},
};

pub struct App {
    anvil: Anvil,
    source_items: Option<Vec<Item>>,
}

pub enum AppMessage {
    ApplyPreset(Vec<Item>),
    SetItemType(ItemType),
    ToggleEnchantment(Enchantment),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            anvil: Anvil::new_java(),
            source_items: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::ApplyPreset(source_items) => {
                self.source_items = Some(source_items);
                true
            }
            AppMessage::SetItemType(item_type) => {
                self.source_items = Some(vec![Item::new(item_type)]);
                true
            }
            AppMessage::ToggleEnchantment(enchantment) => {
                // search for the enchantment
                match self
                    .source_items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .position(|i| i.level_of(enchantment).is_some())
                {
                    // if the enchantment has been found, remove it
                    Some(i) => {
                        self.source_items.as_mut().unwrap().remove(i);
                    }
                    // if the enchantment isn't present, add it
                    None => {
                        self.source_items.as_mut().unwrap().push(item!(
                            ItemType::EnchantedBook,
                            (enchantment, enchantment.max_level())
                        ));
                    }
                }
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

                    for _ in 0..items.len() / 2 {
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
                            {"Maximum "}
                            <span class={classes!("green")}>{results.highest_cost}</span>
                            {" - Saves "}
                            <span class={classes!("green")}>
                                {results.highest_cost - results.lowest_cost}
                            </span>
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
                <div class="container">
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

                    <h1>{"Add Items"}</h1>
                    <div id="items">
                        {for ItemType::iter().map(|item_type| {
                            let selected = match &self.source_items {
                                Some(source_items) => source_items[0].item_type() == &item_type,
                                None => false,
                            };

                            html! {
                                <div
                                    onclick={ctx.link().callback(move |_| AppMessage::SetItemType(item_type))}
                                >
                                    <ItemComponent
                                        item={Item::new(item_type)}
                                        hover={true}
                                        {selected}
                                    />
                                </div>
                            }
                        })}
                    </div>
                </div>

                if let Some(source_items) = &self.source_items {
                    <h2>{"Enchantments"}</h2>
                    <div id="enchantments">
                        {for
                            // get all compatible enchantments
                            // or all enchantments, if the item is an enchanted book
                            if source_items[0].item_type() != &ItemType::EnchantedBook {
                                source_items[0].compatible_enchantments().into_iter()
                            } else {
                                Enchantment::iter().collect::<Vec<_>>().into_iter()
                            }
                            .map(|enchantment| {
                                let selected =  if source_items
                                    .iter()
                                    .any(|i| i.level_of(enchantment).is_some())
                                {
                                    Some("selected")
                                } else {
                                    None
                                };

                                html! {
                                    <div
                                        class={classes!(selected)}
                                        onclick={ctx.link().callback(move |_| AppMessage::ToggleEnchantment(enchantment))}
                                    >
                                        {enchantment}
                                    </div>
                                }
                            })
                        }
                    </div>
                }

                {body_html}

                <footer>
                    <a href="https://github.com/EnyCode/anvil/" target="_blank">
                        {"this project is OPEN SOURCE!"}
                    </a>
                </footer>
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    pub item: Item,
    #[prop_or(false)]
    pub hover: bool,
    #[prop_or(false)]
    pub selected: bool,
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
    if props.selected {
        classes.push("selected".to_string());
    }

    html! {
        <div class={classes!(classes)}>
            <span />
            <div>
                <span>
                    {props.item.item_type()}
                </span>
                <div>
                    {for props.item.enchantments().iter().map(|(e, l)| {
                        let red = if e.is_curse() {
                            Some("red")
                        } else {
                            None
                        };

                        html! {
                            <span class={classes!(red)}>
                                {e}{to_roman_numerals(*l)}
                            </span>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}
