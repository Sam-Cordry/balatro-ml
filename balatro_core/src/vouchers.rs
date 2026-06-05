#[derive(Debug, PartialEq)]
enum VoucherEffect {
    CardSlots(i32),
    PriceMultiplier(f32),
    EditionAppearanceRate(u32),
    RerollCost(i32),
    ConsumableSlots(i32),
    SpectralCardsInArcanaPacks,
    CelestialPacksContainMostPlayedHand,
    PlanetCardsGiveMultiplier,
    HandsPerRound(i32),
    DiscardsPerRound(i32),
    TarotCardFrequency(u32),
    PlanetCardFrequency(u32),
    MaxInterest(u32),
    NoEffect,
    JokerSlots(i32),
    PlayingCardsInShop,
    EnhancedPlayingCardsInShop,
    Composite(Vec<VoucherEffect>),
    Ante(i32),
    OneBossReroll,
    UnlimitedBossRerolls,
    HandSize(i32),
}

#[derive(Debug, PartialEq)]
enum Voucher {
    Overstock,
    OverstockPlus,
    ClearanceSale,
    Liquidation,
    Hone,
    GlowUp,
    RerollSurplus,
    RerollGlut,
    CrystalBall,
    OmenGlobe,
    Telescope,
    Observatory,
    Grabber,
    NachoTong,
    Wasteful,
    Recyclomancy,
    TarotMerchant,
    TarotTycoon,
    PlanetMerchant,
    PlanetTycoon,
    SeedMoney,
    MoneyTree,
    Blank,
    Antimatter,
    MagicTrick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    DirectorsCut,
    Retcon,
    PaintBrush,
    Palette,
}

