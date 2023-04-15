// #LCSTART https://leetcode.com/problems/design-add-and-search-words-data-structure/
fn get(c: u8) -> usize {
    (c - b'a') as usize
}
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_string: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            is_string: false,
            children: Default::default(),
        }
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn add_word(&mut self, word: String) {
        self._insert_iterative(word.as_bytes())
    }

    fn _insert_iterative(&mut self, word: &[u8]) {
        let mut curr_node = self;
        for &c in word {
            // SAFETY: Question invariant that all characters are b/w 'a'..='z', which means c - 'a' will always be b/w
            // [0, 25], which means no out-of-bounds error.
            let c_wala_child_node = unsafe { curr_node.children.get_unchecked_mut(get(c)) };
            curr_node = c_wala_child_node.get_or_insert_with(|| Box::new(WordDictionary::new()));
        }
        curr_node.is_string = true;
    }

    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        self._search(word)
    }

    fn _search(&self, word: &[u8]) -> bool {
        if let Some((&c, rest)) = word.split_first() {
            if c == b'.' {
                self.children
                    .iter()
                    .filter_map(|child| Some(child.as_ref()?._search(rest)))
                    .any(|did_find| did_find)
            } else if let Some(c_wala_node) =
                // SAFETY: Question invariant that all characters are b/w 'a'..='z', which means c - 'a' will always be b/w
                // [0, 25], which means no out-of-bounds error.
                unsafe { self.children.get_unchecked(get(c)) }.as_deref()
            {
                c_wala_node._search(rest)
            } else {
                false
            }
        } else {
            self.is_string
        }
    }
}
// #LCEND
