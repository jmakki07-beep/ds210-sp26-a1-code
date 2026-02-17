use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
       let mut high = max;
       let mut low = min;
     while low < high {
      let guess: u32 = (low + high) / 2;
      let compare: i32 = player.ask_to_compare(guess);

     match compare {
        1  => low = guess + 1,
        -1 => high = guess,
        0  => return guess,
        _  => panic!("Invalid response from player"),
    }
}

  low

    }
}
