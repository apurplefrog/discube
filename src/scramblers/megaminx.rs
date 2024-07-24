use rand::{self, Rng};

pub fn scramble(scramble_number: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut scrambles: Vec<String> = Vec::new();
    for _ in 0..scramble_number {
        let mut scramble = String::new();
        for _ in 0..7 {
            let mut line_moves = Vec::new();
            for j in 0..10 {
                let move_letter = if j % 2 == 0 { "R" } else { "D" };
                let move_sign = if rng.gen_bool(0.5) { "++" } else { "--" };
                let full_move = format!("{move_letter}{move_sign}");
                line_moves.push(full_move);
            }
            scramble.push_str(&format!("  {}\n", line_moves.join(" ")));
        }
        scrambles.push(scramble);
    }

    scrambles
}
