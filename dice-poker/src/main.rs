use rand::Rng;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Dice {
    hand: [u8; 5],      // store player/dealer hand
    die_count: [u8; 7], // count how many each die was rolled
    rank: usize,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            hand: [0; 5],
            die_count: [0; 7],
            rank: 0,
        }
    }

    fn deal_hand(&mut self, max_dice: u8) {
        for n in 0..max_dice as usize {
            self.hand[n] = roll_dice();
        }
    }

    fn rank_hand(&mut self) {
        // count numbers of die was rolled (except index 0)
        for face in self.hand {
            self.die_count[face as usize] += 1;
        }

        // There is five value in die_count (all five dice have the same face value)
        if self.die_count.contains(&5) {
            self.rank = 6;
        } else if self.die_count.contains(&4) {
            self.rank = 5;
        } else if self.die_count.contains(&3) && self.die_count.contains(&2) {
            self.rank = 4;
        } else if self.die_count.contains(&3) {
            self.rank = 3;
        } else if self.die_count.iter().filter(|&n| *n == 2).count() == 2 {
            self.rank = 2;
        } else if self.die_count.contains(&2) {
            self.rank = 1;
        } else {
            self.rank = 0;
        }
    }
}

pub struct Player {
    dice: Dice,

    // stores player's hand dealt stats, index 0 is 'Nothing special', 1 is 'One pair',... and 6 is 'Five of a kind'
    player_stats: [u8; 7],

    // Index 0 is the amount won game, 1 is lost game, 2 is drawn game of player
    player_game_results: [u8; 3],

    // Numbers of games the player has played
    games: u8,
}

impl Player {
    fn new() -> Player {
        Player {
            dice: Dice::new(),
            player_stats: [0; 7],
            player_game_results: [0; 3],
            games: 0,
        }
    }

    fn rolling(&mut self) {
        self.dice.deal_hand(5);
        self.dice.rank_hand();

        // update game stats
        self.player_stats[self.dice.rank] += 1;
    }

    // won, lost, drawn games
    fn update_game_results(&mut self, result: usize) {
        self.player_game_results[result] += 1;
    }

    fn print_dice(&self) {
        println!("hand: {:?}", self.dice.hand);
        println!("die count: {:?}", self.dice.die_count);
        println!("rank: {}", self.dice.rank);
    }

    fn print_result(&self) {
        println!("stats: {:?}", self.player_stats);
        println!("results: {:?}", self.player_game_results);
    }
}

// Return: 0 -> first_rank won, 1 -> fist_rank lost, 2 -> drawn
fn compare_rank_hand(first_rank: usize, second_rank: usize) -> usize {
    if first_rank > second_rank {
        0
    } else if first_rank < second_rank {
        1
    } else {
        2
    }
}

fn roll_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..6)
}

fn main() {
    println!("Game Dice Poker!");
    print!("Would you like to play poker [y|n]? ");
    io::stdout().flush().unwrap();
    let mut play = String::new();
    io::stdin().read_line(&mut play).expect("Failed to input");
    print!("Entered: {}", play);

    // Input string contains a trailing newline. Use trim to remove it
    if play.trim() == "y" {
        // player
        let mut player = Player::new();
        player.rolling();
        player.print_dice();

        // dealer
        let mut dealer = Player::new();
        dealer.rolling();
        dealer.print_dice();

        // compare ranks
        let result = compare_rank_hand(player.dice.rank, dealer.dice.rank);
        if result == 0 {
            println!("Won");
        } else if result == 1 {
            println!("Lost");
        } else {
            println!("Drawn");
        }
        player.update_game_results(result);
        player.print_result();
    } else {
        println!("No worries... Next time!")
    }
}
