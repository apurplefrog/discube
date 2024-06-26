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

pub struct Cube<'a> {
    long_name: &'a str,
    short_name: &'a str,
    average_number: u32,
    scramble: fn(u32) -> Vec<String>,
}

pub const CLOCK: Cube = Cube {
    long_name: "Clock",
    short_name: "Clock",
    average_number: 5,
    scramble: clock::scramble,
};

pub const FEWEST_MOVES: Cube = Cube {
    long_name: "3x3x3 Fewest Moves",
    short_name: "FM",
    average_number: 3,
    scramble: fewest_moves::scramble,
};

pub const FIVE: Cube = Cube {
    long_name: "5x5x5",
    short_name: "5x5",
    average_number: 5,
    scramble: five::scramble,
};

pub const FIVE_BLD: Cube = Cube {
    long_name: "5x5x5 Blindfolded",
    short_name: "5BLD",
    average_number: 3,
    scramble: five_bld::scramble,
};

pub const FOUR: Cube = Cube {
    long_name: "4x4x4",
    short_name: "4x4",
    average_number: 5,
    scramble: four::scramble,
};

pub const FOUR_BLD: Cube = Cube {
    long_name: "4x4x4 Blindfolded",
    short_name: "4BLD",
    average_number: 5,
    scramble: four_bld::scramble,
};

pub const MEGAMINX: Cube = Cube {
    long_name: "Megaminx",
    short_name: "Mega",
    average_number: 5,
    scramble: megaminx::scramble,
};

pub const PYRAMINX: Cube = Cube {
    long_name: "Pyraminx",
    short_name: "pyra",
    average_number: 5,
    scramble: pyraminx::scramble,
};

pub const SEVEN: Cube = Cube {
    long_name: "7x7x7",
    short_name: "7x7",
    average_number: 5,
    scramble: seven::scramble,
};

pub const SIX: Cube = Cube {
    long_name: "6x6x6",
    short_name: "6x6",
    average_number: 5,
    scramble: six::scramble,
};

pub const SKEWB: Cube = Cube {
    long_name: "Skewb",
    short_name: "Skewb",
    average_number: 5,
    scramble: skewb::scramble,
};

pub const SQUARE_ONE: Cube = Cube {
    long_name: "Square-1",
    short_name: "Squan",
    average_number: 5,
    scramble: square_one::scramble,
};

pub const THREE: Cube = Cube {
    long_name: "three",
    short_name: "three",
    average_number: 5,
    scramble: three::scramble,
};

pub const THREE_BLD: Cube = Cube {
    long_name: "3x3x3 Blindfolded",
    short_name: "3BLD",
    average_number: 5,
    scramble: three_bld::scramble,
};

pub const TWO: Cube = Cube {
    long_name: "2x2x2",
    short_name: "2x2",
    average_number: 5,
    scramble: two::scramble,
};
