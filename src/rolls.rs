use rand::Rng;

use crate::structures::dice::Dice;

pub fn roll_stat() -> u8 {
    let mut rng = rand::thread_rng();
    
    let rolls = [
        rng.gen_range(1..=6),
        rng.gen_range(1..=6),
        rng.gen_range(1..=6),
        rng.gen_range(1..=6)
    ];

    let mut sum = 0;
    let mut lowest = 10;
    for roll in rolls.iter() {
        sum += roll;
        if roll < &lowest{
            lowest = roll.clone()
        }
    };
    sum-lowest
}

pub fn roll_dice(dice: Dice) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=dice.to_num())
}

pub fn random(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}