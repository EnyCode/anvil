use yew::{classes, function_component, html, Component, Context, Html, Properties};

use crate::{
    anvil::{Anvil, AnvilCombinationResults},
    enchantments::Enchantment,
    item::{target_item, Item},
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
        let solution = results.lowest_solution;

        let mut rows = Vec::new();

        for i in 0..solution.len() / 2 {
            let item1 = &solution[i * 2];
            let item2 = &solution[i * 2 + 1];

            let (cost, result) = self.anvil.combine(item1.clone(), item2.clone()).unwrap();

            rows.push(html! {
                <div class={classes!("row")}>
                    <ItemComponent item={solution[i * 2].clone()} />
                    {"+"}
                    <ItemComponent item={solution[i * 2 + 1].clone()} />
                    {"="}
                    <ItemComponent item={result} />
                    {format!("({cost} levels)")}
                </div>
            });
        }

        html! {
            <div class={classes!("rows")}>
                {for rows}
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    pub item: Item,
}

#[function_component]
fn ItemComponent(props: &ItemProps) -> Html {
    html! {
        <div class={classes!("item")}>
            <span>
                {props.item.item_type()}
            </span>
            <div>
                {for props.item.enchantments().iter().map(|(e, l)| html! {
                    <span>{e}{to_roman_numerals(*l)}</span>
                })}
            </div>
        </div>
    }
}

