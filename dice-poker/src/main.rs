use rand::Rng;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Dice {
    hand: [u8; 5],      // store player/dealer hand
    die_count: [u8; 7], // count how many each die was rolled
}

impl Dice {
    fn new() -> Dice {
        let hand = [0, 0, 0, 0, 0];
        let die_count = [0, 0, 0, 0, 0, 0, 0];

        Dice { hand, die_count }
    }

    fn roll_dice() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..6)
    }

    fn deal_hand(&mut self, max_dice: u8) {
        for n in 0..max_dice as usize {
            self.hand[n] = Dice::roll_dice();
        }
    }

    fn rank_hand(&mut self) -> u8 {
        // count numbers of die was rolled (except index 0)
        for face in self.hand {
            self.die_count[face as usize] += 1;
        }

        // There is five value in die_count (all five dice have the same face value)
        if self.die_count.contains(&5) {
            6
        } else if self.die_count.contains(&4) {
            5
        } else if self.die_count.contains(&3) && self.die_count.contains(&2) {
            4
        } else if self.die_count.contains(&3) {
            3
        } else if self.die_count.iter().filter(|&n| *n == 2).count() == 2 {
            2
        } else if self.die_count.contains(&2) {
            1
        } else {
            0
        }
    }
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
        let mut player_dice = Dice::new();
        player_dice.deal_hand(5);
        println!("hand: {:?}", player_dice.hand);
        let rank = player_dice.rank_hand();
        println!("die count: {:?}", player_dice.die_count);
        println!("rank: {}", rank);
    } else {
        println!("No worries... Next time!")
    }
}
