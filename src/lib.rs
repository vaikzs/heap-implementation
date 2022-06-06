/**
 * Target: Implement a heap datastructure to perform the following operations:
 * 1. heapify - rearranges the elements in the heap to maintain the heap property
 * 2. insert -  adds an item to a heap while maintaining its heap property
 * 5. isEmpty - boolean, returns true if boolean is empty and false if it has a node
 * 6. size - returns the size of the heap
 * 7. min - returns the min value in a heap
 * 8. print
 *
 *
    1) Heaps = Arrays. (Start with Index 1)
    2) Parent of Current Index = Current Index / 2
    Where to Put new data?
    3) Current Index Left Node = Current Index * 2
    4) Current Index Right Node = (Current Index * 2) + 1
*/

pub struct Heap {
    pub items: Vec<i32>, // We consider this vector to represent the heap
    pub size: usize,     // maintain the size of the heap
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            size: 0,
            items: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /*
        Helper methods
    */
    pub fn get_left_child_index(&self, parent_index: usize) -> usize {
        2 * parent_index + 1
    }

    pub fn get_right_child_index(&self, parent_index: usize) -> usize {
        2 * parent_index + 2
    }

    pub fn get_parent_index(&self, child_index: usize) -> usize {
        (child_index - 1) / 2
    }

    pub fn has_left_child(&self, index: usize) -> bool {
        self.get_left_child_index(index) < self.size
    }

    pub fn has_right_child(&self, index: usize) -> bool {
        self.get_right_child_index(index) < self.size
    }

    pub fn has_parent(&self, index: usize) -> bool {
        self.get_parent_index(index) >= 0
    }

    pub fn left_child(&self, index: usize) -> i32 {
        self.items[self.get_left_child_index(index)]
    }

    pub fn right_child(&self, index: usize) -> i32 {
        self.items[self.get_right_child_index(index)]
    }

    pub fn parent(&self, index: usize) -> i32 {
        self.items[self.get_parent_index(index)]
    }

    pub fn insert(&mut self, val: i32) {
        self.items.insert(self.size, val);
        self.size += 1;
        self.heapify_up();
    }

    fn swap(&mut self, index_one: usize, index_two: usize) {
        let temp = self.items[index_one];
        self.items[index_one] = self.items[index_two];
        self.items[index_two] = temp;
    }

    // Always the first element is expected to be root, return that BUT don't extract
    pub fn peek(&self) -> usize {
        if self.size == 0 {
            panic!("Failed heap is empty")
        }
        self.items[0] as usize
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    // Retrieve the minimum and remove from the array
    // also called 'poll'
    pub fn extract_min(&mut self) -> i32 {
        if self.size == 0 {
            panic!("Failed heap is empty")
        }
        let item = self.items[0];
        self.items[0] = self.items[self.size - 1];
        self.size -= 1;
        self.heapify_down();
        item
    }

    // shift elements up as needed starting from last position where you have inserted
    pub fn heapify_up(&mut self) {
        let mut index = self.size - 1;

        while index > 0 && self.has_parent(index) && self.parent(index) > self.items[index] {
            self.swap(self.get_parent_index(index), index);
            index = self.get_parent_index(index);
        }
    }

    // shift elements down as needed starting from root
    pub fn heapify_down(&mut self) {
        let mut idx = 0;

        // only need to check left child, because if there is no left child, there is definitely no right child
        while self.has_left_child(idx) {
            // get the index of small value (left)
            let mut smaller_child_idx = self.get_left_child_index(idx);

            // if right child is smaller than left child, then we consider that as smaller child
            if self.has_right_child(idx) && self.right_child(idx) < self.left_child(idx) {
                smaller_child_idx = self.get_right_child_index(idx)
            }

            // our current index is correct so we break
            if self.items[idx] < self.items[smaller_child_idx] {
                break;
            }
            // else our current index is not correct, we need to swap with the smaller index that we estimated
            else {
                self.swap(idx, smaller_child_idx);
            }
            // once complete we continue our swaping with the smaller child by updating the current index
            idx = smaller_child_idx; // move down to my smaller child
        }
    }

    pub fn delete_root(&mut self) -> bool {
        if self.extract_min().is_positive() {
            true
        } else {
            false
        }
    }
    pub fn print(&self) {
        println!("{:?}", self.items);
    }
}
