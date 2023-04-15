/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut biggest_area = 0;

        while i < j {
            // assert!(j < height.len());
            while i < j && height[i] <= height[j] {
                biggest_area = biggest_area.max((j - i) as i32 * height[i]);
                i += 1;
            }

            while i < j && height[j] < height[i] {
                biggest_area = biggest_area.max((j - i) as i32 * height[j]);
                j -= 1;
            }
        }
        biggest_area
    }
}
// @lc code=end
