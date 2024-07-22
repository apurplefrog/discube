use rand::{self, Rng};

pub fn scramble(scramble_number: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut moves: Vec<String> = Vec::new();
    for _ in 0..scramble_number {
        let mut current_scramble = String::new();
        for i in 1..=70 {
            let current_move = match (i % 2, i % 10, rng.gen_bool(0.5)) {
                (1, _, true) => "R++ ",
                (1, _, false) => "R-- ",
                (0, 0, true) => "D++ U\n  ",
                (0, 0, false) => "D-- U'\n  ",
                (0, _, true) => "D++ ",
                (0, _, false) => "D-- ",
                (_, _, _) => "ERR ",
            };

            current_scramble.push_str(current_move);
        }
        moves.push(current_scramble[0..current_scramble.len() - 1].to_string());
    }

    moves
}
