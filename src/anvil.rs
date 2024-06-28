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
                    if !new_item.can_have_enchantment(&enchantment) {
                        // if the enchantments are conflicting, this costs one level in java
                        if self.behavior == AnvilBehavior::Java {
                            total_cost += 1;
                        }

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

        Some((total_cost, new_item))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        enchantments::Enchantment,
        item::{item, Item, ItemType},
    };

    use super::Anvil;

    // tests are from https://minecraft.fandom.com/wiki/Anvil_mechanics#Costs_for_combining_enchantments

    macro_rules! assert_anvil_combine {
        ($item1: expr, $item2: expr, $java_cost: expr, $bedrock_cost: expr) => {{
            // assert costs
            let res = Anvil::new_java().combine($item1.clone(), $item2.clone());
            assert!(res.is_some());
            let (cost, item) = res.unwrap();

            let res2 = Anvil::new_bedrock().combine($item1.clone(), $item2.clone());
            assert!(res2.is_some());
            let (cost2, item2) = res2.unwrap();

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
}
