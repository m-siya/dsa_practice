struct BrowserHistory {
    history: Vec<String>,
    curr_idx: isize,
    last_idx: isize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            curr_idx: 0,
            last_idx: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.curr_idx += 1;
        self.last_idx = self.curr_idx;
        if let Some(zombie_link) = self.history.get_mut(self.curr_idx as usize) {
            *zombie_link = url;
        } else {
            self.history.push(url);
        }
    }

    fn back(&mut self, steps: i32) -> String {
        self.curr_idx = (self.curr_idx - steps as isize).clamp(0, self.last_idx);
        self.history[self.curr_idx as usize].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.curr_idx = (self.curr_idx + steps as isize).clamp(0, self.last_idx);
        self.history[self.curr_idx as usize].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */

fn main() {
    println!("Hello, world!");
}
