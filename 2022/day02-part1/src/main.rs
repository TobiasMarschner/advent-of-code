// Use a custom type to identify the different shapes that can be used in the game.
#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

// Allows us to use the shapes without the Shape:: prefix.
use Shape::*;

fn main() {
    // Use command line arguments to specify the input filename.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: ./main <input-file>\nNo input file provided. Exiting.");
    }

    // Next, read the contents of the input file into a string for easier processing.
    let input = std::fs::read_to_string(&args[1]).expect("Error opening file");
    // Line-by-line processing is easiest.
    let input = input.lines();

    // --- TASK BEGIN ---
    let mut total_score = 0;

    for line in input {
        // Translate the line's first character into its respective shape.
        let opponent_shape = match line.chars().next() {
            Some('A') => Rock,
            Some('B') => Paper,
            Some('C') => Scissors,
            _ => { panic!("Unexpected left character."); }
        };

        // Translate the line's second character into its respective shape.
        let player_shape = match line.chars().nth(2) {
            Some('X') => Rock,
            Some('Y') => Paper,
            Some('Z') => Scissors,
            _ => { panic!("Unexpected right character."); }
        };

        // Add the score for the matchup (win/loss/draw) to the total score.
        total_score += match (player_shape, opponent_shape) {
            (Rock    , Rock    ) => 3,
            (Rock    , Paper   ) => 0,
            (Rock    , Scissors) => 6,
            (Paper   , Rock    ) => 6,
            (Paper   , Paper   ) => 3,
            (Paper   , Scissors) => 0,
            (Scissors, Rock    ) => 0,
            (Scissors, Paper   ) => 6,
            (Scissors, Scissors) => 3,
        };

        // Add the score of the player's shape to the total score.
        total_score += match player_shape {
            Rock     => 1,
            Paper    => 2,
            Scissors => 3,
        };
    }

    println!("Total score: {}", total_score);
}

