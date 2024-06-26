use crate::item::{Item, ItemType};

#[derive(PartialEq)]
enum AnvilBehavior {
    Java,
    Bedrock,
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

    pub fn combine(&self, target: Item, sacrifice: Item) -> Option<(u32, Item)> {
        let sacrifice_is_book = sacrifice.item_type() == &ItemType::EnchantedBook;

        // if the two item types are incompatible, return None
        if target.item_type() != sacrifice.item_type() && !sacrifice_is_book {
            return None;
        }

        let mut new_item = target.clone();

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

                    // on java, the enchantment cost is the final level.
                    // on bedrock, it's the difference between the final and initial levels.
                    match self.behavior {
                        AnvilBehavior::Java => new_level,
                        AnvilBehavior::Bedrock => new_level - target_level,
                    }
                }
                // if the enchantment doesn't exist on the target, move it on.
                None => {
                    // TODO: check the enchantment isn't incompatible
                    new_item.enchant(enchantment, sacrifice_level);

                    sacrifice_level
                }
            };

            total_cost += cost
                * match self.behavior {
                    AnvilBehavior::Java => enchantment.java_multiplier(sacrifice_is_book),
                    AnvilBehavior::Bedrock => enchantment.bedrock_multiplier(sacrifice_is_book),
                };
        }

        Some((total_cost, new_item))
    }
}
