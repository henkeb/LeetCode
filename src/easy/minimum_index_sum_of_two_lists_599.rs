// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two arrays of strings list1 and list2, find the common strings with the least index sum.
//
// A common string is a string that appeared in both list1 and list2.
//
// A common string with the least index sum is a common string such that if it appeared at list1[i] and list2[j] then i + j should be the minimum value among all the other common strings.
//
// Return all the common strings with the least index sum. Return the answer in any order.
struct Solution;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut list1_map: HashMap<&str, usize> = HashMap::new();
        for (i, word) in list1.iter().enumerate() {
            list1_map.insert(word, i);
        }

        let it = list2
            .iter()
            .enumerate()
            .filter_map(|(i, word)| list1_map.get(word.as_str()).map(|j| (word, i + j)));
        let min = it.clone().map(|(_, sum)| sum).min();
        it.filter(|(_, sum)| *sum == min.unwrap())
            .map(|(word, _)| word.to_string())
            .collect()
    }
}
