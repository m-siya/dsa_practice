struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        let mut boats_required = 0;
        people.sort_unstable();
        let mut smol_ppl = 0;
        let mut large_ppl = people.len() - 1;
        // [1, 2, 2, 3]
        while smol_ppl < large_ppl {
            let latest_large = people[large_ppl];
            let smol_person = people[smol_ppl];
            if latest_large + smol_person <= limit {
                smol_ppl += 1;
            }
            large_ppl -= 1;
            boats_required += 1;
        }

        if smol_ppl == large_ppl {
            boats_required += 1;
        }

        boats_required
    }
}
fn main() {
    println!("Hello, world!");
}
