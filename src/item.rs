use std::fmt::Display;
use strum::EnumIter;

use crate::{enchantments::Enchantment, util::prettify_pascal_case};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum ItemType {
    EnchantedBook,

    // common
    Pickaxe,
    Sword,
    Axe,
    Shovel,
    Hoe,

    // less common
    Bow,
    Crossbow,
    Trident,
    Mace,

    // armour
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Shield,
    Elytra,

    // uncommon
    FishingRod,
    Shears,
    FlintAndSteel,
    CarrotOnAStick,
    WarpedFungusOnAStick,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    /// the type of item this is (e.g. book, pickaxe, etc)
    item_type: ItemType,
    /// the amount of times this item has been used in an anvil already
    anvil_uses: u32,
    /// the enchantments and their corresponding levels
    enchantments: Vec<(Enchantment, u32)>,
}

impl Item {
    pub fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            anvil_uses: 0,
            enchantments: Vec::new(),
        }
    }

    pub fn item_type(&self) -> &ItemType {
        &self.item_type
    }

    /// calculates the work penalty of this item using the number of anvil uses
    pub fn work_penalty(&self) -> u32 {
        2u32.pow(self.anvil_uses) - 1
    }

    pub fn increment_anvil_uses(&mut self) {
        self.anvil_uses += 1;
    }

    pub fn enchantments(&self) -> &Vec<(Enchantment, u32)> {
        &self.enchantments
    }

    pub fn into_enchantments(self) -> Vec<(Enchantment, u32)> {
        self.enchantments
    }

    /// adds an enchantment to this item.
    /// if the given level is greater than the maximum level of the enchantment, it will be reduced.
    /// if the item already has this enchantment, its level will be changed, even if the incoming level is lower.
    /// ```
    /// let mut item = item!(ItemType::Pickaxe);
    ///
    /// item.enchant(Enchantment::Fortune, 3);
    /// assert_eq!(item.level_of(Enchantment::Fortune), Some(3));
    ///
    /// // the level of the enchantment automatically becomes 5.
    /// item.enchant(Enchantment::Efficiency, 10);
    /// assert_eq!(item.level_of(Enchantment::Efficiency), Some(5));
    ///
    /// // the existing efficiency enchantment is overwritten.
    /// item.enchant(Enchantment::Efficiency, 1);
    /// assert_eq!(item.level_of(Enchantment::Efficiency), Some(1));
    /// ```
    pub fn enchant(&mut self, enchantment: Enchantment, level: u32) {
        let level = u32::min(level, enchantment.max_level());

        for existing_enchantment in &mut self.enchantments {
            if existing_enchantment.0 == enchantment {
                existing_enchantment.1 = level;
                return;
            }
        }

        self.enchantments.push((enchantment, level));
    }

    /// gets the level of the given enchantment, or `None` if the item doesn't have it.
    pub fn level_of(&self, wanted_enchantment: Enchantment) -> Option<u32> {
        for enchantment in &self.enchantments {
            if enchantment.0 == wanted_enchantment {
                return Some(enchantment.1);
            }
        }

        None
    }

    /// checks if the given enchantment is conflicting with the item.
    /// conflicting means that the enchantment is conflicting with another enchantment.
    /// for example, the Fortune and Silk Touch enchantments conflict with each other.
    pub fn has_conflict(&self, enchantment: &Enchantment) -> bool {
        !enchantment.is_conflicting_with(&self.enchantments.iter().map(|(e, _)| e).collect())
    }

    /// checks if the given enchantment is compatible with the item.
    /// for example, Silk Touch is compatible with pickaxes, but not with swords.
    pub fn is_compatible(&self, enchantment: &Enchantment) -> bool {
        if self.item_type == ItemType::EnchantedBook {
            true
        } else {
            self.compatible_enchantments().contains(enchantment)
        }
    }

    pub fn compatible_enchantments(&self) -> Vec<Enchantment> {
        [
            match self.item_type {
                // special case
                ItemType::EnchantedBook => Vec::new(),

                ItemType::Pickaxe | ItemType::Shovel | ItemType::Hoe => vec![
                    Enchantment::Efficiency,
                    Enchantment::SilkTouch,
                    Enchantment::Fortune,
                ],
                ItemType::Axe => vec![
                    Enchantment::Sharpness,
                    Enchantment::Smite,
                    Enchantment::BaneOfArthropods,
                    Enchantment::Efficiency,
                    Enchantment::SilkTouch,
                    Enchantment::Fortune,
                ],
                ItemType::Shears => vec![Enchantment::Efficiency],
                ItemType::FlintAndSteel
                | ItemType::CarrotOnAStick
                | ItemType::WarpedFungusOnAStick
                | ItemType::Shield => Vec::new(),
                ItemType::FishingRod => vec![Enchantment::LuckOfTheSea, Enchantment::Lure],
                ItemType::Sword => vec![
                    Enchantment::Sharpness,
                    Enchantment::Smite,
                    Enchantment::BaneOfArthropods,
                    Enchantment::Knockback,
                    Enchantment::FireAspect,
                    Enchantment::Looting,
                    Enchantment::SweepingEdge,
                ],
                ItemType::Bow => vec![
                    Enchantment::Power,
                    Enchantment::Punch,
                    Enchantment::Flame,
                    Enchantment::Infinity,
                ],
                ItemType::Crossbow => vec![
                    Enchantment::Multishot,
                    Enchantment::Piercing,
                    Enchantment::QuickCharge,
                ],
                ItemType::Trident => vec![
                    Enchantment::Impaling,
                    Enchantment::Riptide,
                    Enchantment::Loyalty,
                    Enchantment::Channeling,
                ],
                ItemType::Mace => vec![
                    Enchantment::Density,
                    Enchantment::Breach,
                    Enchantment::WindBurst,
                    Enchantment::Smite,
                    Enchantment::BaneOfArthropods,
                    Enchantment::FireAspect,
                ],
                // TODO: remove duplicate armour enchantments?
                ItemType::Helmet => vec![
                    Enchantment::Protection,
                    Enchantment::FireProtection,
                    Enchantment::BlastProtection,
                    Enchantment::ProjectileProtection,
                    Enchantment::Thorns,
                    Enchantment::Respiration,
                    Enchantment::AquaAffinity,
                    Enchantment::CurseOfBinding,
                ],
                ItemType::Chestplate => vec![
                    Enchantment::Protection,
                    Enchantment::FireProtection,
                    Enchantment::BlastProtection,
                    Enchantment::ProjectileProtection,
                    Enchantment::Thorns,
                    Enchantment::CurseOfBinding,
                ],
                ItemType::Leggings => vec![
                    Enchantment::Protection,
                    Enchantment::FireProtection,
                    Enchantment::BlastProtection,
                    Enchantment::ProjectileProtection,
                    Enchantment::Thorns,
                    Enchantment::CurseOfBinding,
                    Enchantment::SwiftSneak,
                ],
                ItemType::Boots => vec![
                    Enchantment::Protection,
                    Enchantment::FireProtection,
                    Enchantment::FeatherFalling,
                    Enchantment::BlastProtection,
                    Enchantment::ProjectileProtection,
                    Enchantment::Thorns,
                    Enchantment::DepthStrider,
                    Enchantment::CurseOfBinding,
                    Enchantment::SoulSpeed,
                ],
                ItemType::Elytra => vec![Enchantment::CurseOfBinding],
            },
            // these enchantments can go on everything
            vec![
                Enchantment::Unbreaking,
                Enchantment::Mending,
                Enchantment::CurseOfVanishing,
            ],
        ]
        .concat()
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prettify_pascal_case(format!("{self:?}")))
    }
}

macro_rules! item {
    ($item_type: expr) => {{
        use crate::item::Item;

        Item::new($item_type)
    }};
    ($item_type: expr, $( ($enchantment: expr, $level: expr) ),+) => {{
        use crate::item::Item;

        let mut item = Item::new($item_type);
        $( item.enchant($enchantment, $level); )+
        item
    }};
}

pub(crate) use item;

#[cfg(test)]
mod tests {
    use crate::{enchantments::Enchantment, item::ItemType};

    #[test]
    fn enchant_item() {
        let mut item = item!(ItemType::Pickaxe);

        item.enchant(Enchantment::Fortune, 3);
        assert_eq!(item.level_of(Enchantment::Fortune), Some(3));

        // the level of the enchantment automatically becomes 5.
        item.enchant(Enchantment::Efficiency, 10);
        assert_eq!(item.level_of(Enchantment::Efficiency), Some(5));

        // the existing efficiency enchantment is overwritten.
        item.enchant(Enchantment::Efficiency, 1);
        assert_eq!(item.level_of(Enchantment::Efficiency), Some(1));
    }
}
