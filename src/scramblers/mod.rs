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

pub trait Cube {
    fn long_name() -> String;
    fn short_name() -> String;
    fn average_number() -> u32;
    fn scramble(scramble_number: u32) -> Vec<String>;
}

pub struct Clock;
impl Cube for Clock {
    fn long_name() -> String {
        "Clock".to_string()
    }

    fn short_name() -> String {
        "Clock".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        clock::scramble(scramble_number)
    }
}

pub struct FewestMoves;
impl Cube for FewestMoves {
    fn long_name() -> String {
        "3x3x3 Fewest Moves".to_string()
    }

    fn short_name() -> String {
        "FMC".to_string()
    }

    fn average_number() -> u32 {
        3
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        fewest_moves::scramble(scramble_number)
    }
}

pub struct Five;
impl Cube for Five {
    fn long_name() -> String {
        "5x5x5".to_string()
    }

    fn short_name() -> String {
        "5x5".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        five::scramble(scramble_number)
    }
}

pub struct FiveBld;
impl Cube for FiveBld {
    fn long_name() -> String {
        "5x5x5 Blindfolded".to_string()
    }

    fn short_name() -> String {
        "5BLD".to_string()
    }

    fn average_number() -> u32 {
        3
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        five_bld::scramble(scramble_number)
    }
}

pub struct Four;
impl Cube for Four {
    fn long_name() -> String {
        "4x4x4".to_string()
    }

    fn short_name() -> String {
        "4x4".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        four::scramble(scramble_number)
    }
}

pub struct FourBld;
impl Cube for FourBld {
    fn long_name() -> String {
        "Four_bld".to_string()
    }

    fn short_name() -> String {
        "Four_bld".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        four_bld::scramble(scramble_number)
    }
}

pub struct Megaminx;
impl Cube for Megaminx {
    fn long_name() -> String {
        "Megaminx".to_string()
    }

    fn short_name() -> String {
        "Mega".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        megaminx::scramble(scramble_number)
    }
}

pub struct Pyraminx;
impl Cube for Pyraminx {
    fn long_name() -> String {
        "Pyraminx".to_string()
    }

    fn short_name() -> String {
        "Pyra".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        pyraminx::scramble(scramble_number)
    }
}

pub struct Seven;
impl Cube for Seven {
    fn long_name() -> String {
        "7x7x7".to_string()
    }

    fn short_name() -> String {
        "7x7".to_string()
    }

    fn average_number() -> u32 {
        3
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        seven::scramble(scramble_number)
    }
}

pub struct Six;
impl Cube for Six {
    fn long_name() -> String {
        "6x6x6".to_string()
    }

    fn short_name() -> String {
        "6x6".to_string()
    }

    fn average_number() -> u32 {
        3
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        six::scramble(scramble_number)
    }
}

pub struct Skewb;
impl Cube for Skewb {
    fn long_name() -> String {
        "Skewb".to_string()
    }

    fn short_name() -> String {
        "Skewb".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        skewb::scramble(scramble_number)
    }
}

pub struct SquareOne;
impl Cube for SquareOne {
    fn long_name() -> String {
        "Square-1".to_string()
    }

    fn short_name() -> String {
        "Squan".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        square_one::scramble(scramble_number)
    }
}

pub struct Three;
impl Cube for Three {
    fn long_name() -> String {
        "3x3x3".to_string()
    }

    fn short_name() -> String {
        "3x3".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        three::scramble(scramble_number)
    }
}

pub struct ThreeBld;
impl Cube for ThreeBld {
    fn long_name() -> String {
        "3x3x3 Blindfolded".to_string()
    }

    fn short_name() -> String {
        "3BlD".to_string()
    }

    fn average_number() -> u32 {
        3
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        three_bld::scramble(scramble_number)
    }
}

pub struct Two;
impl Cube for Two {
    fn long_name() -> String {
        "2x2x2".to_string()
    }

    fn short_name() -> String {
        "2x2".to_string()
    }

    fn average_number() -> u32 {
        5
    }

    fn scramble(scramble_number: u32) -> Vec<String> {
        two::scramble(scramble_number)
    }
}
