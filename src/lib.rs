//! Simple library to be used as a min-priority queue.
//!
//! With it you may `insert` new elements and `extract_min` elements. Each operation is O(log(n))
//! as the heap property is maintained.
//!
//! # Usage
//!
//! This crate is not published on `crates.io` and can be used by adding `min_binary_heap` under the
//! `dependencies` section name in your project's `Cargo.toml` as follows:
//!
//! ```toml
//! [dependencies]
//! min_binary_heap = { git = "https://github.com/konstantindt/min-binary-heap" }
//! ```
//!
//! and the following to your crate root:
//!
//! ```rust
//! extern crate min_binary_heap;
//! ```
//!
//! # Examples
//!
//! The following example shows how this library can be used to determine which `Job` should be
//! attempted next.
//!
//! ```{.rust}
//! extern crate min_binary_heap;
//!
//! use std::cmp::Ordering;
//! use min_binary_heap::MinBinaryHeap;
//!
//! #[derive(Debug, Eq)]
//! struct Job {
//!     id: u64,
//!     time_left: u8,
//! }
//!
//! impl Ord for Job {
//!     fn cmp(&self, other: &Job) -> Ordering {
//!         self.time_left.cmp(&other.time_left)
//!     }
//! }
//!
//! impl PartialOrd for Job {
//!     fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! impl PartialEq for Job {
//!     fn eq(&self, other: &Job) -> bool {
//!         self.time_left == other.time_left
//!     }
//! }
//!
//! fn main() {
//!     let mut queue = MinBinaryHeap::new();
//!
//!     queue.insert(Job { id: 1, time_left: 5 });
//!     queue.insert(Job { id: 2, time_left: 6 });
//!     queue.insert(Job { id: 3, time_left: 4 });
//!
//!     assert_eq!(queue.extract_min(), Some(Job { id: 3, time_left: 4 }));
//!     assert_eq!(queue.extract_min(), Some(Job { id: 1, time_left: 5 }));
//!     assert_eq!(queue.extract_min(), Some(Job { id: 2, time_left: 6 }));
//! }
//! ```

/// A min-priority queue implemented with a binary heap.
///
/// The elements of the queue are of a type which implements the standard library's `Ord` trait
/// i.e. the type forms a total order.
pub struct MinBinaryHeap<T> {
    tree: Vec<T>,
}

