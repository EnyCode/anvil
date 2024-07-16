use yew::{classes, function_component, html, Component, Context, Html, Properties};

use crate::{
    anvil::{Anvil, AnvilCombinationResults},
    enchantments::Enchantment,
    item::{target_item, Item, ItemType},
    util::to_roman_numerals,
};

pub struct App {
    anvil: Anvil,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            anvil: Anvil::new_java(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let source_items = target_item!(
            ItemType::Boots,
            Enchantment::Unbreaking,
            Enchantment::Mending,
            Enchantment::FireProtection,
            Enchantment::FeatherFalling,
            Enchantment::DepthStrider,
            Enchantment::SoulSpeed,
            Enchantment::Thorns
        );
        let results = self.anvil.combine_many(source_items);

        let mut items = results.lowest_solution;
        let mut rows = Vec::new();

        while items.len() > 1 {
            let mut new_items = Vec::new();

            for i in 0..items.len() / 2 {
                let item1 = items.remove(0);
                let item2 = items.remove(0);

                let (cost, result) = self.anvil.combine(item1.clone(), item2.clone()).unwrap();
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
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    pub item: Item,
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
