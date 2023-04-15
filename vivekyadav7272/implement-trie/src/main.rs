fn get(c: u8) -> usize {
    (c - b'a') as usize
}
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_string: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            is_string: false,
            children: Default::default(),
        }
    }

    fn new_string(is_string: bool) -> Self {
        Self {
            is_string,
            children: Default::default(),
        }
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn insert(&mut self, word: String) {
        self._insert_iterative(word.as_bytes())
    }

    fn _insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.is_string = true;
            return;
        }
        let next_node = &mut self.children[get(word[0])];
        if let Some(node) = next_node {
            node._insert(&word[1..]);
        } else {
            let mut node = Box::new(Trie::new());
            node._insert(&word[1..]);
            *next_node = Some(node);
        }
    }

    fn _insert_iterative(&mut self, word: &[u8]) {
        let mut curr_node = self;
        for &c in word {
            let c_wala_child_node = &mut curr_node.children[get(c)];
            curr_node = c_wala_child_node.get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr_node.is_string = true;
    }

    fn search(&self, word: String) -> bool {
        let node = self.get_prefix(word.as_bytes());
        if let Some(node) = node {
            node.is_string
        } else {
            false
        }
    }

    fn get_prefix(&self, word: &[u8]) -> Option<&Trie> {
        // if word.is_empty() {
        //     return Some(self);
        // }
        // let nodes = &self.children;
        // if let Some(node) = &nodes[get(word[0])] {
        //     return node.get_prefix(&word[1..]);
        // }
        // None

        let mut curr_node = self;

        for &c in word {
            let c_wala_node = curr_node.children[get(c)].as_deref();
            if let Some(c_wala_node) = c_wala_node {
                curr_node = c_wala_node;
            } else {
                return None;
            }
        }

        Some(curr_node)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get_prefix(prefix.as_bytes()).is_some()
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    println!("Hello, world!");
}
