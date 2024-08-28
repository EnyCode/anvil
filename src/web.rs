use action::{Action, ActionComponent};
use strum::IntoEnumIterator;
use yew::{classes, function_component, html, AttrValue, Component, Context, Html, Properties};

use crate::{
    anvil::Anvil,
    enchantments::Enchantment,
    item::{item, Item, ItemType},
    presets::{presets, Preset},
    util::to_roman_numerals,
};

mod action;

pub struct App {
    anvil: Anvil,
    source_items: Option<Vec<Item>>,
    selected_item: Option<usize>,
}

pub enum AppMessage {
    ApplyPreset(Preset),
    AddItem(ItemType),
    ToggleSelect(usize),
    Action(Action),
    ToggleEnchantment(Enchantment),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            anvil: Anvil::new_java(),
            source_items: None,
            selected_item: None,
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
            AppMessage::ToggleSelect(index) => {
                if self.selected_item.map_or(false, |sel| sel == index) {
                    self.selected_item = None;
                } else {
                    self.selected_item = Some(index);
                }

                true
            }
            AppMessage::Action(action) => {
                match action {
                    Action::Remove => {
                        if let Some(selected) = &self.selected_item {
                            self.source_items.as_mut().unwrap().remove(*selected);

                            let items = self.source_items.as_ref().unwrap();
                            if items.is_empty() {
                                self.source_items = None;
                                self.selected_item = None;
                            } else if items.len() == *selected {
                                self.selected_item = Some(items.len() - 1);
                            }
                        }
                    }
                }
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
        let result_html = match &self.source_items {
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
                            <>
                                <div class="anvil items">
                                    <ItemComponent item={item1} />
                                    <div />
                                    <ItemComponent item={item2} />
                                    <div />
                                    <ItemComponent item={result} />
                                </div>
                                <span class="green-xp">
                                    {format!("Enchantment Cost: {cost}")}
                                </span>
                            </>
                        });
                    }

                    items = [new_items, items].concat();
                }

                html! {
                    <div class="container center">
                        <h1>{"Repair & Name"}</h1>
                        <div class="rows">{for rows}</div>
                        <h1 class="green-xp">
                            {format!(
                                "Total Cost: {} (saves {})",
                                results.lowest_cost,
                                results.highest_cost - results.lowest_cost
                            )}
                        </h1>
                    </div>
                }
            }
            None => html! {},
        };

        let item_html = html! {
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
                    <h1>{"Inventory"}</h1>

                    if let Some(source_items) = &self.source_items {
                        <div class="items">
                            {for source_items.iter().enumerate().map(|(i, item)| html! {
                                <div
                                    onclick={ctx.link().callback(move |_| AppMessage::ToggleSelect(i))}
                                >
                                    <ItemComponent
                                        item={item.clone()}
                                        hint={"Click to edit"}
                                        selected={self.selected_item == Some(i)}
                                    />
                                </div>
                            })}
                        </div>
                    }

                    if let Some(selected_item) = &self.selected_item {
                        <h1>{"Actions"}</h1>
                        <div class="items">
                            <div onclick={ctx.link().callback(move |_| AppMessage::Action(Action::Remove))}>
                                <ActionComponent action={Action::Remove} />
                            </div>
                        </div>

                        <h1>{"Enchantments"}</h1>

                        <div class="items">
                            {for Enchantment::friendly_sort(
                                self.source_items.as_ref().unwrap()[*selected_item]
                                    .compatible_enchantments()
                                    .into_iter()
                            ).map(|enchant| html! {
                                <EnchantmentComponent
                                    {enchant}
                                    level={self.source_items.as_ref().unwrap()[*selected_item].level_of(enchant)}
                                />
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
                            source_items[0].compatible_enchantments()
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
            </>
        };

        html! {
            <>
                <div>{item_html}</div>

                <div>{result_html}</div>

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
struct ItemProps {
    item: Item,
    #[prop_or(true)]
    hover: bool,
    #[prop_or(false)]
    selected: bool,
    #[prop_or_default]
    hint: AttrValue,
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

#[derive(PartialEq, Properties)]
struct EnchantmentProps {
    enchant: Enchantment,
    level: Option<u32>,
}

#[function_component]
fn EnchantmentComponent(props: &EnchantmentProps) -> Html {
    let index = props.enchant as usize;
    let x = index % 8;
    let y = index / 8;

    html! {
        <div class="enchantment hover" style={format!("--x:{x};--y:{y}")}>
            <span />
            if let Some(level) = props.level {
                <aside class="level">{level}</aside>
            }
            <div><span>{props.enchant}</span></div>
        </div>
    }
}
