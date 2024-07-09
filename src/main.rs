use anvil::Anvil;
use enchantments::Enchantment;
use item::target_item;
use itertools::Itertools;

use crate::item::Item;

mod anvil;
mod enchantments;
mod item;

fn main() {
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
    let anvil = Anvil::new_java();

    let mut lowest = 1000;
    let mut lowest_solution = None;

    let enchanted_books = &source_items[1..];

    '_perm_loop: for permutation in enchanted_books.iter().permutations(enchanted_books.len()) {
        let mut items: Vec<Item> = permutation.iter().map(|i| i.clone().clone()).collect();
        items.insert(0, source_items[0].clone());

        // combine all the items together
        let mut total_cost = 0;
        while items.len() > 1 {
            let mut new_items = Vec::new();

            for i in 0..items.len() / 2 {
                let item1 = items[i * 2].clone();
                let item2 = items[i * 2 + 1].clone();

                let res = anvil.combine(item1, item2);

                if res.is_none() {
                    continue '_perm_loop;
                }

                let (cost, new_item) = res.unwrap();

                new_items.push(new_item);
                total_cost += cost;
            }

            if items.len() % 2 == 1 {
                new_items.push(items.last().unwrap().clone());
            }

            items = new_items;
        }

        if total_cost < lowest {
            lowest = total_cost;
            lowest_solution = Some(permutation);
        }
    }

    println!("{lowest} levels");
    println!("{lowest_solution:?}");
}
