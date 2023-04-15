struct Solution;

impl Solution {
    pub fn validate_stack_sequences(mut pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        pushed.push(i32::MAX);
        let mut st: Vec<i32> = Vec::with_capacity(pushed.len());
        let mut to_push = 0;
        let mut to_pop = 0;

        while to_push < pushed.len() {
            if let Some(&last) = st.last() {
                if last == popped[to_pop] {
                    to_pop += 1;
                    st.pop();
                    continue;
                }
            }
            st.push(pushed[to_push]);
            to_push += 1;
        }

        to_pop == popped.len()
    }
}
fn main() {
    println!("Hello, world!");
}
