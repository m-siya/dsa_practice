struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut spells = spells;
        let mut potions = potions;

        // attempt 1: Just sort only potions and do spells linearly
        potions.sort_unstable();

        spells.iter_mut().for_each(|x| {
            let min_pairable = f64::ceil(success as f64 / *x as f64) as i32;
            let pairables = potions.partition_point(|&potion| potion < min_pairable);
            *x = (potions.len() - pairables) as i32;
        });

        spells
    }
}
fn main() {
    println!("Hello, world!");
}
