use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Solution has
// Time complexity: O(n)
// Space complexity: O(k)
//
// Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.
//
// Implement KthLargest class:
//
//     KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
//     int add(int val) Appends the integer val to the stream and returns the element representing the kth largest element in the stream.
struct KthLargest {
    min_heap: BinaryHeap<std::cmp::Reverse<i32>>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::with_capacity((k + 1) as usize);
        for &num in nums.iter() {
            if min_heap.len() < k as usize {
                min_heap.push(Reverse(num));
            } else if let Some(Reverse(v)) = min_heap.peek() {
                if *v < num {
                    min_heap.pop();
                    min_heap.push(Reverse(num));
                }
            }
        }
        KthLargest {
            min_heap,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.k {
            self.min_heap.push(Reverse(val));
        } else if let Some(Reverse(v)) = self.min_heap.peek() {
            if *v < val {
                self.min_heap.pop();
                self.min_heap.push(Reverse(val));
            }
        }
        if let Some(Reverse(v)) = self.min_heap.peek() {
            *v
        } else {
            0
        }
    }
}

//  Your KthLargest object will be instantiated and called as such:
//  let obj = KthLargest::new(k, nums);
//  let ret_1: i32 = obj.add(val);
//
