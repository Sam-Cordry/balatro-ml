use crate::model::cards::{CardEdition, Enhancement, Rank, Seal, Suit};

use super::JokerEdition;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "consumable")]
pub enum Consumable {
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
    DNA,
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

#[derive(Debug, sqlx::FromRow)]
pub struct CardRow {
    session: i32,
    rank: Rank,
    suit: Suit,
    chips: i32,
    enhancement: Option<Enhancement>,
    edition: CardEdition,
    seal: Option<Seal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ConsumableRow {
    session: i32,
    consumable: Consumable,
    negative: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct JokerRow {
    session: i32,
    joker: JokerType,
    edition: JokerEdition,
    index: i32,
    mult: Option<i32>,
    xmult: Option<f32>,
    chips: Option<i32>,
    rank: Option<Rank>,
    suit: Option<Suit>,
    money: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionRow {
    session: i32,
    money: i32,
    ante: i32,
    rounds_completed: i32,
    rounds_skipped: i32,
    hands: i32,
    discards: i32,
    hand_size: i32,
    last_consumable: Option<Consumable>,
    high_card_level: i32,
    pair_level: i32,
    two_pair_level: i32,
    three_of_a_kind_level: i32,
    straight_level: i32,
    flush_level: i32,
    full_house_level: i32,
    four_of_a_kind_level: i32,
    straight_flush_level: i32,
    five_of_a_kind_level: i32,
    flush_house_level: i32,
    flush_five_level: i32,
    high_card_played: i32,
    pair_played: i32,
    two_pair_played: i32,
    three_of_a_kind_played: i32,
    straight_played: i32,
    flush_played: i32,
    full_house_played: i32,
    four_of_a_kind_played: i32,
    straight_flush_played: i32,
    five_of_a_kind_played: i32,
    flush_house_played: i32,
    flush_five_played: i32,
    pluto_used: bool,
    mercury_used: bool,
    uranus_used: bool,
    venus_used: bool,
    saturn_used: bool,
    jupiter_used: bool,
    earth_used: bool,
    mars_used: bool,
    neptune_used: bool,
    planet_x_used: bool,
    ceres_used: bool,
    eris_used: bool,
    overstock_redeemed: bool,
    overstock_plus_redeemed: bool,
    clearance_sale_redeemed: bool,
    liquidation_redeemed: bool,
    hone_redeemed: bool,
    glow_up_redeemed: bool,
    reroll_surplus_redeemed: bool,
    reroll_glut_redeemed: bool,
    crystal_ball_redeemed: bool,
    omen_globe_redeemed: bool,
    telescope_redeemed: bool,
    observatory_redeemed: bool,
    grabber_redeemed: bool,
    nacho_tong_redeemed: bool,
    wasteful_redeemed: bool,
    recyclomancy_redeemed: bool,
    tarot_merchant_redeemed: bool,
    tarot_tycoon_redeemed: bool,
    planet_merchant_redeemed: bool,
    planet_tycoon_redeemed: bool,
    seed_money_redeemed: bool,
    money_tree_redeemed: bool,
    blank_redeemed: bool,
    antimatter_redeemed: bool,
    magic_trick_redeemed: bool,
    illusion_redeemed: bool,
    hieroglyph_redeemed: bool,
    petroglyph_redeemed: bool,
    directors_cut_redeemed: bool,
    retcon_redeemed: bool,
    paint_brush_redeemed: bool,
    palette_redeemed: bool,
}
