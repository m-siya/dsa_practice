#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    idx: usize,
    inner: Vec<NestedIterator>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        NestedIterator::new_with_nested_integer(NestedInteger::List(nestedList))
    }

    fn next(&self) -> i32 {}

    fn has_next(&self) -> bool {}

    fn new_with_nested_integer(nested_list: NestedInteger) -> NestedIterator {
        match nested_list {
            NestedInteger::Int(x) => NestedIterator {
                idx: 0,
                inner: nested_list,
            },
        }
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    println!("Hello, world!");
}
