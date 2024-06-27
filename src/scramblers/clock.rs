use rand::{self, Rng};

pub fn scramble(scramble_number: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut moves: Vec<String> = Vec::new();
    for _ in 0..scramble_number {
        let mut is_valid_scramble = false;
        let mut numbers: Vec<i32> = vec![0; 14];
        while !is_valid_scramble {
            numbers = numbers.iter().map(|_| rng.gen_range(-5..6)).collect();
            is_valid_scramble = numbers.iter().filter(|&n| *n != 0).count() > 2;
        }

        let numbers_as_strings: Vec<String> = numbers
            .iter()
            .map(|&n| format!("{}{}", n.abs(), if n < 0 { "-" } else { "+" }))
            .collect();

        let current_scramble = format!(
            "UR{} DR{} DL{} UL{} U{} R{} D{} L{} ALL{} y2 U{} R{} D{} L{} ALL{}",
            numbers_as_strings[0],
            numbers_as_strings[1],
            numbers_as_strings[2],
            numbers_as_strings[3],
            numbers_as_strings[4],
            numbers_as_strings[5],
            numbers_as_strings[6],
            numbers_as_strings[7],
            numbers_as_strings[8],
            numbers_as_strings[9],
            numbers_as_strings[10],
            numbers_as_strings[11],
            numbers_as_strings[12],
            numbers_as_strings[13]
        );

        moves.push(current_scramble);
    }

    moves
}
