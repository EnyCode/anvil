#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Enchantment {
    Protection,
    FireProtection,
    FeatherFalling,
    BlastProtection,
    ProjectileProtection,
    Thorns,
    Respiration,
    DepthStrider,
    AquaAffinity,
    Sharpness,
    Smite,
    BaneOfArthropods,
    Knockback,
    FireAspect,
    Looting,
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
    FrostWalker,
    Mending,
    CurseOfBinding,
    CurseOfVanishing,
    Impaling,
    Riptide,
    Loyalty,
    Channeling,
    Multishot,
    Piercing,
    Density,
    Breach,
    WindBurst,
    QuickCharge,
    SoulSpeed,
    SwiftSneak,
    SweepingEdge,
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
        &[Self::Riptide, Self::Loyalty],
        &[Self::Riptide, Self::Channeling],
        &[Self::Multishot, Self::Piercing],
    ];

    pub fn max_level(&self) -> u32 {
        match self {
            Self::Sharpness
            | Self::Smite
            | Self::BaneOfArthropods
            | Self::Efficiency
            | Self::Power
            | Self::Density
            | Self::Impaling => 5,
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
            Self::Knockback | Self::FireAspect | Self::Punch | Self::FrostWalker => 2,
            Self::AquaAffinity
            | Self::SilkTouch
            | Self::Flame
            | Self::Infinity
            | Self::Mending
            | Self::CurseOfBinding
            | Self::CurseOfVanishing
            | Self::Channeling
            | Self::Multishot => 1,
        }
    }

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

    pub fn bedrock_multiplier(&self, from_book: bool) -> u32 {
        let java_multipler = self.java_multiplier(from_book);

        if self == &Self::Impaling {
            java_multipler / 2
        } else {
            java_multipler
        }
    }
}
