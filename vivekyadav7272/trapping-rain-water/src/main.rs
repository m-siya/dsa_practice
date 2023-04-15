struct Solution;

impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        if height.len() <= 2 {
            return 0;
        }

        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut trapped_water = 0i32;

        while left < right {
            let left_elem = height[left];
            let right_elem = height[right];

            let to_fill = if left_elem < right_elem {
                left += 1; // Set the future value of left
                left // It so happens that the future value of left is also the current value we need to operate on
                     // (no specific connection as such)
            } else {
                right -= 1; // Set the future value of left
                right // It so happens that the future value of left is also the current value we need to operate on
                      // (no specific connection as such)
            };

            let to_fill_elem = &mut height[to_fill];
            let fill_until = left_elem.min(right_elem) - *to_fill_elem;

            if fill_until > 0 {
                *to_fill_elem += fill_until;
                trapped_water += fill_until;
            }
        }

        trapped_water
    }
}

fn main() {
    println!("Hello, world!");
}
