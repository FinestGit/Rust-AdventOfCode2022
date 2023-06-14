use std::collections;
use std::fs;

fn main() {
    // Read in file for consumption
    let content = fs::read_to_string("./input.txt").expect("Failed to read file.");

    let play_points: collections::HashMap<&str, i32> = get_play_ponts();

    let winning_points: collections::HashMap<&str, collections::HashMap<&str, i32>> =
        get_winning_points();

    let mut accumulator: i32 = 0;

    // Grab each line from the content
    for line in content.lines() {
        // Split each line into it's parts so as to be able to seperate them
        let split_string: Vec<&str> = line.split(" ").collect();
        let opponent_move = split_string[0];
        let your_move = split_string[1];

        // Grab the points that you would be awarded based on the move you played
        let points_for_your_move = play_points.get(your_move).expect("Could not find key");

        // Grab the points that you would be awarded based on the outcome of the round
        let points_for_win_loss_draw = winning_points
            .get(opponent_move)
            .expect("Could not find key")
            .get(your_move)
            .expect("Could not find key");

        accumulator += points_for_your_move + points_for_win_loss_draw;
    }

    println!("{accumulator}");

    get_winning_moves(content);
}

fn get_play_ponts() -> collections::HashMap<&'static str, i32> {
    // We need a map that says { X: 1, Y: 2, Z: 3 }
    let mut play_points = collections::HashMap::new();
    play_points.insert("X", 1);
    play_points.insert("Y", 2);
    play_points.insert("Z", 3);

    play_points
}

fn get_winning_points(
) -> collections::HashMap<&'static str, collections::HashMap<&'static str, i32>> {
    let mut rock_winning_moves = collections::HashMap::new();
    let mut paper_winning_moves = collections::HashMap::new();
    let mut scissor_winning_moves = collections::HashMap::new();
    let mut winning_moves = collections::HashMap::new();

    rock_winning_moves.insert("X", 3);
    rock_winning_moves.insert("Y", 6);
    rock_winning_moves.insert("Z", 0);

    paper_winning_moves.insert("X", 0);
    paper_winning_moves.insert("Y", 3);
    paper_winning_moves.insert("Z", 6);

    scissor_winning_moves.insert("X", 6);
    scissor_winning_moves.insert("Y", 0);
    scissor_winning_moves.insert("Z", 3);

    winning_moves.insert("A", rock_winning_moves);
    winning_moves.insert("B", paper_winning_moves);
    winning_moves.insert("C", scissor_winning_moves);

    winning_moves
}

fn get_winning_moves(content: String) {
    let points_by_move: collections::HashMap<&str, i32> =
        collections::HashMap::from([("Rock", 1), ("Paper", 2), ("Scissor", 3)]);
    let mapped_values: collections::HashMap<&str, &str> =
        collections::HashMap::from([("A", "Rock"), ("B", "Paper"), ("C", "Scissor")]);
    let what_to_do: collections::HashMap<&str, &str> =
        collections::HashMap::from([("X", "Loss"), ("Y", "Draw"), ("Z", "Win")]);
    let points_by_win_loss_draw: collections::HashMap<&str, i32> =
        collections::HashMap::from([("Win", 6), ("Draw", 3), ("Loss", 0)]);
    let win_loss_draw_moves: collections::HashMap<&str, collections::HashMap<&str, &str>> =
        collections::HashMap::from([
            (
                "Rock",
                collections::HashMap::from([
                    ("Win", "Paper"),
                    ("Draw", "Rock"),
                    ("Loss", "Scissor"),
                ]),
            ),
            (
                "Paper",
                collections::HashMap::from([
                    ("Win", "Scissor"),
                    ("Draw", "Paper"),
                    ("Loss", "Rock"),
                ]),
            ),
            (
                "Scissor",
                collections::HashMap::from([
                    ("Win", "Rock"),
                    ("Draw", "Scissor"),
                    ("Loss", "Paper"),
                ]),
            ),
        ]);

    let mut accumulated_points = 0;

    for line in content.lines() {
        let split_string: Vec<&str> = line.split(" ").collect();
        let opponent_move: &str = split_string[0];
        let what_should_i_do: &str = split_string[1];

        let opponent_move_actual = mapped_values
            .get(opponent_move)
            .expect("Could not find key");
        let win_loss_or_draw = what_to_do
            .get(what_should_i_do)
            .expect("Could not find key");

        let what_do_i_play = win_loss_draw_moves
            .get(opponent_move_actual)
            .expect("Could not find key")
            .get(win_loss_or_draw)
            .expect("Could not find key");
        let points_earned_for_move = points_by_move
            .get(what_do_i_play)
            .expect("Could not find key");
        let points_earned_for_win_loss_draw = points_by_win_loss_draw
            .get(win_loss_or_draw)
            .expect("Could not find key");
        let total_points_for_round = points_earned_for_move + points_earned_for_win_loss_draw;
        accumulated_points += total_points_for_round;
    }

    println!("Total points earned: {accumulated_points}");
}
