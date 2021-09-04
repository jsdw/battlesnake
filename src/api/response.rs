//! Types adapted from https://github.com/McRaeAlex/RustySnake

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Info {
    pub apiversion: String,
    #[serde(skip_serializing_if = "Option::is_none")]    
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Start {
    pub color: String,
    #[serde(rename = "headType")]
    pub head_type: HeadType,
    #[serde(rename = "tailType")]
    pub tail_type: TailType,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HeadType {
    // Standard
    Regular,
    Beluga,
    Bendr,
    Dead,
    Evil,
    Fang,
    Pixel,
    Safe,
    #[serde(rename = "sand-worm")]
    SandWorm,
    Shades,
    Silly,
    Smile,
    Tongue,

    // Branded Customisations
    #[serde(rename = "rbc-bowler")]
    RbcBowler,
    #[serde(rename = "replit-mark")]
    ReplitMark,

    // Community
    #[serde(rename = "all-seeing")]
    AllSeeing,
    #[serde(rename = "smart-caterpillar")]
    SmartCaterpillar,

    // Battlesnake Winter Classic 2019
    Bonhomme,
    Earmuffs,
    Rudolph,
    Scarf,
    Ski,
    Snowman,
    #[serde(rename = "snow-worm")]
    SnowWorm,

    // Stay home and code 2020
    Caffeine,
    Gamer,
    #[serde(rename = "tiger-king")]
    TigerKing,
    Workout
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TailType {
    // Standard
    Regular,
    #[serde(rename = "block-bum")]
    BlockBum,
    Bolt,
    Curled,
    #[serde(rename = "fat-rattle")]
    FatRattle,
    Freckled,
    Hook,
    Pixel,
    #[serde(rename = "round-bum")]
    RoundBum,
    Sharp,
    Skinny,
    #[serde(rename = "small-rattle")]
    SmallRattle,

    // Branded Customisations
    #[serde(rename = "rbc-necktie")]
    RbcNecktie,
    #[serde(rename = "replit-notmark")]
    ReplitNotmark,

    // Community
    #[serde(rename = "mystic-moon")]
    MysticMoon,

    // Battlesnake winter classic 2019
    Bonhomme,
    Flake,
    IceSkate,
    Present,

    // Stay home and code 2020
    Coffee,
    Mouse,
    #[serde(rename = "tiger-tail")]
    TigerTail,
    Weight
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Move {
    #[serde(rename = "move")]
    pub movement: Movement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shout: Option<String>,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            movement: Movement::Up,
            shout: None
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}