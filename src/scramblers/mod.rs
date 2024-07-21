pub mod clock;
pub mod fewest_moves;
pub mod five;
pub mod five_bld;
pub mod four;
pub mod four_bld;
pub mod megaminx;
pub mod pyraminx;
pub mod seven;
pub mod six;
pub mod skewb;
pub mod square_one;
pub mod three;
pub mod three_bld;
pub mod two;

use rand::{self, rngs::ThreadRng, Rng};

pub enum Cube {
    Clock,
    FewestMoves,
    Five,
    FiveBld,
    Four,
    FourBld,
    Megaminx,
    Pyraminx,
    Seven,
    Six,
    Skewb,
    SquareOne,
    Three,
    ThreeBld,
    Two,
}

impl Cube {
    pub fn short_name(self) -> String {
        match self {
            Cube::Clock => "clock",
            Cube::FewestMoves => "FM",
            Cube::Five => "5x5",
            Cube::FiveBld => "5BLD",
            Cube::Four => "4x4",
            Cube::FourBld => "4BLD",
            Cube::Megaminx => "mega",
            Cube::Pyraminx => "pyra",
            Cube::Seven => "7x7",
            Cube::Six => "6x6",
            Cube::Skewb => "skewb",
            Cube::SquareOne => "squan",
            Cube::Three => "3x3",
            Cube::ThreeBld => "3BLD",
            Cube::Two => "2x2",
        }
        .to_string()
    }

    pub fn long_name(self) -> String {
        match self {
            Cube::Clock => "clock",
            Cube::FewestMoves => "3x3x3 fewest moves",
            Cube::Five => "5x5x5",
            Cube::FiveBld => "5x5x5 blindfolded",
            Cube::Four => "4x4x4",
            Cube::FourBld => "4x4x4 blindfolded",
            Cube::Megaminx => "megaminx",
            Cube::Pyraminx => "pyraminx",
            Cube::Seven => "7x7x7",
            Cube::Six => "6x6x6",
            Cube::Skewb => "skewb",
            Cube::SquareOne => "squan",
            Cube::Three => "3x3x3",
            Cube::ThreeBld => "3x3x3 blindfolded",
            Cube::Two => "2x2x2",
        }
        .to_string()
    }

    pub fn scramble(self, scramble_number: u32) -> Vec<String> {
        match self {
            Cube::Clock => clock::scramble(scramble_number),
            Cube::FewestMoves => fewest_moves::scramble(scramble_number),
            Cube::Five => five::scramble(scramble_number),
            Cube::FiveBld => five_bld::scramble(scramble_number),
            Cube::Four => four::scramble(scramble_number),
            Cube::FourBld => four_bld::scramble(scramble_number),
            Cube::Megaminx => megaminx::scramble(scramble_number),
            Cube::Pyraminx => pyraminx::scramble(scramble_number),
            Cube::Seven => seven::scramble(scramble_number),
            Cube::Six => six::scramble(scramble_number),
            Cube::Skewb => skewb::scramble(scramble_number),
            Cube::SquareOne => square_one::scramble(scramble_number),
            Cube::Three => three::scramble(scramble_number),
            Cube::ThreeBld => three_bld::scramble(scramble_number),
            Cube::Two => two::scramble(scramble_number),
        }
    }

    pub fn average_scramble_count(self) -> u32 {
        match self {
            Cube::Clock => 5,
            Cube::FewestMoves => 3,
            Cube::Five => 5,
            Cube::FiveBld => 3,
            Cube::Four => 5,
            Cube::FourBld => 3,
            Cube::Megaminx => 5,
            Cube::Pyraminx => 5,
            Cube::Seven => 3,
            Cube::Six => 3,
            Cube::Skewb => 5,
            Cube::SquareOne => 5,
            Cube::Three => 5,
            Cube::ThreeBld => 3,
            Cube::Two => 5,
        }
    }

    pub fn random_cube() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..15) {
            0 => Self::Clock,
            1 => Self::FewestMoves,
            2 => Self::Five,
            3 => Self::FiveBld,
            4 => Self::Four,
            5 => Self::FourBld,
            6 => Self::Megaminx,
            7 => Self::Pyraminx,
            8 => Self::Seven,
            9 => Self::Six,
            10 => Self::Skewb,
            11 => Self::SquareOne,
            12 => Self::Three,
            _ => Self::ThreeBld,
        }
    }
}
