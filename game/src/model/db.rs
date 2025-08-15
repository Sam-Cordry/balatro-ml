use serde::{Deserialize, Serialize};
use sqlx_pg_uint::PgU64;

use crate::model::{
    cards::{CardEdition, Enhancement, Rank, Seal, Suit},
    planets::Planet,
    spectrals::Spectral,
    tarots::Tarot,
    traits, HandType, JokerEdition,
};

#[derive(Debug, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[sqlx(type_name = "consumable")]
pub enum ConsumableType {
    #[sqlx(rename = "The Fool")]
    Fool,
    #[sqlx(rename = "The Magician")]
    Magician,
    #[sqlx(rename = "The High Priestess")]
    Priestess,
    #[sqlx(rename = "The Empress")]
    Empress,
    #[sqlx(rename = "The Emperor")]
    Emperor,
    #[sqlx(rename = "The Hierophant")]
    Hierophant,
    #[sqlx(rename = "The Lovers")]
    Lovers,
    #[sqlx(rename = "The Chariot")]
    Chariot,
    Justice,
    #[sqlx(rename = "The Hermit")]
    Hermit,
    #[sqlx(rename = "The Wheel of Fortune")]
    Wheel,
    Strength,
    #[sqlx(rename = "The Hanged Man")]
    Hanged,
    Death,
    Temperance,
    #[sqlx(rename = "The Devil")]
    Devil,
    #[sqlx(rename = "The Tower")]
    Tower,
    #[sqlx(rename = "The Star")]
    Star,
    #[sqlx(rename = "The Moon")]
    Moon,
    #[sqlx(rename = "The Sun")]
    Sun,
    Judgement,
    #[sqlx(rename = "The World")]
    World,
    Pluto,
    Mercury,
    Uranus,
    Venus,
    Saturn,
    Jupiter,
    Earth,
    Mars,
    Neptune,
    #[sqlx(rename = "Planet X")]
    PlanetX,
    Ceres,
    Eris,
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    #[sqlx(rename = "Deja Vu")]
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    #[sqlx(rename = "The Soul")]
    Soul,
    #[sqlx(rename = "Black Hole")]
    BlackHole,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "joker")]
