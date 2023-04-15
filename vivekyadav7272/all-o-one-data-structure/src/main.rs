// #LCSTART https://leetcode.com/problems/all-oone-data-structure/description/
use std::collections::{HashMap, HashSet};

struct AllOne {
    string_counts: HashMap<String, isize>,
    max_key: isize,
    min_key: isize,
    count_group: HashMap<isize, CountGroup>,
}

enum Operation {
    INC,
    DEC,
}

struct CountGroup {
    pub elems: HashSet<String>,
    pub prev: isize,
    pub next: isize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            min_key: 0,
            max_key: 0,
            string_counts: HashMap::new(),
            count_group: HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        println!("inc({key})");
        let count = self.string_counts.entry(key.clone()).or_insert(0);

        let mut prev_grp = self.count_group.get_mut(&*count);
        // Remove from previous group and add into count+1 group.
        // (has no group if added for the first time).
        prev_grp.as_mut().map(|grp| grp.elems.remove(&key));

        *count += 1;

        if *count > self.max_key {
            self.max_key = *count;
        }

        let prev_grp_val = *count - 1;
        let next_grp_val = prev_grp.as_ref().map(|grp| grp.next);

        prev_grp.map(|grp| grp.next = *count);

        let next_grp_val = if let Some(next_grp_val) = next_grp_val {
            next_grp_val
        } else {
            if self.min_key != 1 {
                self.min_key
            } else {
                self.count_group
                    .get(&1)
                    .expect("1 doesn't exist in the count_group, but it should")
                    .next
            }
        };

        if *count == 1 {
            self.min_key = 1;
        }
        // Also wire the prev's _old_ next to point to you.
        // (but only if I'm not going into this grp)
        if let Some(next_grp) = self.count_group.get_mut(&next_grp_val) {
            if next_grp_val != *count {
                next_grp.prev = *count;
            }
        }

        self.count_group
            .entry(*count)
            .or_insert_with(|| CountGroup {
                elems: HashSet::new(),
                prev: prev_grp_val,
                next: next_grp_val,
            })
            .elems
            .insert(key);

        self.pop_group_if_empty(prev_grp_val, Operation::INC);
    }

    fn dec(&mut self, key: String) {
        println!("dec({key})");
        let count = self
            .string_counts
            .get_mut(&key)
            .expect("dec() shouldn't be called before inc()");

        let old_prev_grp_val = *count;
        let old_prev_grp = self.count_group.get_mut(&old_prev_grp_val).expect(
            "Bug in code, string exists with a count but the count doesn't exist in count_group.",
        );

        // Remove from previous group.
        old_prev_grp.elems.remove(&key);

        // Whatever the min_key was is about to be nuked, better handle it rn.
        if *count == self.min_key && old_prev_grp.elems.len() == 0 {
            if *count - 1 != 0 {
                // the string that just got decremented will save us.
                self.min_key = *count - 1;
            } else {
                // the string that got decremented is also gonna die.
                // set it to the next block.
                self.min_key = old_prev_grp.next;
            }
        }
        *count -= 1;

        if *count < self.min_key && *count != 0 {
            self.min_key = *count;
        }
        if *count != 0 {
            let new_prev_grp_val = old_prev_grp.prev;
            let new_next_grp_val = *count + 1; // i.e old_prev_grp
            old_prev_grp.prev = *count;

            // Wire the old's prev to point to you.
            if let Some(new_prev_grp) = self.count_group.get_mut(&new_prev_grp_val) {
                new_prev_grp.next = *count;
            }

            self.count_group
                .entry(*count)
                .or_insert_with(|| CountGroup {
                    elems: HashSet::new(),
                    prev: new_prev_grp_val,
                    next: new_next_grp_val,
                })
                .elems
                .insert(key);
        } else {
            // If count becomes zero, pop it from string_counts.
            self.string_counts.remove(&key);
        }
        self.pop_group_if_empty(old_prev_grp_val, Operation::DEC);
    }

    fn get_max_key(&self) -> String {
        self.count_group
            .get(&self.max_key)
            .and_then(|x| x.elems.iter().next().cloned())
            .unwrap_or(String::new())
    }

    fn get_min_key(&self) -> String {
        self.count_group
            .get(&self.min_key)
            .and_then(|x| x.elems.iter().next().cloned())
            .unwrap_or(String::new())
    }

    fn pop_group_if_empty(&mut self, grp: isize, op: Operation) {
        if grp == 0 {
            return;
        }
        let cnt_grp = self
            .count_group
            .get(&grp)
            .expect("Key should've been there!! I'm the one who pops them :(");

        if cnt_grp.elems.len() != 0 {
            // Yeah this one is all good to go!
            return;
        }
        // This one needs some fixin'
        let prev_grp_val = cnt_grp.prev;
        let next_grp_val = cnt_grp.next;

        self.count_group.remove(&grp);
        if let Some(prev_grp) = self.count_group.get_mut(&prev_grp_val) {
            prev_grp.next = next_grp_val;
        }
        if let Some(next_grp) = self.count_group.get_mut(&next_grp_val) {
            next_grp.prev = prev_grp_val;
        }

        match op {
            Operation::INC => {
                if prev_grp_val == 0 {
                    // This means this was the minimum block
                    self.min_key = next_grp_val;
                }
            }
            Operation::DEC => {
                if next_grp_val == 0 {
                    // This was the max block
                    self.max_key = prev_grp_val;
                }
            }
        }
    }
}
// #LCEND
/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

fn main() {
    println!("Hello, world!");
}
