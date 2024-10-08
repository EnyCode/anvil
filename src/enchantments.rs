use std::{collections::HashMap, fmt::Display, sync::LazyLock};

use itertools::Itertools;
use strum::EnumIter;

use crate::{item::Item, util::prettify_pascal_case};

#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum Enchantment {
    Protection,
    FireProtection,
    FeatherFalling,
    BlastProtection,
    ProjectileProtection,
    Respiration,
    AquaAffinity,
    Thorns,
    DepthStrider,
    FrostWalker,
    CurseOfBinding,
    SoulSpeed,
    SwiftSneak,
    Sharpness,
    Smite,
    BaneOfArthropods,
    Knockback,
    FireAspect,
    Looting,
    SweepingEdge,
    Efficiency,
    SilkTouch,
    Unbreaking,
    Fortune,
    Power,
    Punch,
    Flame,
    Infinity,
    LuckOfTheSea,
    Lure,
    Loyalty,
    Impaling,
    Riptide,
    Channeling,
    Multishot,
    QuickCharge,
    Piercing,
    Density,
    Breach,
    WindBurst,
    Mending,
    CurseOfVanishing,
}

impl Enchantment {
    const CONFLICTING_GROUPS: &'static [&'static [Self]] = &[
        &[
            Self::Protection,
            Self::FireProtection,
            Self::BlastProtection,
            Self::ProjectileProtection,
        ],
        &[Self::DepthStrider, Self::FrostWalker],
        &[
            Self::Sharpness,
            Self::Smite,
            Self::BaneOfArthropods,
            Self::Impaling,
            Self::Density,
            Self::Breach,
        ],
        &[Self::SilkTouch, Self::Fortune],
        &[Self::Infinity, Self::Mending],
        &[Self::Loyalty, Self::Riptide],
        &[Self::Riptide, Self::Channeling],
        &[Self::Multishot, Self::Piercing],
    ];

    /// returns the maximum level for the current enchantment
    pub fn max_level(&self) -> u32 {
        match self {
            Self::Sharpness
            | Self::Smite
            | Self::BaneOfArthropods
            | Self::Efficiency
            | Self::Power
            | Self::Impaling
            | Self::Density => 5,
            Self::Protection
            | Self::FireProtection
            | Self::FeatherFalling
            | Self::BlastProtection
            | Self::ProjectileProtection
            | Self::Piercing
            | Self::Breach => 4,
            Self::Respiration
            | Self::Thorns
            | Self::DepthStrider
            | Self::SoulSpeed
            | Self::SwiftSneak
            | Self::Looting
            | Self::SweepingEdge
            | Self::Unbreaking
            | Self::Fortune
            | Self::LuckOfTheSea
            | Self::Lure
            | Self::Loyalty
            | Self::Riptide
            | Self::QuickCharge
            | Self::WindBurst => 3,
            Self::FrostWalker | Self::Knockback | Self::FireAspect | Self::Punch => 2,
            Self::AquaAffinity
            | Self::CurseOfBinding
            | Self::SilkTouch
            | Self::Flame
            | Self::Infinity
            | Self::Channeling
            | Self::Multishot
            | Self::Mending
            | Self::CurseOfVanishing => 1,
        }
    }

    /// returns the maximum level that can be obtained in survival gameplay
    pub fn max_obtainable(&self) -> u32 {
        match self {
            Self::WindBurst => 1,
            _ => self.max_level(),
        }
    }

    /// returns `true` if this enchantment conflicts with any of the given ones.
    /// conflicting enchantments means they cannot be applied together (e.g. Silk Touch and Fortune)
    pub fn is_conflicting_with(&self, existing: &Vec<&Enchantment>) -> bool {
        for group in Self::CONFLICTING_GROUPS {
            // if this enchantment is in the group,
            if group.contains(&self) {
                // check if any of the existing enchantments are also in the group.
                for enchantment in existing {
                    if group.contains(enchantment) {
                        // if they are, then the enchantments are conflicting.
                        return true;
                    }
                }
            }
        }

        false
    }

    /// returns `true` if this enchantment is a curse
    pub fn is_curse(&self) -> bool {
        self == &Self::CurseOfBinding || self == &Self::CurseOfVanishing
    }

    /// the level multiplier for this enchantment on java edition.
    /// this value varies depending on if the source item is a book or not.
    pub fn java_multiplier(&self, from_book: bool) -> u32 {
        let anvil_multiplier = match self {
            Self::Protection
            | Self::Sharpness
            | Self::Efficiency
            | Self::Power
            | Self::Piercing => 1,
            Self::FireProtection
            | Self::FeatherFalling
            | Self::ProjectileProtection
            | Self::Smite
            | Self::BaneOfArthropods
            | Self::Knockback
            | Self::Unbreaking
            | Self::Loyalty
            | Self::QuickCharge
            | Self::Density => 2,
            Self::BlastProtection
            | Self::Respiration
            | Self::AquaAffinity
            | Self::DepthStrider
            | Self::FrostWalker
            | Self::FireAspect
            | Self::Looting
            | Self::SweepingEdge
            | Self::Fortune
            | Self::Punch
            | Self::Flame
            | Self::LuckOfTheSea
            | Self::Lure
            | Self::Impaling
            | Self::Riptide
            | Self::Multishot
            | Self::Breach
            | Self::WindBurst
            | Self::Mending => 4,
            Self::Thorns
            | Self::CurseOfBinding
            | Self::SoulSpeed
            | Self::SwiftSneak
            | Self::SilkTouch
            | Self::Infinity
            | Self::Channeling
            | Self::CurseOfVanishing => 8,
        };

        if from_book {
            (anvil_multiplier / 2).max(1)
        } else {
            anvil_multiplier
        }
    }

    /// the level multiplier for this enchantment on bedrock platforms.
    /// this value varies depending on if the source item is a book or not.
    pub fn bedrock_multiplier(&self, from_book: bool) -> u32 {
        let java_multipler = self.java_multiplier(from_book);

        if self == &Self::Loyalty || self == &Self::Impaling {
            java_multipler / 2
        } else {
            java_multipler
        }
    }

    /// the index into a friendly ordering of enchantments
    pub fn friendly_index(&self) -> usize {
        use Enchantment::*;

        macro_rules! map {
            ($($ench: expr,)+) => {
                LazyLock::new(#[allow(unused_assignments)] || {
                    let mut map = HashMap::new();
                    let mut i = 0;
                    $(
                        map.insert($ench, i);
                        i += 1;
                    )*
                    map
                })
            };
        }

        static FRIENDLY: LazyLock<HashMap<Enchantment, usize>> = map![
            // generic
            Unbreaking,
            Mending,
            // tool
            Efficiency,
            Fortune,
            SilkTouch,
            // weapon
            Sharpness,
            Looting,
            FireAspect,
            Knockback,
            SweepingEdge,
            // bow
            Power,
            Infinity,
            Flame,
            Punch,
            // armor
            Protection,
            FireProtection,
            BlastProtection,
            ProjectileProtection,
            FeatherFalling,
            Thorns,
            // piece specific armor
            AquaAffinity,
            Respiration,
            SwiftSneak,
            DepthStrider,
            SoulSpeed,
            FrostWalker,
            // mace
            Density,
            Breach,
            WindBurst,
            // trident
            Impaling,
            Loyalty,
            Riptide,
            Channeling,
            // crossbow
            Piercing,
            QuickCharge,
            Multishot,
            // fishing rod
            LuckOfTheSea,
            Lure,
            // rarely used
            Smite,
            BaneOfArthropods,
            // curses
            CurseOfBinding,
            CurseOfVanishing,
        ];

        FRIENDLY[self]
    }

    pub fn friendly_sort(
        iter: impl Iterator<Item = Enchantment>,
    ) -> impl Iterator<Item = Enchantment> {
        iter.sorted_by_cached_key(|e| e.friendly_index())
    }

    pub fn friendly_sort_with(
        iter: impl Iterator<Item = Enchantment>,
        item: &Item,
    ) -> impl Iterator<Item = Enchantment> {
        Self::friendly_sort(iter).sorted_by_cached_key(|e| !item.is_compatible(&e))
    }
}

impl Display for Enchantment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prettify_pascal_case(format!("{self:?}")))
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use super::Enchantment;

    #[test]
    fn all_enchantments_are_friendly() {
        Enchantment::iter().for_each(|e| {
            println!("{} => {}", e, e.friendly_index());
        });
    }
}