impl Voucher {
    fn get_effect(&self) -> VoucherEffect {
        match self {
            Self::Overstock => VoucherEffect::CardSlots(1),
            Self::OverstockPlus => VoucherEffect::CardSlots(1),
            Self::ClearanceSale => VoucherEffect::PriceMultiplier(0.75),
            Self::Liquidation => VoucherEffect::PriceMultiplier(0.5),
            Self::Hone => VoucherEffect::EditionAppearanceRate(2),
            Self::GlowUp => VoucherEffect::EditionAppearanceRate(4),
            Self::RerollSurplus => VoucherEffect::RerollCost(-2),
            Self::RerollGlut => VoucherEffect::RerollCost(-2),
            Self::CrystalBall => VoucherEffect::ConsumableSlots(1),
            Self::OmenGlobe => VoucherEffect::SpectralCardsInArcanaPacks,
            Self::Telescope => VoucherEffect::CelestialPacksContainMostPlayedHand,
            Self::Observatory => VoucherEffect::PlanetCardsGiveMultiplier,
            Self::Grabber => VoucherEffect::HandsPerRound(1),
            Self::NachoTong => VoucherEffect::HandsPerRound(1),
            Self::Wasteful => VoucherEffect::DiscardsPerRound(1),
            Self::Recyclomancy => VoucherEffect::DiscardsPerRound(1),
            Self::TarotMerchant => VoucherEffect::TarotCardFrequency(2),
            Self::TarotTycoon => VoucherEffect::TarotCardFrequency(2),
            Self::PlanetMerchant => VoucherEffect::PlanetCardFrequency(2),
            Self::PlanetTycoon => VoucherEffect::PlanetCardFrequency(2),
            Self::SeedMoney => VoucherEffect::MaxInterest(10),
            Self::MoneyTree => VoucherEffect::MaxInterest(20),
            Self::Blank => VoucherEffect::NoEffect,
            Self::Antimatter => VoucherEffect::JokerSlots(1),
            Self::MagicTrick => VoucherEffect::PlayingCardsInShop,
            Self::Illusion => VoucherEffect::EnhancedPlayingCardsInShop,
            Self::Hieroglyph => VoucherEffect::Composite(vec![
                VoucherEffect::Ante(-1),
                VoucherEffect::HandsPerRound(-1),
            ]),
            Self::Petroglyph => VoucherEffect::Composite(vec![
                VoucherEffect::Ante(-1),
                VoucherEffect::DiscardsPerRound(-1),
            ]),
            Self::DirectorsCut => VoucherEffect::OneBossReroll,
            Self::Retcon => VoucherEffect::UnlimitedBossRerolls,
            Self::PaintBrush => VoucherEffect::HandSize(1),
            Self::Palette => VoucherEffect::HandSize(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_overstock_effect() {
        assert_matches!(Voucher::Overstock.get_effect(), VoucherEffect::CardSlots(1));
    }

    #[test]
    fn test_overstick_plus_effect() {
        assert_matches!(
            Voucher::OverstockPlus.get_effect(),
            VoucherEffect::CardSlots(1)
        );
    }

    #[test]
    fn test_clearance_sale_effect() {
        assert_matches!(
            Voucher::ClearanceSale.get_effect(),
            VoucherEffect::PriceMultiplier(0.75)
        );
    }

    #[test]
    fn test_liquidation_effect() {
        assert_matches!(
            Voucher::Liquidation.get_effect(),
            VoucherEffect::PriceMultiplier(0.5)
        );
    }

    #[test]
    fn test_hone_effect() {
        assert_matches!(
            Voucher::Hone.get_effect(),
            VoucherEffect::EditionAppearanceRate(2)
        );
    }

    #[test]
    fn test_glow_up_effect() {
        assert_matches!(
            Voucher::GlowUp.get_effect(),
            VoucherEffect::EditionAppearanceRate(4)
        );
    }

    #[test]
    fn test_reroll_surplus_effect() {
        assert_matches!(
            Voucher::RerollSurplus.get_effect(),
            VoucherEffect::RerollCost(-2)
        );
    }

    #[test]
    fn test_reroll_glut_effect() {
        assert_matches!(
            Voucher::RerollGlut.get_effect(),
            VoucherEffect::RerollCost(-2)
        );
    }

    #[test]
    fn test_crystal_ball_effect() {
        assert_matches!(
            Voucher::CrystalBall.get_effect(),
            VoucherEffect::ConsumableSlots(1)
        );
    }

    #[test]
    fn test_omen_globe_effect() {
        assert_matches!(
            Voucher::OmenGlobe.get_effect(),
            VoucherEffect::SpectralCardsInArcanaPacks
        );
    }

    #[test]
    fn test_telescope_effect() {
        assert_matches!(
            Voucher::Telescope.get_effect(),
            VoucherEffect::CelestialPacksContainMostPlayedHand
        );
    }

    #[test]
    fn test_observatory_effect() {
        assert_matches!(
            Voucher::Observatory.get_effect(),
            VoucherEffect::PlanetCardsGiveMultiplier
        );
    }

    #[test]
    fn test_grabber_effect() {
        assert_matches!(
            Voucher::Grabber.get_effect(),
            VoucherEffect::HandsPerRound(1)
        );
    }

    #[test]
    fn test_nacho_tong() {
        assert_matches!(
            Voucher::NachoTong.get_effect(),
            VoucherEffect::HandsPerRound(1)
        );
    }

    #[test]
    fn test_wasteful_effect() {
        assert_matches!(
            Voucher::Wasteful.get_effect(),
            VoucherEffect::DiscardsPerRound(1)
        );
    }

    #[test]
    fn test_recyclomancy_effect() {
        assert_matches!(
            Voucher::Recyclomancy.get_effect(),
            VoucherEffect::DiscardsPerRound(1)
        );
    }

    #[test]
    fn test_tarot_merchant_effect() {
        assert_matches!(
            Voucher::TarotMerchant.get_effect(),
            VoucherEffect::TarotCardFrequency(2)
        );
    }

    #[test]
    fn test_tarot_tycoon_effect() {
        assert_matches!(
            Voucher::TarotTycoon.get_effect(),
            VoucherEffect::TarotCardFrequency(2)
        );
    }

    #[test]
    fn test_planet_merchant_effect() {
        assert_matches!(
            Voucher::PlanetMerchant.get_effect(),
            VoucherEffect::PlanetCardFrequency(2)
        );
    }

    #[test]
    fn test_planet_tycoon_effect() {
        assert_matches!(
            Voucher::PlanetTycoon.get_effect(),
            VoucherEffect::PlanetCardFrequency(2)
        );
    }

    #[test]
    fn test_seed_money_effect() {
        assert_matches!(
            Voucher::SeedMoney.get_effect(),
            VoucherEffect::MaxInterest(10)
        );
    }

    #[test]
    fn test_money_tree_effect() {
        assert_matches!(
            Voucher::MoneyTree.get_effect(),
            VoucherEffect::MaxInterest(20)
        );
    }

    #[test]
    fn test_blank_effect() {
        assert_matches!(Voucher::Blank.get_effect(), VoucherEffect::NoEffect);
    }

    #[test]
    fn test_antimatter_effect() {
        assert_matches!(
            Voucher::Antimatter.get_effect(),
            VoucherEffect::JokerSlots(1)
        );
    }

    #[test]
    fn test_magic_trick_effect() {
        assert_matches!(
            Voucher::MagicTrick.get_effect(),
            VoucherEffect::PlayingCardsInShop
        );
    }

    #[test]
    fn test_illusion_effect() {
        assert_matches!(
            Voucher::Illusion.get_effect(),
            VoucherEffect::EnhancedPlayingCardsInShop
        );
    }

    #[test]
    fn test_hieroglyph_effect() {
        assert_matches!(
            Voucher::Hieroglyph.get_effect(),
            VoucherEffect::Composite(effects)
                if effects.contains(&VoucherEffect::Ante(-1)) &&
                    effects.contains(&VoucherEffect::HandsPerRound(-1))
        );
    }

    #[test]
    fn test_petroglyph_effect() {
        assert_matches!(
            Voucher::Petroglyph.get_effect(),
            VoucherEffect::Composite(effects)
                if effects.contains(&VoucherEffect::Ante(-1)) &&
                    effects.contains(&VoucherEffect::DiscardsPerRound(-1))
        );
    }

    #[test]
    fn test_directors_cut_effect() {
        assert_matches!(
            Voucher::DirectorsCut.get_effect(),
            VoucherEffect::OneBossReroll
        );
    }

    #[test]
    fn test_recon_effect() {
        assert_matches!(
            Voucher::Retcon.get_effect(),
            VoucherEffect::UnlimitedBossRerolls
        );
    }

    #[test]
    fn test_paint_brush_effect() {
        assert_matches!(Voucher::PaintBrush.get_effect(), VoucherEffect::HandSize(1));
    }

    #[test]
    fn test_palette_effect() {
        assert_matches!(Voucher::Palette.get_effect(), VoucherEffect::HandSize(1));
    }
}
