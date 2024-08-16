use strum::IntoEnumIterator;
use yew::{classes, function_component, html, AttrValue, Component, Context, Html, Properties};

use crate::{
    anvil::Anvil,
    enchantments::Enchantment,
    item::{item, Item, ItemType},
    presets::{presets, Preset},
    util::to_roman_numerals,
};

pub struct App {
    anvil: Anvil,
    source_items: Option<Vec<Item>>,
}

pub enum AppMessage {
    ApplyPreset(Preset),
    AddItem(ItemType),
    EditItem(usize),
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
            AppMessage::ApplyPreset(preset) => {
                self.source_items = Some([preset.items, preset.books].concat());
                true
            }
            AppMessage::AddItem(item_type) => {
                // item can be added if its type already exists in the items, or it's an enchanted book
                let can_add = self.source_items.as_ref().map_or(true, |items| {
                    items.iter().any(|item| item.item_type() == &item_type)
                }) || item_type == ItemType::EnchantedBook;

                if can_add {
                    if self.source_items.is_none() {
                        self.source_items = Some(Vec::new());
                    }
                    self.source_items
                        .as_mut()
                        .unwrap()
                        .push(Item::new(item_type));
                }

                true
            }
            AppMessage::EditItem(index) => {
                self.source_items.as_mut().unwrap().remove(index);

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

                        let (cost, result, _) =
                            self.anvil.combine(item1.clone(), item2.clone()).unwrap();
                        new_items.push(result.clone());

                        rows.push(html! {
                            <div class="row">
                                <ItemComponent item={item1} hover={false} />
                                {"+"}
                                <ItemComponent item={item2} hover={false} />
                                {"="}
                                <ItemComponent item={result} />
                                <span class="green">
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
                            <span class="green">
                                {format!("{} levels", results.lowest_cost)}
                            </span>
                        </h1>
                        <h2>
                            {"Maximum "}
                            <span class="green">{results.highest_cost}</span>
                            {" - Saves "}
                            <span class="green">
                                {results.highest_cost - results.lowest_cost}
                            </span>
                        </h2>
                        <div class="rows">
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
                    <div class="items">
                        {for presets().into_iter().map(|preset| {
                            // no i dont know why im cloning twice.
                            let preset_clone = preset.clone();
                            html! {
                                <div
                                    onclick={ctx.link().callback(move |_| AppMessage::ApplyPreset(preset_clone.clone()))}
                                >
                                    <ItemComponent item={preset.result} />
                                </div>
                            }
                        })}
                    </div>

                    <h1>{"Add Items"}</h1>
                    <div class="items">
                        {for ItemType::iter().map(|item_type| html! {
                            <div
                                onclick={ctx.link().callback(move |_| AppMessage::AddItem(item_type))}
                            >
                                <ItemComponent item={Item::new(item_type)} />
                            </div>
                        })}
                    </div>
                </div>

                <div class="container">
                    <h1>{ "Inventory" }</h1>

                    if let Some(source_items) = &self.source_items {
                        <div class="items">
                            {for source_items.iter().enumerate().map(|(i, item)| html! {
                                <div
                                    onclick={ctx.link().callback(move |_| AppMessage::EditItem(i))}
                                >
                                    <ItemComponent
                                        item={item.clone()}
                                        hint={"Click to edit"}
                                    />
                                </div>
                            })}
                        </div>
                    }
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
    #[prop_or(true)]
    pub hover: bool,
    #[prop_or(false)]
    pub selected: bool,
    #[prop_or_default]
    pub hint: AttrValue,
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

                if props.hint.as_str().len() > 0 {
                    <div class="blue">{props.hint.clone()}</div>
                }
            </div>
        </div>
    }
}
