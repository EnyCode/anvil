use itertools::Itertools;

use crate::item::{Item, ItemType};

#[derive(PartialEq)]
enum AnvilBehavior {
    Java,
    Bedrock,
}

#[derive(PartialEq)]
pub struct AnvilCombinationResults {
    pub lowest_cost: u32,
    pub lowest_solution: Vec<Item>,
    pub highest_cost: u32,
    pub highest_solution: Vec<Item>,
    pub rank: AnvilCombinationRank,
}

#[derive(Debug, PartialEq)]
pub enum AnvilCombinationRank {
    Perfect,
    Flawed,
}

pub struct Anvil {
    behavior: AnvilBehavior,
}

impl Anvil {
    pub fn new_java() -> Self {
        Anvil {
            behavior: AnvilBehavior::Java,
        }
    }

    pub fn new_bedrock() -> Self {
        Anvil {
            behavior: AnvilBehavior::Bedrock,
        }
    }

    /// combines the target and sacrifice items in this anvil.
    /// returns `None` if the items are incompatible.
    /// returns a tuple containing the price, resulting item, and whether enchantment levels are lost.
    pub fn combine(
        &self,
        target: Item,
        sacrifice: Item,
    ) -> Option<(u32, Item, AnvilCombinationRank)> {
        let sacrifice_is_book = sacrifice.item_type() == &ItemType::EnchantedBook;

        // if the two item types are incompatible, return None
        if target.item_type() != sacrifice.item_type() && !sacrifice_is_book {
            return None;
        }

        let mut new_item = target.clone();
        let mut rank = AnvilCombinationRank::Perfect;

        let mut total_cost = target.work_penalty() + sacrifice.work_penalty();

        for (enchantment, sacrifice_level) in sacrifice.into_enchantments() {
            let cost = match new_item.level_of(enchantment.clone()) {
                // the enchantment already exists on the target item
                Some(target_level) => {
                    // if they have the same level, increment the level.
                    // otherwise, use the maximum of the two levels.
                    let new_level = (if target_level == sacrifice_level {
                        target_level + 1
                    } else {
                        u32::max(target_level, sacrifice_level)
                    })
                    .min(enchantment.max_level());

                    new_item.enchant(enchantment, new_level);

                    // the combination is flawed if an enchantment is "lost"
                    if sacrifice_level != target_level {
                        rank = AnvilCombinationRank::Flawed;
                    }

                    // in java, the enchantment cost is the final level.
                    // in bedrock, it's the difference between the final and initial levels.
                    match self.behavior {
                        AnvilBehavior::Java => new_level,
                        AnvilBehavior::Bedrock => new_level - target_level,
                    }
                }
                // if the enchantment doesn't exist on the target, add it on.
                None => {
                    if !new_item.has_conflict(&enchantment) {
                        // if the enchantments are conflicting, this costs one level in java
                        if self.behavior == AnvilBehavior::Java {
                            total_cost += 1;
                        }

                        0
                    } else if !new_item.is_compatible(&enchantment) {
                        0
                    } else {
                        new_item.enchant(enchantment, sacrifice_level);

                        sacrifice_level
                    }
                }
            };

            total_cost += cost
                * match self.behavior {
                    AnvilBehavior::Java => enchantment.java_multiplier(sacrifice_is_book),
                    AnvilBehavior::Bedrock => enchantment.bedrock_multiplier(sacrifice_is_book),
                };
        }

        new_item.increment_anvil_uses();
        Some((total_cost, new_item, rank))
    }

