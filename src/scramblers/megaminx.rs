use rand::{self, Rng};

pub fn scramble() -> String {
    let mut rng = rand::thread_rng();
    let mut moves = String::new();
    for i in 1..=70 {
        let current_move = rng.gen_bool(0.5);
        if i % 2 == 1 {
            if current_move {
                moves += "R++ ";
            } else {
                moves += "R-- ";
            };
        } else {
            if current_move {
                moves += "D++ ";
            } else {
                moves += "D-- ";
            };
        };

        if i % 10 == 0 {
            if current_move {
                moves += "U\n";
            } else {
                moves += "U'\n";
            };
        };
    }
    moves.to_string()
}
