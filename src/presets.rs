use crate::item::Item;

#[derive(Clone)]
pub struct Preset {
    pub items: Vec<Item>,
    pub books: Vec<Item>,
    pub result: Item,
}

pub fn presets() -> Vec<Preset> {
    vec![
        preset!(Pickaxe, Unbreaking, Mending, Efficiency, Fortune),
        preset!(Pickaxe, Unbreaking, Mending, Efficiency, SilkTouch),
        preset!(Sword, Unbreaking, Mending, Sharpness, Looting, FireAspect),
        preset!(Axe, Unbreaking, Mending, Efficiency, Sharpness, SilkTouch),
        preset!(Shovel, Unbreaking, Mending, Efficiency, SilkTouch),
        preset!(Hoe, Unbreaking, Mending, Efficiency, Fortune),
        preset!(Bow, Unbreaking, Mending, Power, Flame),
        preset!(Bow, Unbreaking, Infinity, Power, Flame),
        preset!(Crossbow, Unbreaking, Mending, QuickCharge, Piercing),
        preset!(Trident, Unbreaking, Mending, Loyalty, Channeling, Impaling),
        preset!(Trident, Unbreaking, Mending, Riptide, Impaling),
        preset!(Mace, Unbreaking, Density, WindBurst, Mending),
        preset!(
            Helmet,
            Unbreaking,
            Mending,
            ProjectileProtection,
            Respiration,
            AquaAffinity,
            Thorns
        ),
        preset!(Chestplate, Unbreaking, Mending, BlastProtection, Thorns),
        preset!(Leggings, Unbreaking, Mending, Protection, Thorns, SwiftSneak),
        preset!(
            Boots,
            Unbreaking,
            Mending,
            FireProtection,
            FeatherFalling,
            DepthStrider,
            SoulSpeed,
            Thorns
        ),
        preset!(Shield, Unbreaking, Mending),
        preset!(Elytra, Unbreaking, Mending),
        preset!(FishingRod, Unbreaking, Mending, LuckOfTheSea, Lure),
        preset!(Shears, Unbreaking, Mending, Efficiency),
        preset!(FlintAndSteel, Unbreaking, Mending),
        preset!(CarrotOnAStick, Unbreaking, Mending),
        preset!(WarpedFungusOnAStick, Unbreaking, Mending),
    ]
}

macro_rules! preset {
    ($item_type: expr, $( $enchantment: expr ),+) => {{
        use crate::item::{item, Item, ItemType::*};
        use crate::enchantments::Enchantment::*;

        let mut preset = Preset {
            items: vec![Item::new($item_type)],
            books: Vec::new(),
            result: item!(
                $item_type,
                $( ($enchantment, $enchantment.max_level()) ),+
            ),
        };

        $(
            let ench = $enchantment;
            for _ in 0..2u32.pow(ench.max_level() - ench.max_obtainable()) {
                preset.books.push(item!(
                    EnchantedBook,
                    ($enchantment, ench.max_obtainable())
                ));
            }
        )+
        preset
    }};
}
use preset;