    /// given a vector of source items, this function checks all the possible ways to combine the items together.
    /// the function returns a struct containing information about the found results.
    pub fn combine_many(&self, source_items: Vec<Item>) -> AnvilCombinationResults {
        let mut lowest_cost = u32::MAX;
        let mut lowest_solution = Vec::new();

        let mut lowest_cost_flawed = u32::MAX;
        let mut lowest_solution_flawed = Vec::new();

        let mut highest_cost = u32::MIN;
        let mut highest_solution = Vec::new();

        // TODO: for playground mode
        // the slice of enchanted books may not be [1..]
        // the items which aren't enchanted books should be permuted as well
        // when inserting below, insert all the non-book items

        let e_books = &source_items[1..];
        let e_book_count = e_books.len();
        let e_book_permutations = e_books.into_iter().permutations(e_book_count);

        for permutation in e_book_permutations {
            let mut items: Vec<Item> = permutation.into_iter().map(|i| i.clone()).collect();
            items.insert(0, source_items[0].clone());

            let items_original = items.clone();

            // combine all the items together
            let mut total_cost = 0;
            let mut rank = AnvilCombinationRank::Perfect;
            while items.len() > 1 {
                let mut new_items = Vec::new();

                for _ in 0..items.len() / 2 {
                    let item1 = items.remove(0);
                    let item2 = items.remove(0);

                    let (cost, new_item, new_rank) = self.combine(item1, item2).unwrap();

                    if new_rank == AnvilCombinationRank::Flawed {
                        rank = AnvilCombinationRank::Flawed;
                    }

                    new_items.push(new_item);
                    total_cost += cost;
                }

                if items.len() > 0 {
                    new_items.push(items.remove(0));
                }

                items = new_items;
            }

            if total_cost < lowest_cost && rank == AnvilCombinationRank::Perfect {
                lowest_cost = total_cost;
                lowest_solution = items_original;
            } else if total_cost < lowest_cost_flawed && rank == AnvilCombinationRank::Flawed {
                lowest_cost_flawed = total_cost;
                lowest_solution_flawed = items_original;
            } else if total_cost > highest_cost {
                highest_cost = total_cost;
                highest_solution = items_original;
            }
        }

        // when there is a single solution, only the lowest solution is assigned (to appease the borrow checker).
        // when this happens, we can just copy those values to the highest solution.
        if highest_cost == 0 && lowest_cost > 0 {
            highest_cost = lowest_cost;
            highest_solution = lowest_solution.clone();
        }

        // if there aren't any perfect solutions, go with a flawed one instead
        let rank = if lowest_cost == u32::MAX {
            lowest_cost = lowest_cost_flawed;
            lowest_solution = lowest_solution_flawed;
            AnvilCombinationRank::Flawed
        } else {
            AnvilCombinationRank::Perfect
        };

        AnvilCombinationResults {
            lowest_cost,
            lowest_solution,
            highest_cost,
            highest_solution,
            rank,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        enchantments::Enchantment,
        item::{item, ItemType},
    };

    use super::Anvil;

    // tests are from https://minecraft.wiki/w/Anvil_mechanics#Costs_for_combining_enchantments

    macro_rules! assert_anvil_combine {
        ($item1: expr, $item2: expr, $java_cost: expr, $bedrock_cost: expr) => {{
            // assert costs
            let res = Anvil::new_java().combine($item1.clone(), $item2.clone());
            assert!(res.is_some());
            let (cost, item, _) = res.unwrap();

            let res2 = Anvil::new_bedrock().combine($item1.clone(), $item2.clone());
            assert!(res2.is_some());
            let (cost2, item2, _) = res2.unwrap();

            assert_eq!(cost, $java_cost);
            assert_eq!(cost2, $bedrock_cost);

            // assert result items are equal
            assert_eq!(item, item2);

            item
        }};
    }

    macro_rules! assert_enchantments {
        ($item: expr, $( ($enchantment: expr, $level: expr) ),+) => {
            // assert that the item has as many enchantments as we give
            assert_eq!($item.enchantments().len(), [$($level),+].len());

            // assert each individual enchantment
            $(
                assert_eq!($item.level_of($enchantment), Some($level));
            )+
        }
    }

    #[test]
    fn equal_enchantments() {
        let item1 = item!(
            ItemType::Sword,
            (Enchantment::Sharpness, 3),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 3)
        );
        let item2 = item!(
            ItemType::Sword,
            (Enchantment::Sharpness, 3),
            (Enchantment::Looting, 3)
        );

        // item1 + item2
        let item = assert_anvil_combine!(item1, item2, 16, 1);
        assert_enchantments!(
            item,
            (Enchantment::Sharpness, 4),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 3)
        );

        // item2 + item1
        let item = assert_anvil_combine!(item2, item1, 20, 5);
        assert_enchantments!(
            item,
            (Enchantment::Sharpness, 4),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 3)
        );
    }

    #[test]
    fn unequal_enchantments() {
        let item1 = item!(
            ItemType::Sword,
            (Enchantment::Sharpness, 3),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 1)
        );
        let item2 = item!(
            ItemType::Sword,
            (Enchantment::Sharpness, 1),
            (Enchantment::Looting, 3)
        );

        // item1 + item2
        let item = assert_anvil_combine!(item1, item2, 15, 8);
        assert_enchantments!(
            item,
            (Enchantment::Sharpness, 3),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 3)
        );

        // item2 + item1
        let item = assert_anvil_combine!(item2, item1, 19, 6);
        assert_enchantments!(
            item,
            (Enchantment::Sharpness, 3),
            (Enchantment::Knockback, 2),
            (Enchantment::Looting, 3)
        );
    }

    #[test]
    fn conflicting_enchantments() {
        let item1 = item!(
            ItemType::Sword,
            (Enchantment::Sharpness, 2),
            (Enchantment::Looting, 2)
        );
        let item2 = item!(
            ItemType::Sword,
            (Enchantment::Smite, 5),
            (Enchantment::Looting, 2)
        );

        // item1 + item2
        let item = assert_anvil_combine!(item1, item2, 13, 4);
        assert_enchantments!(item, (Enchantment::Sharpness, 2), (Enchantment::Looting, 3));

        // item2 + item1
        let item = assert_anvil_combine!(item2, item1, 13, 4);
        assert_enchantments!(item, (Enchantment::Smite, 5), (Enchantment::Looting, 3));
    }

    #[test]
    fn using_books() {
        let item1 = item!(ItemType::Sword, (Enchantment::Looting, 2));
        let item2 = item!(
            ItemType::EnchantedBook,
            (Enchantment::Protection, 3),
            (Enchantment::Sharpness, 1),
            (Enchantment::Looting, 2)
        );

        let item = assert_anvil_combine!(item1, item2, 7, 3);
        assert_eq!(item.item_type(), &ItemType::Sword);
        assert_enchantments!(item, (Enchantment::Sharpness, 1), (Enchantment::Looting, 3));
    }
}
