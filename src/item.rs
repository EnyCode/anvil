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

    pub fn can_have_enchantment(&self, enchantment: &Enchantment) -> bool {
        !enchantment.is_conflicting_with(&self.enchantments.iter().map(|(e, _)| e).collect())
    }
}

macro_rules! item {
    ($item_type: expr) => {
        Item::new($item_type)
    };
    ($item_type: expr, $( ($enchantment: expr, $level: expr) ),+) => {{
        let mut item = Item::new($item_type);
        $( item.enchant($enchantment, $level); )+
        item
    }};
}

pub(crate) use item;