impl<T> MinBinaryHeap<T>
    where T: Ord
{
    /// Removes the smallest item from the binary heap and returns it, or `None` if heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use min_binary_heap::MinBinaryHeap;
    /// let mut queue = MinBinaryHeap::new();
    /// queue.insert(10);
    /// queue.insert(1);
    ///
    /// assert_eq!(queue.extract_min(), Some(1));
    /// assert_eq!(queue.extract_min(), Some(10));
    /// assert_eq!(queue.extract_min(), None);
    /// ```
    pub fn extract_min(&mut self) -> Option<T> {
        if self.tree.len() < 1 {
            // Attempted 'extract_min' on empty tree.
            return None;
        } else if self.tree.len() < 2 {
            // There is only one element.
            return Some(self.tree.pop().unwrap());
        }
        // Remove root and replace it with last node given by BFS.
        let last_node_index = self.tree.len() - 1;
        self.tree.swap(0, last_node_index);
        let minimum = self.tree.pop().unwrap();

        if self.tree.len() > 1 {
            let mut current_index = 0;
            let mut child_index_min = match self.child_index_min(current_index) {
                Some(child_index) => child_index,
                None => return Some(minimum),
            };
            // Trickle root down tree.
            while self.tree[current_index] > self.tree[child_index_min] {
                self.tree.swap(current_index, child_index_min);

                current_index = child_index_min;
                child_index_min = match self.child_index_min(current_index) {
                    Some(index) => index,
                    None => break,
                }
            }
        }
        // return root before extract.
        Some(minimum)
    }

    /// Returns the index of the parent node of a given node with index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i < 1` or `i >= size`
    ///
    fn parent_index(&self, of_index: usize) -> usize {
        if of_index < 1 || of_index >= self.size() {
            panic!("MinBinaryHeap index out of bounds.");
        } else if of_index % 2 == 0 {
            (of_index - 2) / 2
        } else {
            (of_index - 1) / 2
        }
    }

    /// Returns the index of the smallest child node of a given parent node with index `i`
    /// or `None` if no children exist.
    fn child_index_min(&self, of_index: usize) -> Option<usize> {
        // Compute potential indexes of children.
        let left_child_index = 2 * of_index + 1;
        let right_child_index = 2 * of_index + 2;

        if left_child_index < self.tree.len() {
            // Left child exists...
            if right_child_index < self.tree.len() {
                // Right child exists... Return index of minimum child node.
                if self.tree[left_child_index] < self.tree[right_child_index] {
                    Some(left_child_index)
                } else {
                    Some(right_child_index)
                }
            } else {
                // Only left child exists.
                Some(left_child_index)
            }
        } else {
            // No children.
            None
        }
    }

    /// Returns the number of elements present inside the binary heap (queue).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use min_binary_heap::MinBinaryHeap;
    /// let mut queue = MinBinaryHeap::new();
    /// queue.insert(10);
    /// queue.insert(1);
    ///
    /// assert_eq!(queue.size(), 2)
    /// ```
    ///
    pub fn size(&self) -> usize {
        self.tree.len()
    }

    /// Insert elements into the binary heap.
    ///
    /// Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use min_binary_heap::MinBinaryHeap;
    /// let mut queue = MinBinaryHeap::new();
    /// queue.insert(10);
    /// queue.insert(1);
    /// queue.insert(5);
    ///
    /// assert_eq!(queue.size(), 3);
    /// assert_eq!(queue.extract_min(), Some(1));
    /// ```
    pub fn insert(&mut self, new_node: T) {
        self.tree.push(new_node);

        if self.tree.len() > 1 {
            let mut current_index = self.tree.len() - 1;
            let mut parent_index = self.parent_index(current_index);
            // Bubble new node up tree.
            while self.tree[current_index] < self.tree[parent_index] {
                self.tree.swap(current_index, parent_index);

                if parent_index < 1 {
                    break;
                }

                current_index = parent_index;
                parent_index = self.parent_index(current_index);
            }
        }
    }

    /// Helper function used to create an empty binary min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use min_binary_heap::MinBinaryHeap;
    /// let queue: MinBinaryHeap<u8> = MinBinaryHeap::new();
    ///
    /// assert_eq!(queue.size(), 0);
    /// ```
    pub fn new() -> MinBinaryHeap<T> {
        MinBinaryHeap { tree: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use MinBinaryHeap;

    #[test]
    fn parent_index() {
        use std::panic::catch_unwind;

        let queue = &setup();

        assert!(catch_unwind(move || queue.parent_index(0)).is_err());
        assert!(catch_unwind(move || queue.parent_index(queue.size())).is_err());
        assert_eq!(queue.parent_index(9), 4);
        assert_eq!(queue.parent_index(8), 3);
        assert_eq!(queue.parent_index(7), 3);
        assert_eq!(queue.parent_index(6), 2);
        assert_eq!(queue.parent_index(5), 2);
        assert_eq!(queue.parent_index(4), 1);
        assert_eq!(queue.parent_index(3), 1);
        assert_eq!(queue.parent_index(2), 0);
        assert_eq!(queue.parent_index(1), 0);
    }

    #[test]
    fn child_index_min() {
        let queue = setup();

        assert_eq!(queue.child_index_min(9), None);
        assert_eq!(queue.child_index_min(8), None);
        assert_eq!(queue.child_index_min(7), None);
        assert_eq!(queue.child_index_min(6), None);
        assert_eq!(queue.child_index_min(5), None);
        assert_eq!(queue.child_index_min(4), Some(9));
        assert_eq!(queue.child_index_min(3), Some(8));
        assert_eq!(queue.child_index_min(2), Some(6));
        assert_eq!(queue.child_index_min(1), Some(4));
        assert_eq!(queue.child_index_min(0), Some(1));
    }

    fn setup() -> MinBinaryHeap<u8> {
        let mut queue = MinBinaryHeap::new();
        queue.insert(16);
        queue.insert(14);
        queue.insert(10);
        queue.insert(8);
        queue.insert(7);
        queue.insert(9);
        queue.insert(3);
        queue.insert(2);
        queue.insert(4);
        queue.insert(1);

        queue
    }
}