pub enum JokerType {
    Joker,
    #[sqlx(rename = "Greedy Joker")]
    Greedy,
    #[sqlx(rename = "Lusty Joker")]
    Lusty,
    #[sqlx(rename = "Wrathful Joker")]
    Wrathful,
    #[sqlx(rename = "Gluttonous Joker")]
    Gluttonous,
    #[sqlx(rename = "Jolly Joker")]
    Jolly,
    #[sqlx(rename = "Zany Joker")]
    Zany,
    #[sqlx(rename = "Mad Joker")]
    Mad,
    #[sqlx(rename = "Crazy Joker")]
    Crazy,
    #[sqlx(rename = "Droll Joker")]
    Droll,
    #[sqlx(rename = "Sly Joker")]
    Sly,
    #[sqlx(rename = "Wily Joker")]
    Wily,
    #[sqlx(rename = "Clever Joker")]
    Clever,
    #[sqlx(rename = "Devious Joker")]
    Devious,
    #[sqlx(rename = "Crafty Joker")]
    Crafty,
    #[sqlx(rename = "Half Joker")]
    Half,
    #[sqlx(rename = "Joker Stencil")]
    Stencil,
    #[sqlx(rename = "Four Fingers")]
    Fingers,
    Mime,
    #[sqlx(rename = "Credit Card")]
    Credit,
    #[sqlx(rename = "Ceremonial Dagger")]
    Dagger,
    Banner,
    #[sqlx(rename = "Mystic Summit")]
    Mystic,
    #[sqlx(rename = "Marble Joker")]
    Marble,
    #[sqlx(rename = "Loyalty Card")]
    Loyalty,
    #[sqlx(rename = "8 Ball")]
    Ball,
    Misprint,
    Dusk,
    #[sqlx(rename = "Raised Fist")]
    Fist,
    #[sqlx(rename = "Chaos the Clown")]
    Chaos,
    Fibonacci,
    #[sqlx(rename = "Steel Joker")]
    Steel,
    #[sqlx(rename = "Scary Face")]
    Scary,
    #[sqlx(rename = "Abstract Joker")]
    Abstract,
    #[sqlx(rename = "Delayed Gratification")]
    Gratification,
    Hack,
    Pareidolia,
    #[sqlx(rename = "Gros Michel")]
    Michel,
    #[sqlx(rename = "Even Steven")]
    Steven,
    #[sqlx(rename = "Odd Todd")]
    Todd,
    Scholar,
    #[sqlx(rename = "Business Card")]
    Business,
    Supernova,
    #[sqlx(rename = "Ride the Bus")]
    Bus,
    #[sqlx(rename = "Space Joker")]
    Space,
    Egg,
    Burglar,
    Blackboard,
    Runner,
    #[sqlx(rename = "Ice Cream")]
    Cream,
    #[sqlx(rename = "DNA")]
    Dna,
    Splash,
    #[sqlx(rename = "Blue Joker")]
    Blue,
    #[sqlx(rename = "Sixth Sense")]
    Sixth,
    Constellation,
    Hiker,
    #[sqlx(rename = "Faceless Joker")]
    Faceless,
    #[sqlx(rename = "Green Joker")]
    Green,
    Superposition,
    #[sqlx(rename = "To Do List")]
    List,
    Cavendish,
    #[sqlx(rename = "Card Sharp")]
    Sharp,
    #[sqlx(rename = "Red Card")]
    Red,
    Madness,
    #[sqlx(rename = "Square Joker")]
    Square,
    Seance,
    #[sqlx(rename = "Riff-Raff")]
    Riff,
    Vampire,
    Shortcut,
    Hologram,
    Vagabond,
    Baron,
    #[sqlx(rename = "Cloud 9")]
    Cloud,
    Rocket,
    Obelisk,
    #[sqlx(rename = "Midas Mask")]
    Midas,
    Luchador,
    Photograph,
    #[sqlx(rename = "Gift Card")]
    Gift,
    #[sqlx(rename = "Turtle Bean")]
    Turtle,
    Erosion,
    #[sqlx(rename = "Reserved Parking")]
    Parking,
    #[sqlx(rename = "Mail-In Rebate")]
    Rebate,
    #[sqlx(rename = "To the Moon")]
    Moon,
    Hallucination,
    #[sqlx(rename = "Fortune Teller")]
    Fortune,
    Juggler,
    Drunkard,
    #[sqlx(rename = "Stone Joker")]
    Stone,
    #[sqlx(rename = "Golden Joker")]
    Golden,
    #[sqlx(rename = "Lucky Cat")]
    Lucky,
    #[sqlx(rename = "Baseball Card")]
    Baseball,
    Bull,
    #[sqlx(rename = "Diet Cola")]
    Cola,
    #[sqlx(rename = "Trading Card")]
    Trading,
    #[sqlx(rename = "Flash Card")]
    Flash,
    Popcorn,
    #[sqlx(rename = "Spare Trousers")]
    Trousers,
    #[sqlx(rename = "Ancient Joker")]
    Ancient,
    Ramen,
    #[sqlx(rename = "Walkie Talkie")]
    Walkie,
    Seltzer,
    Castle,
    #[sqlx(rename = "Smiley Face")]
    Smiley,
    Campfire,
    #[sqlx(rename = "Golden Ticket")]
    Ticket,
    #[sqlx(rename = "Mr. Bones")]
    Bones,
    Acrobat,
    #[sqlx(rename = "Sock and Buskin")]
    Sock,
    Swashbuckler,
    Troubadour,
    Certificate,
    #[sqlx(rename = "Smeared Joker")]
    Smeared,
    Throwback,
    #[sqlx(rename = "Hanging Chad")]
    Chad,
    #[sqlx(rename = "Rough Gem")]
    Gem,
    Bloodstone,
    Arrowhead,
    #[sqlx(rename = "Onyx Agate")]
    Onyx,
    #[sqlx(rename = "Glass Joker")]
    Glass,
    Showman,
    #[sqlx(rename = "Flower Pot")]
    Flower,
    Blueprint,
    #[sqlx(rename = "Wee Joker")]
    Wee,
    #[sqlx(rename = "Merry Andy")]
    Andy,
    #[sqlx(rename = "Oops! All 6s")]
    Oops,
    #[sqlx(rename = "The Idol")]
    Idol,
    #[sqlx(rename = "Seeing Double")]
    Double,
    Matador,
    #[sqlx(rename = "Hit the Road")]
    Road,
    #[sqlx(rename = "The Duo")]
    Duo,
    #[sqlx(rename = "The Trio")]
    Trio,
    #[sqlx(rename = "The Family")]
    Family,
    #[sqlx(rename = "The Order")]
    Order,
    #[sqlx(rename = "The Tribe")]
    Tribe,
    Stuntman,
    #[sqlx(rename = "Invisible Joker")]
    Invisible,
    Brainstorm,
    Satellite,
    #[sqlx(rename = "Shoot the Moon")]
    Shoot,
    #[sqlx(rename = "Drivers License")]
    License,
    Cartomancer,
    Astronomer,
    #[sqlx(rename = "Burnt Joker")]
    Burnt,
    Bootstraps,
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct CardRow {
    pub session: i32,
    pub rank: Rank,
    pub suit: Suit,
    pub chips: i32,
    pub enhancement: Option<Enhancement>,
    pub edition: CardEdition,
    pub seal: Option<Seal>,
    pub in_deck: bool,
    pub in_hand: bool,
    pub index: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ConsumableRow {
    pub session: i32,
    pub consumable: ConsumableType,
    pub negative: bool,
}

impl From<ConsumableRow> for Box<dyn traits::Consumable> {
    fn from(val: ConsumableRow) -> Self {
        match val.consumable {
            ConsumableType::Fool => Box::new(Tarot::Fool(val.negative)),
            ConsumableType::Magician => Box::new(Tarot::Magician(val.negative)),
            ConsumableType::Priestess => Box::new(Tarot::Priestess(val.negative)),
            ConsumableType::Empress => Box::new(Tarot::Empress(val.negative)),
            ConsumableType::Emperor => Box::new(Tarot::Emperor(val.negative)),
            ConsumableType::Hierophant => Box::new(Tarot::Hierophant(val.negative)),
            ConsumableType::Lovers => Box::new(Tarot::Lovers(val.negative)),
            ConsumableType::Chariot => Box::new(Tarot::Chariot(val.negative)),
            ConsumableType::Justice => Box::new(Tarot::Justice(val.negative)),
            ConsumableType::Hermit => Box::new(Tarot::Hermit(val.negative)),
            ConsumableType::Wheel => Box::new(Tarot::Wheel(val.negative)),
            ConsumableType::Strength => Box::new(Tarot::Strength(val.negative)),
            ConsumableType::Hanged => Box::new(Tarot::Hanged(val.negative)),
            ConsumableType::Death => Box::new(Tarot::Death(val.negative)),
            ConsumableType::Temperance => Box::new(Tarot::Temperance(val.negative)),
            ConsumableType::Devil => Box::new(Tarot::Devil(val.negative)),
            ConsumableType::Tower => Box::new(Tarot::Tower(val.negative)),
            ConsumableType::Star => Box::new(Tarot::Star(val.negative)),
            ConsumableType::Moon => Box::new(Tarot::Moon(val.negative)),
            ConsumableType::Sun => Box::new(Tarot::Sun(val.negative)),
            ConsumableType::Judgement => Box::new(Tarot::Judgement(val.negative)),
            ConsumableType::World => Box::new(Tarot::World(val.negative)),
            ConsumableType::Pluto => Box::new(Planet::Pluto(val.negative)),
            ConsumableType::Mercury => Box::new(Planet::Mercury(val.negative)),
            ConsumableType::Uranus => Box::new(Planet::Uranus(val.negative)),
            ConsumableType::Venus => Box::new(Planet::Venus(val.negative)),
            ConsumableType::Saturn => Box::new(Planet::Saturn(val.negative)),
            ConsumableType::Jupiter => Box::new(Planet::Jupiter(val.negative)),
            ConsumableType::Earth => Box::new(Planet::Earth(val.negative)),
            ConsumableType::Mars => Box::new(Planet::Mars(val.negative)),
            ConsumableType::Neptune => Box::new(Planet::Neptune(val.negative)),
            ConsumableType::PlanetX => Box::new(Planet::PlanetX(val.negative)),
            ConsumableType::Ceres => Box::new(Planet::Ceres(val.negative)),
            ConsumableType::Eris => Box::new(Planet::Eris(val.negative)),
            ConsumableType::Familiar => Box::new(Spectral::Familiar(val.negative)),
            ConsumableType::Grim => Box::new(Spectral::Grim(val.negative)),
            ConsumableType::Incantation => Box::new(Spectral::Incantation(val.negative)),
            ConsumableType::Talisman => Box::new(Spectral::Talisman(val.negative)),
            ConsumableType::Aura => Box::new(Spectral::Aura(val.negative)),
            ConsumableType::Wraith => Box::new(Spectral::Wraith(val.negative)),
            ConsumableType::Sigil => Box::new(Spectral::Sigil(val.negative)),
            ConsumableType::Ouija => Box::new(Spectral::Ouija(val.negative)),
            ConsumableType::Ectoplasm => Box::new(Spectral::Ectoplasm(val.negative)),
            ConsumableType::Immolate => Box::new(Spectral::Immolate(val.negative)),
            ConsumableType::Ankh => Box::new(Spectral::Ankh(val.negative)),
            ConsumableType::DejaVu => Box::new(Spectral::DejaVu(val.negative)),
            ConsumableType::Hex => Box::new(Spectral::Hex(val.negative)),
            ConsumableType::Trance => Box::new(Spectral::Trance(val.negative)),
            ConsumableType::Medium => Box::new(Spectral::Medium(val.negative)),
            ConsumableType::Cryptid => Box::new(Spectral::Cryptid(val.negative)),
            ConsumableType::Soul => Box::new(Spectral::Soul(val.negative)),
            ConsumableType::BlackHole => Box::new(Spectral::BlackHole(val.negative)),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct JokerRow {
    pub session: i32,
    pub joker: JokerType,
    pub edition: JokerEdition,
    pub index: i32,
    pub mult: Option<i32>,
    pub xmult: Option<f32>,
    pub chips: Option<i32>,
    pub rank: Option<Rank>,
    pub suit: Option<Suit>,
    pub money: Option<i32>,
    pub sell_value: i32,
    pub hands: Option<i32>,
    pub hand_type: Option<HandType>,
    pub hand_size: Option<i32>,
    pub rounds: Option<i32>,
    pub discards: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionRow {
    pub session: i32,
    pub money: i32,
    pub ante: i32,
    pub rounds_completed: i32,
    pub rounds_skipped: i32,
    pub hands: i32,
    pub discards: i32,
    pub hand_size: i32,
    pub last_consumable: Option<ConsumableType>,
    pub high_card_level: i32,
    pub pair_level: i32,
    pub two_pair_level: i32,
    pub three_of_a_kind_level: i32,
    pub straight_level: i32,
    pub flush_level: i32,
    pub full_house_level: i32,
    pub four_of_a_kind_level: i32,
    pub straight_flush_level: i32,
    pub five_of_a_kind_level: i32,
    pub flush_house_level: i32,
    pub flush_five_level: i32,
    pub high_card_played: i32,
    pub pair_played: i32,
    pub two_pair_played: i32,
    pub three_of_a_kind_played: i32,
    pub straight_played: i32,
    pub flush_played: i32,
    pub full_house_played: i32,
    pub four_of_a_kind_played: i32,
    pub straight_flush_played: i32,
    pub five_of_a_kind_played: i32,
    pub flush_house_played: i32,
    pub flush_five_played: i32,
    pub pluto_used: bool,
    pub mercury_used: bool,
    pub uranus_used: bool,
    pub venus_used: bool,
    pub saturn_used: bool,
    pub jupiter_used: bool,
    pub earth_used: bool,
    pub mars_used: bool,
    pub neptune_used: bool,
    pub planet_x_used: bool,
    pub ceres_used: bool,
    pub eris_used: bool,
    pub overstock_redeemed: bool,
    pub overstock_plus_redeemed: bool,
    pub clearance_sale_redeemed: bool,
    pub liquidation_redeemed: bool,
    pub hone_redeemed: bool,
    pub glow_up_redeemed: bool,
    pub reroll_surplus_redeemed: bool,
    pub reroll_glut_redeemed: bool,
    pub crystal_ball_redeemed: bool,
    pub omen_globe_redeemed: bool,
    pub telescope_redeemed: bool,
    pub observatory_redeemed: bool,
    pub grabber_redeemed: bool,
    pub nacho_tong_redeemed: bool,
    pub wasteful_redeemed: bool,
    pub recyclomancy_redeemed: bool,
    pub tarot_merchant_redeemed: bool,
    pub tarot_tycoon_redeemed: bool,
    pub planet_merchant_redeemed: bool,
    pub planet_tycoon_redeemed: bool,
    pub seed_money_redeemed: bool,
    pub money_tree_redeemed: bool,
    pub blank_redeemed: bool,
    pub antimatter_redeemed: bool,
    pub magic_trick_redeemed: bool,
    pub illusion_redeemed: bool,
    pub hieroglyph_redeemed: bool,
    pub petroglyph_redeemed: bool,
    pub directors_cut_redeemed: bool,
    pub retcon_redeemed: bool,
    pub paint_brush_redeemed: bool,
    pub palette_redeemed: bool,
    pub seed: PgU64,
    pub consumable_slots: i32,
    pub joker_slots: i32,
    pub hands_remaining: i32,
    pub discards_remaining: i32,
    pub tarots_used: i32,
}
