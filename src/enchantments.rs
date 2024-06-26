#[derive(Copy, Clone, PartialEq)]
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
    QuickCharge,
    SoulSpeed,
    SwiftSneak,
    SweepingEdge,
}

impl Enchantment {
    pub fn max_level(&self) -> u32 {
        match self {
            Self::Sharpness
            | Self::Smite
            | Self::BaneOfArthropods
            | Self::Efficiency
            | Self::Power
            | Self::Impaling => 5,
            Self::Protection
            | Self::FireProtection
            | Self::FeatherFalling
            | Self::BlastProtection
            | Self::ProjectileProtection
            | Self::Piercing => 4,
            Self::Thorns
            | Self::Respiration
            | Self::DepthStrider
            | Self::Looting
            | Self::Unbreaking
            | Self::Fortune
            | Self::LuckOfTheSea
            | Self::Lure
            | Self::Riptide
            | Self::Loyalty
            | Self::QuickCharge
            | Self::SoulSpeed
            | Self::SwiftSneak
            | Self::SweepingEdge => 3,
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

    pub fn java_multiplier(&self, from_book: bool) -> u32 {
        let book_multiplier = match self {
            Self::Protection
            | Self::FireProtection
            | Self::FeatherFalling
            | Self::ProjectileProtection
            | Self::Sharpness
            | Self::Smite
            | Self::BaneOfArthropods
            | Self::Knockback
            | Self::Efficiency
            | Self::Unbreaking
            | Self::Power
            | Self::Loyalty
            | Self::Piercing
            | Self::QuickCharge => 1,
            Self::BlastProtection
            | Self::Respiration
            | Self::DepthStrider
            | Self::AquaAffinity
            | Self::FireAspect
            | Self::Looting
            | Self::Fortune
            | Self::Punch
            | Self::Flame
            | Self::LuckOfTheSea
            | Self::Lure
            | Self::FrostWalker
            | Self::Mending
            | Self::Impaling
            | Self::Riptide
            | Self::Multishot
            | Self::SweepingEdge => 2,
            Self::Thorns
            | Self::SilkTouch
            | Self::Infinity
            | Self::CurseOfBinding
            | Self::CurseOfVanishing
            | Self::Channeling
            | Self::SoulSpeed
            | Self::SwiftSneak => 4,
        };

        if from_book {
            book_multiplier
        } else {
            match self {
                Self::Protection
                | Self::Sharpness
                | Self::Efficiency
                | Self::Power
                | Self::Loyalty
                | Self::Piercing => 1,
                _ => book_multiplier * 2,
            }
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
