use crate::enchantments::Enchantment;

#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    EnchantedBook,

    // tools
    Pickaxe,
    Axe,
    Shovel,
    Hoe,
    Shears,
    FlintAndSteel,
    FishingRod,
    CarrotOnAStick,
    WarpedFungusOnAStick,

    // combat
    Sword,
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    item_type: ItemType,
    anvil_uses: u32,
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

    pub fn level_of(&self, wanted_enchantment: Enchantment) -> Option<u32> {
        for enchantment in &self.enchantments {
            if enchantment.0 == wanted_enchantment {
                return Some(enchantment.1);
            }
        }

        None
    }

    pub fn has_conflict(&self, enchantment: &Enchantment) -> bool {
        !enchantment.is_conflicting_with(&self.enchantments.iter().map(|(e, _)| e).collect())
    }

    pub fn is_compatible(&self, enchantment: &Enchantment) -> bool {
        self.compatible_enchantments().contains(enchantment)
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
                ItemType::Mace => vec![
                    Enchantment::Density,
                    Enchantment::Breach,
                    Enchantment::WindBurst,
                    Enchantment::Smite,
                    Enchantment::BaneOfArthropods,
                    Enchantment::FireAspect,
                ],
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

macro_rules! item {
    ($item_type: expr) => {
        use crate::item::Item;

        Item::new($item_type)
    };
    ($item_type: expr, $( ($enchantment: expr, $level: expr) ),+) => {{
        use crate::item::Item;

        let mut item = Item::new($item_type);
        $( item.enchant($enchantment, $level); )+
        item
    }};
}

pub(crate) use item;

macro_rules! target_item {
    ($item_type: expr, $( $enchantment: expr ),+) => {{
        use crate::item::{item, Item, ItemType};

        let mut items = vec![Item::new($item_type)];
        $(
            items.push(item!(
                ItemType::EnchantedBook,
                ($enchantment, $enchantment.max_level())
            ));
        )+
        items
    }};
}

pub(crate) use target_item;
